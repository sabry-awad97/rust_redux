use crate::{store_api::StoreApi, Store};

pub trait Middleware<State, Action>
where
    State: Send + Sync + Clone + 'static,
    Action: Send + 'static,
{
    fn dispatch<StoreImpl, NextFn>(&self, store: &StoreImpl, action: Action, next: NextFn)
    where
        StoreImpl: StoreApi<State, Action>,
        NextFn: Fn(Action);
}

pub struct StoreMiddleware<State, Action, M>
where
    State: Send + Sync + Clone + 'static,
    Action: Send + 'static,
    M: Middleware<State, Action> + Send + Sync + 'static,
{
    pub inner: Store<State, Action>,
    pub middleware: M,
}

impl<State, Action, M> StoreMiddleware<State, Action, M>
where
    State: Send + Sync + Clone + 'static,
    Action: Send + 'static,
    M: Middleware<State, Action> + Send + Sync + 'static,
{
    pub fn dispatch(&self, action: Action) {
        self.middleware.dispatch(&self.inner, action, |action| {
            self.inner.dispatch(action);
        });
    }

    pub fn get_state(&self) -> State {
        self.inner.get_state()
    }

    pub fn select<F, Result>(&self, selector: F) -> Result
    where
        F: Fn(&State) -> Result,
    {
        self.inner.select(selector)
    }

    pub fn subscribe<F>(&self, callback: F) -> impl Fn() + '_
    where
        F: Fn(&State) + Send + Sync + 'static,
    {
        self.inner.subscribe(callback)
    }
}
