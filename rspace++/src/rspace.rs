/*
    - Create object called RSpace with type "RSpaceStore" and method "create".
    - See RSpace.scala in rspace/ and AddressBookExample.scala in rspace/examples

    - Type "RSpaceStore" is defined in scala like:

    /**
    * Maps (key-value stores) used to create [[RSpace]] or [[ReplayRSpace]].
    */
  final case class RSpaceStore[F[_]](
      history: KeyValueStore[F],
      roots: KeyValueStore[F],
      cold: KeyValueStore[F]
  )

    - Method "create" takes in 6 parameters, implicit parameter called "Match" and one argument, store, of type RSpaceStore.
      - Parameter 1 (F) is a type constructor representing the effect type used in the computation?
        Type "cats.{Id}" in example.
      - Parameter 2 - 5 represent the types of the data stored in the RSpace.
      - The implicit parameter Match provides a match implementation for matching against the patterns.
      - Argument store of type RSpaceStore[F], which represents the underlying store used to persist the data.

*/

use crate::key_value_store::KeyValueStore;
use crate::r#match::Match;
use crate::serialize::Serialize;
use std::error::Error;

struct RSpaceStore<F: KeyValueStore<F>> {
    history: F,
    roots: F,
    cold: F,
}

pub trait RSpace {
    fn create<F, C, P, A, K>(
        &self,
        store: RSpaceStore<F>,
        sc: impl Serialize<C>,
        sp: impl Serialize<P>,
        sa: impl Serialize<A>,
        sk: impl Serialize<K>,
        m: dyn Match<F, P, A>,
        // scheduler: ExecutionContext,
    ) -> Result<dyn RSpace, dyn Error>;
}
