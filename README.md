![Coinnect](https://raw.githubusercontent.com/hugues31/coinnect/master/coinnect.png)
===========
[![crates.io](https://img.shields.io/crates/v/coinnect.svg)](https://crates.io/crates/coinnect)
[![doc.rs](https://docs.rs/coinnect/badge.svg)](https://docs.rs/coinnect/)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)


Coinnect is a Rust library aiming to provide a complete access to REST APIs for
various crypto-currencies exchanges (see below for a list of supported
exchanges).
All methods consume HTTPS api. The purpose of this crate is not to stream data
(you should use websocket/FIX in that case).


**WARNING:**  This library is highly experimental at the momen