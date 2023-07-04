pub trait Reducer<S, A> {
    fn reduce(&self, state: S, action: A) -> S;
}

impl<F, S, A> Reducer<S, A> for F
where
    F: Fn(S, A) -> S,
{
    fn reduce(&self, state: S, action: A) -> S {
        self(state, action)
    }
}
