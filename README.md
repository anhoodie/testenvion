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


**WARNING:**  This library is highly experimental at the moment. Please do not
invest what you can't afford to loose. This is a personal project, I can not be
held responsible for the library malfunction, which can lead to a loss of money.

*The project is licensed under the terms of the MIT License.*

### Exchanges support:
| Exchange | Raw API supported | Generic API supported | Note |
|:--------:|:-----------------:|:---------------------