//! Use this module to create a generic API.



use std::collections::HashMap;
use std::path::PathBuf;
use serde_json::value::Map;
use serde_json::value::Value;

use error::Error;
use types::Ticker;
use exchange::{Exchange, ExchangeApi};
use pair::Pair;
use bitstamp::api::BitstampApi;
use kraken::api::KrakenApi;
use poloniex::api::PoloniexApi;

#[derive(Debug)]
pub struct Coinnect;

impl Coinnect {
    /// Create a new CoinnectApi by providing an API key & API secret
    pub fn new(exchange: Exchange,
               api_key: &str,
               api_secret: &str,
               customer_id: Option<&str>)
               -> Box<ExchangeApi> {
        match exchange {
            Exchange::Bitstamp => {
                let mut params = HashMap::new();
                params.insert("api_key", api_key);
                params.insert("api_secret", api_se