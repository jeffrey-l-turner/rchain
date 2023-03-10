/**
  * Type class for matching patterns with data.
  *
  * @tparam P A type representing patterns
  * @tparam A A type representing data and match result
  */
trait Match<F, P, A> {
    fn get(&self, p: P, a: A) -> F<Option<A>>;
}