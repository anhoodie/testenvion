//! Types definition used for handling returned data when generic API is used.

use pair::Pair;

type Price = f64;
type Volume = f64;

#[derive(Debug)]
pub struct Ticker {
    /// UNIX timestamp in ms (when the response was recei