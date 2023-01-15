#[cfg(test)]
mod bitstamp_tests {
    extern crate coinnect;
    use self::coinnect::bitstamp::utils;
    use self::coinnect::bitstamp::api::BitstampApi;

    use self::coinnect::exchange::ExchangeApi;
    use self::coinnect::pair::Pair;

    use std::collections::HashMap;

    #[test]
    fn build_url_should_return_the_a_url() {
        assert_eq!(utils::build_url("ticker", "btcusd"),
                   "https://www.bitstamp.net/api/v2/ticker/btcusd/");
    }
    #[test]
    fn build_url_should_return_the_url_for_transactions_for_btc_usd() {
        assert_eq!(utils::build_url("transactions", "btcusd"),
                   "https://www.bitstamp.net