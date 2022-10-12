
//! Use this module to interact with Kraken through a Generic API.
//! This a more convenient and safe way to deal with the exchange since methods return a Result<>
//! but this generic API does not provide all the functionnality that Kraken offers.

use serde_json::Value;
use serde_json::value::Map;

use exchange::ExchangeApi;
use kraken::api::KrakenApi;

use error::Error;
use pair::Pair;
use types::Ticker;
use kraken::utils;
use helpers;

impl ExchangeApi for KrakenApi {