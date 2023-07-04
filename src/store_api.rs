use crate::selector::Selector;

pub trait StoreApi<State, Action>
where
    State: Send + Sync + Clone + 'static,
    Action: Send + 'static,
{
    fn dispatch(&self, action: Action);
    fn get_state(&self) -> State;
    fn select<F>(&self, selector: F) -> F::Output
    where
        F: Selector<State>;
}
