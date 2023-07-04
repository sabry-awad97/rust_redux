use std::sync::{Arc, RwLock};

use crossbeam_channel::{unbounded, Sender};

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
    pub fn new_with_state<R>(initial_state: State, root_reducer: R) -> Self
    where
        R: Reducer<State, Action> + Send + 'static,
    {
        let state = Arc::new(RwLock::new(initial_state));
        let (dispatcher, receiver) = unbounded();

        Self { state, dispatcher }
    }

    pub fn dispatch(&self, action: Action) {
        if self.dispatcher.send(action).is_err() {
            // Handle send error gracefully
        }
    }
}
