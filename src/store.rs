use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    thread,
};

use crossbeam_channel::{unbounded, Sender};
use parking_lot::RwLock;

use crate::reducer::Reducer;

type Subscriber<T> = Box<dyn Fn(&T) + Send + Sync>;
type Subscribers<T> = HashMap<usize, Subscriber<T>>;
pub struct Store<State, Action> {
    state: Arc<RwLock<State>>,
    dispatcher: Sender<Action>,
    subscribers: Arc<RwLock<Subscribers<State>>>,
    next_subscriber_id: Arc<AtomicUsize>,
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
        let (dispatcher, receiver) = unbounded();

        let cloned_state = state.clone();
        let cloned_subscribers = subscribers.clone();

        thread::spawn(move || {
            while let Ok(action) = receiver.recv() {
                let state = &mut cloned_state.write();
                let new_state = root_reducer.reduce(&**state, action);
                **state = new_state;

                let subscriber_map = cloned_subscribers.read();
                for subscriber in subscriber_map.values() {
                    subscriber(&*state);
                }
            }
        });

        Self {
            state,
            dispatcher,
            subscribers,
            next_subscriber_id,
        }
    }

    pub fn dispatch(&self, action: Action) {
        if self.dispatcher.send(action).is_err() {
            // Handle send error gracefully
        }
    }

    pub fn get_state(&self) -> State {
        self.state.read().clone()
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
}
