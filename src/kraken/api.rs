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
    ///         "exchange"  : "poloniex",
    ///         "api_key"   : "XYXY-XYXY-XYXY-XY",
    ///         "api_secret": "A0A0B1B1C2C2"
    ///     }
    /// }
    /// ```
    /// For this example, you could use load your Kraken account with
    /// `new_from_file("account_kraken", Path::new("/keys.json"))`
    pub fn new_from_file(config_name: &str, path: PathBuf) -> KrakenApi {
        let mut f = File::open(&path).unwrap();
        let mut buffer = String::new();
        f.read_to_string(&mut buffer).unwrap();

        let data: Value = serde_json::from_str(&buffer).unwrap();
        let json_obj = data.as_object().unwrap().get(config_name).unwrap();
        let api_key = json_obj.get("api_key").unwrap().as_str().unwrap();
        let api_secret = json_obj.get("api_secret").unwrap().as_str().unwrap();

        KrakenApi::new(api_key, api_secret)
    }

    fn block_or_continue(&self) {
        let threshold = 2000; // 1 request/2sec
        let delay = helpers::get_unix_timestamp_ms() - self.last_request;
        if delay < threshold {
            let duration_ms = Duration::from_millis(delay as u64);
            thread::sleep(duration_ms);
        }
    }

    fn public_query(&mut self,
                    method: &str,
                    params: &mut HashMap<&str, &str>)
                    -> Result<Map<String, Value>, error::Error> {
        helpers::strip_empties(params);
        let url = "https://api.kraken.com/0/public/".to_string() + method + "?" +
                  &helpers::url_encode_hashmap(&params);

        self.block_or_continue();
        let mut response = match self.http_client.get(&url).send() {
            Ok(response) => response,
            Err(_) => return Err(error::Error::ServiceUnavailable),
        };
        self.last_request = helpers::get_unix_timestamp_ms();
        let mut buffer = String::new();
        response.read_to_string(&mut buffer).unwrap();
        return utils::deserialize_json(buffer);
    }

    fn private_query(&mut self,
                     method: &str,
                     mut params: &mut HashMap<&str, &str>)
                     -> Result<Map<String, Value>, error::Error> {
        let url = "https://api.kraken.com/0/private/".to_string() + method;

        let urlpath = "/0/private/".to_string() + method;

        let nonce