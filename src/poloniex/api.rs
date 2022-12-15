
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
use serde_json::Value;
use serde_json::value::Map;

use std::collections::HashMap;
use std::io::Read;
use std::thread;
use std::time::Duration;
use std::path::PathBuf;
use std::fs::File;

use error;
use helpers;

use poloniex::utils;

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
pub struct PoloniexApi {
    last_request: i64, // unix timestamp in ms, to avoid ban
    api_key: String,
    api_secret: String,
    http_client: Client,
}


impl PoloniexApi {
    /// Create a new PoloniexApi by providing an API key & API secret
    pub fn new(api_key: &str, api_secret: &str) -> PoloniexApi {
        let ssl = NativeTlsClient::new().unwrap();
        let connector = HttpsConnector::new(ssl);

        PoloniexApi {
            last_request: 0,
            api_key: api_key.to_string(),
            api_secret: api_secret.to_string(),
            http_client: Client::with_connector(connector),
        }
    }

    /// Create a new PoloniexApi from a json configuration file. This file must follow this
    /// structure:
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
    /// For this example, you could use load your Poloniex account with
    /// `new_from_file("account_poloniex", Path::new("/keys.json"))`
    pub fn new_from_file(config_name: &str, path: PathBuf) -> PoloniexApi {
        let mut f = File::open(&path).unwrap();
        let mut buffer = String::new();
        f.read_to_string(&mut buffer).unwrap();

        let data: Value = serde_json::from_str(&buffer).unwrap();
        let json_obj = data.as_object().unwrap().get(config_name).unwrap();
        let api_key = json_obj.get("api_key").unwrap().as_str().unwrap();
        let api_secret = json_obj.get("api_secret").unwrap().as_str().unwrap();

        PoloniexApi::new(api_key, api_secret)
    }

    fn block_or_continue(&self) {
        let threshold = 167; // 6 requests/sec = 1/6*1000
        let delay = helpers::get_unix_timestamp_ms() - self.last_request;
        if delay < threshold {
            let duration_ms = Duration::from_millis(delay as u64);
            thread::sleep(duration_ms);
        }
    }

    fn public_query(&mut self,
                    method: &str,
                    params: &HashMap<&str, &str>)
                    -> Result<Map<String, Value>, error::Error> {
        let mut params = params.clone();
        helpers::strip_empties(&mut params);
        let url = "https://poloniex.com/public?command=".to_string() + method + "&" +
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
                     params: &HashMap<&str, &str>)
                     -> Result<Map<String, Value>, error::Error> {
        let unix_timestamp = helpers::get_unix_timestamp_ms().to_string();
        let mut post_params = params.clone();
        post_params.insert("command", method);
        post_params.insert("nonce", &unix_timestamp);
        helpers::strip_empties(&mut post_params);
        let post_data = helpers::url_encode_hashmap(&post_params);

        let mut hmac = Hmac::new(Sha512::new(), self.api_secret.as_bytes());
        hmac.input(post_data.as_bytes());

        let sign = hmac.result().code().to_hex();

        let mut custom_header = header::Headers::new();
        custom_header.set(KeyHeader(self.api_key.to_owned()));
        custom_header.set(SignHeader(sign));
        custom_header.set(ContentHeader("application/x-www-form-urlencoded".to_owned()));

        self.block_or_continue();

        let mut response = match self.http_client
            .post("https://poloniex.com/tradingApi")
            .body(&post_data)
            .headers(custom_header)
            .send() {
            Ok(response) => response,
            Err(_) => return Err(error::Error::ServiceUnavailable),
        };
        self.last_request = helpers::get_unix_timestamp_ms();

        let mut buffer = String::new();
        response.read_to_string(&mut buffer).unwrap();
        return utils::deserialize_json(buffer);
    }

