use redux::Store;

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

fn main() {
    let store = Store::new(reducer);
    store.dispatch(Action::Increment);
    store.dispatch(Action::Increment);
    store.dispatch(Action::Increment);
    store.dispatch(Action::Decrement);
}
