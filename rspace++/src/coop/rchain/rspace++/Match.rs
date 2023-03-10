/**
 * Type class for matching patterns with data.
 *
 * @tparam P A type representing patterns
 * @tparam A A type representing data and match result
 */
#[repr(C)]
pub struct OptionOfA<A> {
    pub is_some: bool,
    pub value: A,
}

pub trait Match<F, P, A> {
    fn get(&self, p: P, a: A) -> F;
}

fn main() {}
