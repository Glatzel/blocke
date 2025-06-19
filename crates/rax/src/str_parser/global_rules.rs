
pub trait IStrGlobalRules<'a, O> {
    fn name(&self) -> &str;
    fn apply(&self, input: &'a str) -> O;
}
