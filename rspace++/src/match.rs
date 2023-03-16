pub trait Match<F, P, A> {
    fn get(&self, p: P, a: A) -> F;
}
