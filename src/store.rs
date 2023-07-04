use std::{
    marker::PhantomData,
    sync::{Arc, RwLock},
};

use crossbeam_channel::Sender;

pub struct Store<State, Action, RootReducer> {
    state: Arc<RwLock<State>>,
    dispatcher: Sender<Action>,
    _reducer: PhantomData<RootReducer>,
}
