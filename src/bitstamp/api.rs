
//! Use this module to interact with Bitstamp exchange.
//! Please see examples for more informations.

use hyper_native_tls::NativeTlsClient;
use hyper::Client;
use hyper::header::ContentType;
use hyper::net::HttpsConnector;

use serde_json;
use serde_json::Value;
use serde_json::value::Map;

use std::collections::HashMap;
use std::io::Read;
use std::path::PathBuf;
use std::fs::File;

use error;
use helpers;
use bitstamp::utils;
use pair::Pair;

header! {
    #[doc(hidden)]
    (KeyHeader, "Key") => [String]
}

header! {
    #[doc(hidden)]
    (SignHeader, "Sign") => [String]
}

header! {
    #[doc(hidden)]
    (ContentHeader, "Content-Type") => [String]
}

#[derive(Debug)]
pub struct BitstampApi {
    last_request: i64, // unix timestamp in ms, to avoid ban
    api_key: String,
    api_secret: String,
    customer_id: String,
    http_client: Client,
}


impl BitstampApi {
    /// Create a new BitstampApi by providing an API key & API secret
    pub fn new(params: &HashMap<&str, &str>) -> BitstampApi {
        let mut params = params.clone();
        helpers::strip_empties(&mut params);

        let empty_str: &str = "";

        let api_key = params.get("api_key").unwrap_or(&empty_str);
        let api_secret = params.get("api_secret").unwrap_or(&empty_str);
        let customer_id = params.get("customer_id").unwrap_or(&empty_str);

        let ssl = NativeTlsClient::new().unwrap();
        let connector = HttpsConnector::new(ssl);