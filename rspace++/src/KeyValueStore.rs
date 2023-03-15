use std::error::Error;

pub trait KeyValueStore<F> {
    /*
      Retrieves the values associated with the given keys from the store.
      It takes a vector of byte vectors keys, which represent the keys to be retrieved,
      and a function from_buffer that converts a byte vector to a value of type T or returns an error of type E.
      The function returns a vector of Option<T>, which contains either the retrieved value
      or None if the corresponding key is not found in the store.
      The function returns a Result that can contain an error of type Box<dyn Error>, which is a trait object representing any type of error.
    */
    fn get<T, E>(
        &self,
        keys: Vec<Vec<u8>>,
        from_buffer: fn(Vec<u8>) -> Result<T, E>,
    ) -> Result<Vec<Option<T>>, Box<dyn Error>>;

    /*
      Adds or updates the given key-value pairs in the store.
      It takes a vector of tuples (Vec<u8>, T), which represent the key-value pairs to be added or updated,
      and a function to_buffer that converts a value of type T to a byte vector or returns an error of type E.
      The function returns a Result that can contain an error of type Box<dyn Error>.
    */
    fn put<T, E>(
        &mut self,
        kv_pairs: Vec<(Vec<u8>, T)>,
        to_buffer: fn(T) -> Result<Vec<u8>, E>,
    ) -> Result<(), Box<dyn Error>>;

    /*
      Deletes the values associated with the given keys from the store.
      It takes a vector of byte vectors keys, which represent the keys to be deleted.
      The function returns the number of values that were deleted as an i32, and a Result that can contain an error of type Box<dyn Error>.
    */
    fn delete(&mut self, keys: Vec<Vec<u8>>) -> Result<i32, Box<dyn Error>>;

    /*
     Allows iterating over all key-value pairs in the store.
     It takes a function f that takes a mutable reference to a trait object implementing Iterator
     and returns a value of type R. The function returns a Result that can contain an error of type Box<dyn Error>.
    */
    fn iterate<T, R>(
        &self,
        f: fn(&mut dyn Iterator<Item = (&Vec<u8>, &Vec<u8>)>) -> R,
    ) -> Result<R, Box<dyn Error>>;
}
