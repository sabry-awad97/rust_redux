pub trait Selector<S> {
    type Output;

    fn select(&self, state: &S) -> Self::Output;
}

impl<F, S, O> Selector<S> for F
where
    F: Fn(&S) -> O,
{
    type Output = O;

    fn select(&self, state: &S) -> Self::Output {
        self(state)
    }
}
