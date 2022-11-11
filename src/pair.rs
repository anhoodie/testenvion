//! This module contains Pair enum.

/// Pair lists all pairs that can be traded on supported exchanges.
/// Update date : 11/04/2017.
/// Note : Kraken uses XBT instead of BTC (so the XBT/EUR pair becomes BTC/EUR).
/// Order of quote currency <-> base currency is important. For example, Kraken supports ZEC_BTC
/// but Poloniex is doing the opposite :  BTC_ZEC, which equal to 1/ZEC_BTC. So: ZEC_BTC != BTC_ZEC
#[derive(Debug, Copy, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum Pair {
    BTC_AMP,
    BTC_ARDR,
 