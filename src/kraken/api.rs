//! Use this module to interact with the raw-original API provided by Kraken.
//! WARNING: Special attention should be paid to error management: parsing number, etc.

use crypto::digest::Digest;
use crypto::hmac::Hmac;
use crypto::mac::Mac;
use crypto::sha2::{Sha256, Sha512};

use hyper_native_tls::NativeTlsClient;
use hyper::Client;
use hyper::header;
use hyper::net::HttpsConnector;

use rustc_serialize::base64::{STANDARD, ToBase64, FromBase64};

use serde_json;
use serde_json::Value;
use serde_json::value::Map;

use std::collections::HashMap;
use std::io::Read;
use std::thread;
use std::time::Duration;
use std::path::PathBuf;
use std::fs::File;
use std::str;
use std::iter::repeat;

use error;
use helpers;

use kraken::utils;

header! {
    #[doc(hidden)]
    (KeyHeader, "API-Key") => [String]
}

header! {
    #[doc(hidden)]
    (SignHeader, "API-Sign") => [String]
}

#[derive(Debug)]
pub struct KrakenApi {
    last_request: i64, // unix timestamp in ms, to avoid ban
    api_key: String,
    api_secret: String,
    http_client: Client,
}


impl KrakenApi {
    /// Create a new KrakenApi by providing an API key & API secret
    pub fn new(api_key: &str, api_secret: &str) -> KrakenApi {
        let ssl = NativeTlsClient::new().unwrap();
        let connector = HttpsConnector::new(ssl);

        KrakenApi {
            last_request: 0,
            api_key: api_key.to_string(),
            api_secret: api_secret.to_string(),
            http_client: Client::with_connector(connector),
        }
    }

    /// Create a new KrakenApi from a json configuration file. This file must follow this structure:
    ///
    /// ```ignore
    /// {
    ///     "account_kraken": {
    ///         "exchange"  : "kraken",
    ///         "api_key"   : "123456789ABCDEF",
    ///         "api_secret": "ABC&EF?abcdef"
    ///     },
    ///     "account_poloniex": {
    ///         "exc