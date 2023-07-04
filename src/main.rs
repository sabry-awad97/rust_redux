use std::fmt::Debug;

use redux::{Middleware, Store, StoreApi};

#[derive(Debug, Clone, Default)]
struct AppState {
    counter: i8,
}

#[derive(Debug)]
enum Action {
    Increment,
    Decrement,
}

fn reducer(state: &AppState, action: Action) -> AppState {
    match action {
        Action::Increment => AppState {
            counter: state.counter + 1,
        },
        Action::Decrement => AppState {
            counter: state.counter - 1,
        },
    }
}

struct LoggingMiddleware;

impl<State, Action> Middleware<State, Action> for LoggingMiddleware
where
    State: Send + Sync + Clone + Debug + 'static,
    Action: Send + 'static,
{
    fn dispatch<StoreImpl, NextFn>(&self, store: &StoreImpl, action: Action, next: NextFn)
    where
        StoreImpl: StoreApi<State, Action>,
        NextFn: Fn(Action),
    {
        let state_before = store.get_state();
        println!("Before dispatch: {:?}", state_before);

        next(action);

        let state_after = store.get_state();
        println!("After dispatch: {:?}", state_after);
    }
}

fn main() {
    let store = Store::new(reducer).middleware(LoggingMiddleware);
    store.dispatch(Action::Increment);
    store.dispatch(Action::Increment);
    store.dispatch(Action::Increment);
    store.dispatch(Action::Decrement);

    let counter = store.select(|state: &AppState| state.counter);
    println!("{:?}", counter);
}
