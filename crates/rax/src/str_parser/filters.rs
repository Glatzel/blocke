pub trait IFilter<'a, O> {
    fn name(&self) -> &str;
    fn filter<I>(&self, input: I) -> bool;
}
