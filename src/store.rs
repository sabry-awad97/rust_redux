use std::{sync::Arc, thread};

use crossbeam_channel::{unbounded, Sender};
use parking_lot::RwLock;

use crate::reducer::Reducer;

pub struct Store<State, Action> {
    state: Arc<RwLock<State>>,
    dispatcher: Sender<Action>,
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
        let (dispatcher, receiver) = unbounded();

        let cloned_state = state.clone();
        thread::spawn(move || {
            while let Ok(action) = receiver.recv() {
                let state = &mut cloned_state.write();
                let new_state = root_reducer.reduce(&**state, action);
                **state = new_state;
            }
        });

        Self { state, dispatcher }
    }

    pub fn dispatch(&self, action: Action) {
        if self.dispatcher.send(action).is_err() {
            // Handle send error gracefully
        }
    }

    pub fn get_state(&self) -> State {
        self.state.read().clone()
    }
}
