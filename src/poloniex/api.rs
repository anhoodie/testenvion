
//! Use this module to interact with Poloniex exchange.
//! See examples for more informations.

use crypto::hmac::Hmac;
use crypto::mac::Mac;
use crypto::sha2::Sha512;

use hyper_native_tls::NativeTlsClient;
use hyper::Client;
use hyper::header;
use hyper::net::HttpsConnector;

use rustc_serialize::hex::ToHex;

use serde_json;