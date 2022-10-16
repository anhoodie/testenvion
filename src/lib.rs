//! ![Coinnect](https://raw.githubusercontent.com/hugues31/coinnect/master/coinnect.png)
//!
//! Coinnect is a Rust library aiming to provide a complete access to REST APIs for various
//! crypto-currencies exchanges (see below for a list of supported exchanges).
//! All methods consume HTTPS api. The purpose of this crate is not
//! to stream data (you should use websocket/FIX in that case).
//!
//! For optional parameters, most methods require an empty str (`""`) if you don't want to specify
//! them.
//!
//! ### Exchanges support:
//! - [x] Poloniex
//! - [x] Kraken
//! - [x] Bitstamp (partial)
//!
//! # WARNING
//! This library is highly experimental at the moment. Please do not invest what you
//! can't 