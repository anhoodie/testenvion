//! Types definition used for handling returned data when generic API is used.

use pair::Pair;

type Price = f64;
type Volume = f64;

#[derive(Debug)]
pub struct Ticker {
    /// UNIX timestamp in ms (when the response was received)
    pub timestamp: i64,
    /// The Pair corresponding to the Ticker returned (maybe useful later for asynchronous APIs)