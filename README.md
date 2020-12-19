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
|:--------:|:-----------------:|:---------------------:|:----:|
| Bitstamp | X | X | Not every method are implemented for now. Generic API supports only Ticker for now. |
| Kraken   | X | X | Generic API supports only Ticker for now. |
| Poloniex | X | X | Generic API supports only Ticker for now. |

Feel free to make a PR to add support to your favorite exchange ;)

### Documentation

- [Master](https://docs.rs/coinnect/)


## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
coinnect = "0.2"
```

and this to your crate root:

```rust
extern crate coinnect;
```

For optional parameters, most methods require an empty str (`""`) if you don't
want to specify them.

Since 0.2, you have access to a generic API to communicate across exchanges in
the same way. Note that this functionality is under active development, changes
constantly and not every Exchange is supported for now.
For more info, look at ExchangeApi trait doc.

## Example

The example below shows you how to connect to Poloniex

```rust
extern crate coinnect;

use coinnect::poloniex::PoloniexApi;

fn main() {
    // We create a PoloniexApi by providing API key/secret
    // You can give an empty str if you only use public methods
    let mut my_api = PoloniexApi::new("api_key", "api_secret");

    // Let's look at the ticker!
    let list_coi