    /// Sample output :
    ///
    /// ```ignore
    /// {
    /// "BTC_LTC":{
    /// "last":"0.0251","lowestAsk":"0.02589999","highestBid":"0.0251",
    /// "percentChange":"0.02390438","baseVolume":"6.16485315","quoteVolume":"245.82513926"},
    /// "BTC_NXT":{
    /// "last":"0.00005730","lowestAsk":"0.00005710","highestBid":"0.00004903",
    /// "percentChange":"0.16701570","baseVolume":"0.45347489","quoteVolume":"9094"},
    /// ... }
    /// ```
    pub fn return_ticker(&mut self) -> Result<Map<String, Value>, error::Error> {
        let params = HashMap::new();
        self.public_query("returnTicker", &params)
    }

    /// Sample output :
    ///
    /// ```ignore
    /// {"BTC_LTC":{"BTC":"2.23248854","LTC":"87.10381314"},"BTC_NXT":{"BTC":"0.981616",
    /// "NXT":"14145"},
    /// ... "totalBTC":"81.89657704","totalLTC":"78.52083806"}
    /// ```
    pub fn return_24_volume(&mut self) -> Result<Map<String, Value>, error::Error> {
        let params = HashMap::new();
        self.public_query("return24Volume", &params)
    }

    /// Sample output :
    ///
    /// ```ignore
    /// {"asks":[[0.00007600,1164],[0.00007620,1300], ... ], "bids":[[0.00006901,200],
    /// [0.00006900,408], ... ], "isFrozen": 0, "seq": 18849}
    /// ```
    pub fn return_order_book(&mut self,
                             currency_pair: &str,
                             depth: &str)
                             -> Result<Map<String, Value>, error::Error> {
        let mut params = HashMap::new();
        params.insert("currencyPair", currency_pair);
        params.insert("depth", depth);
        self.public_query("returnOrderBook", &params)
    }

    /// Sample output :
    ///
    /// ```ignore
    /// [{"date":"2014-02-10 04:23:23","type":"buy","rate":"0.00007600","amount":"140",
    /// "total":"0.01064"},
    /// {"date":"2014-02-10 01:19:37","type":"buy","rate":"0.00007600","amount":"655",
    /// "total":"0.04978"}, ... ]
    /// ```
    pub fn return_trade_history(&mut self,
                                currency_pair: &str,
                                start: &str,
                                end: &str)
                                -> Result<Map<String, Value>, error::Error> {
        let mut params = HashMap::new();
        params.insert("currencyPair", currency_pair);
        params.insert("start", start);
        params.insert("end", end);
        self.public_query("returnTradeHistory", &params)
    }

    /// Sample output :
    ///
    /// ```ignore
    /// [{"date":1405699200,"high":0.0045388,"low":0.00403001,"open":0.00404545,"close":0.00427592,
    /// "volume":44.11655644,"quoteVolume":10259.29079097,"weightedAverage":0.00430015}, ...]
    /// ```
    pub fn return_chart_data(&mut self,
                             currency_pair: &str,
                             start: &str,
                             end: &str,
                             period: &str)
                             -> Result<Map<String, Value>, error::Error> {
        let mut params = HashMap::new();
        params.insert("currencyPair", currency_pair);
        params.insert("start", start);
        params.insert("end", end);
        params.insert("period", period);
        self.public_query("returnChartData", &params)
    }

    /// Sample output :
    ///
    /// ```ignore
    /// {"1CR":{"maxDailyWithdrawal":10000,"txFee":0.01,"minConf":3,"disabled":0},
    /// "ABY":{"maxDailyWithdrawal":10000000,"txFee":0.01,"minConf":8,"disabled":0}, ... }
    /// ```
    pub fn return_currencies(&mut self) -> Result<Map<String, Value>, error::Error> {
        let params = HashMap::new();
        self.public_query("returnCurrencies", &params)
    }

    /// Sample output :
    ///
    /// ```ignore
    /// {"offers":[{"rate":"0.00200000","amount":"64.66305732","rangeMin":2,"rangeMax":8}, ... ],
    /// "demands":[{"rate":"0.00170000","amount":"26.54848841","rangeMin":2,"rangeMax":2}, ... ]}
    /// ```
    pub fn return_loan_orders(&mut self,
                              currency: &str)
                              -> Result<Map<String, Value>, error::Error> {
        let mut params = HashMap::new();
        params.insert("currency", currency);
        self.public_query("returnLoanOrders", &params)
    }

