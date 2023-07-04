# Redux

## Introduction

The Redux library provides a central store for managing application state and facilitates the dispatching of actions to modify the state. The store supports middleware, allowing you to intercept dispatched actions and perform additional logic. The store also supports subscribing to state changes and selecting specific data from the state.

## Installation

To use the Redux in your Rust project, add the following to your Cargo.toml file:

```toml
[dependencies]
redux = [ git = "https://github.com/sabry-awad97/rust_redux" ]
```

## Usage

To demonstrate the usage of the `Store` struct, we will provide examples of defining the state, actions, and reducer, creating a store with middleware, dispatching actions, and accessing the state.

### Defining the State

First, you need to define the state of your application. The state is a data structure that represents the current state of your application. In this example, we define an `AppState` struct with a single field `counter`:

```rs
#[derive(Debug, Clone, Default)]
struct AppState {
    counter: i8,
}
```

### Defining Actions

Actions represent the different operations or events that can occur in your application. In this example, we define an `Action` enum with two variants: `Increment` and `Decrement`:

```rs
#[derive(Debug)]
enum Action {
    Increment,
    Decrement,
}
```

## Defining the Reducer

The reducer is a function that takes the current state and an action as arguments, and returns a new state based on the action. In this example, we define a `reducer` function that increments or decrements the `counter` field of the state based on the action:

```rs
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
```

## Creating a Store with Middleware

To create a store with middleware, you can use the `Store::new` method and provide the reducer function. In this example, we create a store with the `LoggingMiddleware`:

```rs
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
```

### Dispatching Actions

Once you have created the store, you can dispatch actions using the `dispatch` method. In this example, we dispatch multiple actions to increment and decrement the counter:

```rs
store.dispatch(Action::Increment);
store.dispatch(Action::Decrement);
```

### Accessing the State

To access the current state from the Store, you can use the `get_state` method. It returns a clone of the current state. Here's an example:

```rs
let state = store.get_state();
```

### Selecting Data from the State

The `Store` allows selecting specific data from the state using selectors. Selectors are functions that take the current state as an argument and return a specific piece of data from the state. Here's an example of selecting data from the state:

```rs
let counter = store.select(|state: &AppState| state.counter);
```

## Contributing

Contributions to the Store library are welcome! If you find a bug or want to suggest an improvement, please create an issue or submit a pull request on the GitHub repository.
