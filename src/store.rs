use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    thread,
};

use crossbeam_channel::{unbounded, Sender};
use parking_lot::{Condvar, Mutex, RwLock};

use crate::{
    middleware::{Middleware, StoreMiddleware},
    reducer::Reducer,
    selector::Selector,
    StoreApi,
};

type Subscriber<T> = Box<dyn Fn(&T) + Send + Sync>;
type Subscribers<T> = HashMap<usize, Subscriber<T>>;
pub struct Store<State, Action> {
    state: Arc<RwLock<State>>,
    dispatcher: Sender<Action>,
    subscribers: Arc<RwLock<Subscribers<State>>>,
    next_subscriber_id: Arc<AtomicUsize>,
    condvar: Arc<(Mutex<bool>, Condvar)>,
}

impl<State, Action> Store<State, Action>
where
    State: Send + Sync + Clone + 'static,
    Action: Send + 'static,
{
    pub fn new<R>(root_reducer: R) -> Self
    where
        R: Reducer<State, Action> + Send + 'static,
        State: Default,
    {
        Self::new_with_state(Default::default(), root_reducer)
    }

    pub fn new_with_state<R>(initial_state: State, root_reducer: R) -> Self
    where
        R: Reducer<State, Action> + Send + 'static,
    {
        let state = Arc::new(RwLock::new(initial_state));
        let subscribers: Arc<RwLock<Subscribers<State>>> = Arc::new(RwLock::new(HashMap::new()));
        let next_subscriber_id = Arc::new(AtomicUsize::new(0));
        let condvar = Arc::new((Mutex::new(false), Condvar::new()));

        let (dispatcher, receiver) = unbounded();

        let cloned_state = state.clone();
        let cloned_subscribers = subscribers.clone();
        let cloned_condvar = condvar.clone();

        thread::spawn(move || {
            while let Ok(action) = receiver.recv() {
                let state = &mut cloned_state.write();
                let new_state = root_reducer.reduce(&**state, action);
                **state = new_state;

                let subscriber_map = cloned_subscribers.read();
                for subscriber in subscriber_map.values() {
                    subscriber(&*state);
                }

                let (lock, condvar) = &*cloned_condvar;
                let mut updated = lock.lock();
                *updated = true;
                condvar.notify_one();
            }
        });

        Self {
            state,
            dispatcher,
            subscribers,
            next_subscriber_id,
            condvar,
        }
    }

    pub fn subscribe<F>(&self, callback: F) -> impl Fn() + '_
    where
        F: Fn(&State) + Send + Sync + 'static,
    {
        let subscriber_id = self.next_subscriber_id.fetch_add(1, Ordering::Relaxed);
        let mut subscriber_map = self.subscribers.write();
        subscriber_map.insert(subscriber_id, Box::new(callback));
        move || self.unsubscribe(subscriber_id)
    }

    fn unsubscribe(&self, subscriber_id: usize) {
        let mut subscriber_map = self.subscribers.write();
        subscriber_map.remove(&subscriber_id);
    }

    fn wait_for_update(&self) {
        let (lock, condvar) = &*self.condvar;
        let mut updated = lock.lock();
        while !*updated {
            condvar.wait(&mut updated);
        }
        *updated = false;
    }

    pub fn middleware<M>(self, middleware: M) -> StoreMiddleware<State, Action, M>
    where
        M: Middleware<State, Action> + Send + Sync + 'static,
    {
        StoreMiddleware {
            inner: self,
            middleware,
        }
    }
}

impl<State, Action> StoreApi<State, Action> for Store<State, Action>
where
    State: Send + Sync + Clone + 'static,
    Action: Send + 'static,
{
    fn dispatch(&self, action: Action) {
        if self.dispatcher.send(action).is_err() {
            // Handle send error gracefully
        }

        self.wait_for_update()
    }

    fn get_state(&self) -> State {
        self.state.read().clone()
    }

    fn select<F>(&self, selector: F) -> F::Output
    where
        F: Selector<State>,
    {
        let state = self.state.read();
        selector.select(&*state)
    }
}
