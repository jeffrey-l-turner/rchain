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
