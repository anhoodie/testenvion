
use bidir_map::BidirMap;
use serde_json;
use serde_json::Value;
use serde_json::value::Map;

use error;
use pair::Pair;
use pair::Pair::*;

lazy_static! {
    static ref PAIRS_STRING: BidirMap<Pair, &'static str> = {
        let mut m = BidirMap::new();
        m.insert(BTC_CAD_d, "XXBTZCAD.d");
        m.insert(BTC_CAD,   "XXBTZCAD");
        m.insert(BTC_EUR_d, "XXBTZEUR.d");
        m.insert(BTC_EUR,   "XXBTZEUR");
        m.insert(BTC_GBP_d, "XXBTZGBP.d");
        m.insert(BTC_GBP,   "XXBTZGBP");
        m.insert(BTC_JPY_d, "XXBTZJPY.d");
        m.insert(BTC_JPY,   "XXBTZJPY");
        m.insert(BTC_USD_d, "XXBTZUSD.d");
        m.insert(BTC_USD,   "XXBTZUSD");
        m.insert(ETC_BTC,   "XETCXXBT");
        m.insert(ETC_ETH,   "XETCXETH");
        m.insert(ETC_EUR,   "XETCZEUR");
        m.insert(ETC_USD,   "XETCZUSD");
        m.insert(ETH_BTC,   "XETHXXBT");
        m.insert(ETH_CAD_d, "XETHZCAD.d");
        m.insert(ETH_CAD,   "XETHZCAD");
        m.insert(ETH_EUR_d, "XETHZEUR.d");
        m.insert(ETH_EUR,   "XETHZEUR");
        m.insert(ETH_GBP_d, "XETHZGBP.d");
        m.insert(ETH_GBP,   "XETHZGBP");
        m.insert(ETH_JPY_d, "XETHZJPY.d");
        m.insert(ETH_JPY,   "XETHZJPY");
        m.insert(ETH_USD_d, "XETHZUSD.d");
        m.insert(ETH_USD,   "XETHZUSD");
        m.insert(ETH_XBT_d, "XETHXXBT.d");
        m.insert(ICN_BTC,   "XICNXXBT");
        m.insert(ICN_ETH,   "XICNXETH");
        m.insert(LTC_BTC,   "XLTCXXBT");
        m.insert(LTC_EUR,   "XLTCZEUR");
        m.insert(LTC_USD,   "XLTCZUSD");
        m.insert(MLN_BTC,   "XMLNXXBT");
        m.insert(MLN_ETH,   "XMLNXETH");
        m.insert(REP_BTC,   "XREPXXBT");
        m.insert(REP_ETH,   "XREPXETH");
        m.insert(REP_EUR,   "XREPZEUR");
        m.insert(REP_USD,   "XREPZUSD");
        m.insert(USDT_USD,  "USDTZUSD");
        m.insert(XDG_BTC,   "XXDGXXBT");
        m.insert(XLM_BTC,   "XXLMXXBT");
        m.insert(XLM_EUR,   "XXLMZEUR");
        m.insert(XLM_USD,   "XXLMZUSD");
        m.insert(XMR_BTC,   "XXMRXXBT");
        m.insert(XMR_EUR,   "XXMRZEUR");
        m.insert(XMR_USD,   "XXMRZUSD");
        m.insert(XRP_BTC,   "XXRPXXBT");
        m.insert(ZEC_BTC,   "XZECXXBT");
        m.insert(ZEC_EUR,   "XZECZEUR");
        m.insert(ZEC_USD,   "XZECZUSD");
        m
    };
}

/// Return the name associated to pair used by Kraken
/// If the Pair is not supported, None is returned.