pub trait Projection<E>: Default {
    fn apply(&mut self, events: Vec<E>);

    fn from(events: Vec<E>) -> Self {
        let mut projection = Self::default();
        projection.apply(events);
        projection
    }
}
