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
    BTC_LBC,
    BTC_LSK,
    BTC_LTC,
    BTC_MAID,
    BTC_MYR,
    BTC_NAUT,
    BTC_NAV,
    BTC_NEOS,
    BTC_NMC,
    BTC_NOBL,
    BTC_NOTE,
    BTC_NSR,
    BTC_NXC,
    BTC_NXT,
    BTC_OMNI,
    BTC_PASC,
    BTC_PINK,
    BTC_POT,
    BTC_PPC,
    BTC_QBK,
    BTC_QORA,
    BTC_QTL,
    BTC_RADS,
    BTC_RBY,
    BTC_REP,
    BTC_RIC,
    BTC_SBD,
    BTC_SC,
    BTC_SDC,
    BTC_SJCX,
    BTC_STEEM,
    BTC_STR,
    BTC_STRAT,
    BTC_SYS,
    BTC_UNITY,
    BTC_USD_d,
    BTC_USD,
    BTC_VIA,
    BTC_VOX,
    BTC_VRC,
    BTC_VTC,
    BTC_XBC,
    BTC_XC