    /// Returns all of your available balances.
    ///
    /// Sample output:
    ///
    /// ```ignore
    /// {"BTC":"0.59098578","LTC":"3.31117268", ... }
    /// ```
    pub fn return_balances(&mut self) -> Result<Map<String, Value>, error::Error> {
        let params = HashMap::new();
        self.private_query("returnBalances", &params)
    }

    /// Returns all of your balances, including available balance, balance on orders,
    /// and the estimated BTC value of your balance. By default, this call is limited to your
    /// exchange account; set the "account" POST parameter to "all" to include your margin and
    /// lending accounts.
    ///
    /// Sample output:
    ///
    /// ```ignore
    /// {"LTC":{"available":"5.015","onOrders":"1.0025","btcValue":"0.078"},"NXT":{...}, ... }
    /// ```
    pub fn return_complete_balances(&mut self) -> Result<Map<String, Value>, error::Error> {
        let mut params = HashMap::new();
        params.insert("account", "all");
        self.private_query("returnCompleteBalances", &params)
    }

    /// Returns all of your deposit addresses.
    ///
    /// Sample output:
    ///
    /// ```ignore
    /// {"BTC":"19YqztHmspv2egyD6jQM3yn81x5t5krVdJ","LTC":"LPgf9kjv9H1Vuh4XSaKhzBe8JHdou1WgUB",
    /// ... "ITC":"Press Generate.." ... }
    /// ```
    pub fn return_deposit_addresses(&mut self) -> Result<Map<String, Value>, error::Error> {
        let params = HashMap::new();
        self.private_query("returnDepositAddresses", &params)
    }

    /// Generates a new deposit address for the currency specified by the "currency" POST parameter.
    ///
    /// Sample output:
    ///
    /// ```ignore
    /// {"success":1,"response":"CKXbbs8FAVbtEa397gJHSutmrdrBrhUMxe"}
    /// ```
    pub fn generate_new_address(&mut self,
                                currency: &str)
                                -> Result<Map<String, Value>, error::Error> {
        let mut params = HashMap::new();
        params.insert("currency", currency);
        self.private_query("generateNewAddress", &params)
    }

    /// Returns your deposit and withdrawal history within a range, specified by the "start" and
    /// "end" POST parameters,
    /// both of which should be given as UNIX timestamps.
    ///
    /// Sample output:
    ///
    /// ```ignore
    /// {"deposits":
    /// [{"currency":"BTC","address":"...","amount":"0.01006132","confirmations":10,
    /// "txid":"17f819a91369a9ff6c4a34216d434597cfc1b4a3d0489b46bd6f924137a47701",
    /// "timestamp":1399305798,"status":"COMPLETE"},
    /// {"currency":"BTC","address":"...","amount":"0.00404104","confirmations":10,
    /// "txid":"7acb90965b252e55a894b535ef0b0b65f45821f2899e4a379d3e43799604695c",
    /// "timestamp":1399245916,"status":"COMPLETE"}],
    /// "withdrawals":[{"withdrawalNumber":134933,"currency":"BTC",
    /// "address":"1N2i5n8DwTGzUq2Vmn9TUL8J1vdr1XBDFg","amount":"5.00010000",
    /// "timestamp":1399267904,
    /// "status":"COMPLETE: 36e483efa6aff9fd53a235177579d98451c4eb237c210e66cd2b9a2d4a988f8e",
    /// "ipAddress":"..."}]}
    /// ```
    pub fn return_deposits_withdrawals(&mut self,
                                       start: &str,
                                       end: &str)
                                       -> Result<Map<String, Value>, error::Error> {
        let mut params = HashMap::new();
        params.insert("start", start);
        params.insert("end", end);
        self.private_query("returnDepositsWithdrawals", &params)
    }

    ///Returns your open orders for a given market, specified by the "currencyPair" POST parameter,
    /// e.g. "BTC_XCP". Set "currencyPair" to "all" to return open orders for all markets.
    ///
    /// Sample output for single market:
    ///
    /// ```ignore
    /// [{"orderNumber":"120466","type":"sell","rate":"0.025","amount":"100","total":"2.5"},