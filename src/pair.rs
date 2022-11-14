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
    BTC_BBR,
    BTC_BCN,
    BTC_BCY,
    BTC_BELA,
    BTC_BITS,
    BTC_BLK,
    BTC_BTCD,
    BTC_BTM,
    BTC_BTS,
    BTC_BURST,
    BTC_C2,
    BTC_CAD_d,
    BTC_CAD,
    BTC_CLAM,
    BTC_CURE,
    BTC_DASH,
    BTC_DCR,
    BTC_DGB,
    BTC_DOGE,
    BTC_EMC2,
    BTC_ETC,
    BTC_ETH,
    BTC_EUR_d,
    BTC_EUR,
    BTC_EXP,
    BTC_FCT,
    BTC_FLDC,
    BTC_FLO,
    BTC_GAME,
    BTC_GBP_d,
    BTC_GBP,
    BTC_GNT,
    BTC_GRC,
    BTC_HUC,
    BTC_HZ,
    BTC_IOC,
    BTC_JPY_d,
    BTC_JPY,
 