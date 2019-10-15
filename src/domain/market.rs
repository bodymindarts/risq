use super::currency::{from_code, Currency};
use lazy_static::lazy_static;

#[derive(Clone, Debug)]
pub struct Market {
    pub pair: String,
    pub name: String,
    pub left: &'static Currency,
    pub right: &'static Currency,
}

macro_rules! new_market {
    ($left:literal, $right:literal) => {{
        let left = from_code($left).unwrap();
        let right = from_code($right).unwrap();
        Market {
            pair: format!("{}_{}", left.code.to_lowercase(), right.code.to_lowercase()),
            name: format!("{}/{}", left.name, right.name),
            left,
            right,
        }
    }};
}

pub fn from_pair(left: &Currency, right: &Currency) -> Option<&'static Market> {
    ALL.iter()
        .find(|m| m.left.code == left.code && m.right.code == right.code)
}

lazy_static! {
    pub static ref ALL: Vec<Market> = {
        let mut vec = Vec::with_capacity(248);
        vec.push(new_market!("ACM", "BTC"));
        vec.push(new_market!("ADE", "BTC"));
        vec.push(new_market!("AEON", "BTC"));
        vec.push(new_market!("AEUR", "BTC"));
        vec.push(new_market!("AMIT", "BTC"));
        vec.push(new_market!("ARQ", "BTC"));
        vec.push(new_market!("ASK", "BTC"));
        vec.push(new_market!("AUS", "BTC"));
        vec.push(new_market!("BEAM", "BTC"));
        vec.push(new_market!("BLUR", "BTC"));
        vec.push(new_market!("BSQ", "BTC"));
        vec.push(new_market!("BTC", "AED"));
        vec.push(new_market!("BTC", "AFN"));
        vec.push(new_market!("BTC", "ALL"));
        vec.push(new_market!("BTC", "AMD"));
        vec.push(new_market!("BTC", "ANG"));
        vec.push(new_market!("BTC", "AOA"));
        vec.push(new_market!("BTC", "ARS"));
        vec.push(new_market!("BTC", "AUD"));
        vec.push(new_market!("BTC", "AWG"));
        vec.push(new_market!("BTC", "AZN"));
        vec.push(new_market!("BTC", "BAM"));
        vec.push(new_market!("BTC", "BBD"));
        vec.push(new_market!("BTC", "BDT"));
        vec.push(new_market!("BTC", "BGN"));
        vec.push(new_market!("BTC", "BHD"));
        vec.push(new_market!("BTC", "BIF"));
        vec.push(new_market!("BTC", "BMD"));
        vec.push(new_market!("BTC", "BND"));
        vec.push(new_market!("BTC", "BOB"));
        vec.push(new_market!("BTC", "BRL"));
        vec.push(new_market!("BTC", "BSD"));
        vec.push(new_market!("BTC", "BTN"));
        vec.push(new_market!("BTC", "BWP"));
        vec.push(new_market!("BTC", "BYN"));
        vec.push(new_market!("BTC", "BZD"));
        vec.push(new_market!("BTC", "CAD"));
        vec.push(new_market!("BTC", "CDF"));
        vec.push(new_market!("BTC", "CHF"));
        vec.push(new_market!("BTC", "CLP"));
        vec.push(new_market!("BTC", "CNY"));
        vec.push(new_market!("BTC", "COP"));
        vec.push(new_market!("BTC", "CRC"));
        vec.push(new_market!("BTC", "CUP"));
        vec.push(new_market!("BTC", "CVE"));
        vec.push(new_market!("BTC", "CZK"));
        vec.push(new_market!("BTC", "DJF"));
        vec.push(new_market!("BTC", "DKK"));
        vec.push(new_market!("BTC", "DOP"));
        vec.push(new_market!("BTC", "DZD"));
        vec.push(new_market!("BTC", "EGP"));
        vec.push(new_market!("BTC", "ERN"));
        vec.push(new_market!("BTC", "ETB"));
        vec.push(new_market!("BTC", "EUR"));
        vec.push(new_market!("BTC", "FJD"));
        vec.push(new_market!("BTC", "FKP"));
        vec.push(new_market!("BTC", "GBP"));
        vec.push(new_market!("BTC", "GEL"));
        vec.push(new_market!("BTC", "GHS"));
        vec.push(new_market!("BTC", "GIP"));
        vec.push(new_market!("BTC", "GMD"));
        vec.push(new_market!("BTC", "GNF"));
        vec.push(new_market!("BTC", "GTQ"));
        vec.push(new_market!("BTC", "GYD"));
        vec.push(new_market!("BTC", "HKD"));
        vec.push(new_market!("BTC", "HNL"));
        vec.push(new_market!("BTC", "HRK"));
        vec.push(new_market!("BTC", "HTG"));
        vec.push(new_market!("BTC", "HUF"));
        vec.push(new_market!("BTC", "IDR"));
        vec.push(new_market!("BTC", "ILS"));
        vec.push(new_market!("BTC", "INR"));
        vec.push(new_market!("BTC", "IQD"));
        vec.push(new_market!("BTC", "IRR"));
        vec.push(new_market!("BTC", "ISK"));
        vec.push(new_market!("BTC", "JMD"));
        vec.push(new_market!("BTC", "JOD"));
        vec.push(new_market!("BTC", "JPY"));
        vec.push(new_market!("BTC", "KES"));
        vec.push(new_market!("BTC", "KGS"));
        vec.push(new_market!("BTC", "KHR"));
        vec.push(new_market!("BTC", "KMF"));
        vec.push(new_market!("BTC", "KPW"));
        vec.push(new_market!("BTC", "KRW"));
        vec.push(new_market!("BTC", "KWD"));
        vec.push(new_market!("BTC", "KYD"));
        vec.push(new_market!("BTC", "KZT"));
        vec.push(new_market!("BTC", "LAK"));
        vec.push(new_market!("BTC", "LBP"));
        vec.push(new_market!("BTC", "LKR"));
        vec.push(new_market!("BTC", "LRD"));
        vec.push(new_market!("BTC", "LSL"));
        vec.push(new_market!("BTC", "LYD"));
        vec.push(new_market!("BTC", "MAD"));
        vec.push(new_market!("BTC", "MDL"));
        vec.push(new_market!("BTC", "MGA"));
        vec.push(new_market!("BTC", "MKD"));
        vec.push(new_market!("BTC", "MMK"));
        vec.push(new_market!("BTC", "MNT"));
        vec.push(new_market!("BTC", "MOP"));
        vec.push(new_market!("BTC", "MRO"));
        vec.push(new_market!("BTC", "MUR"));
        vec.push(new_market!("BTC", "MVR"));
        vec.push(new_market!("BTC", "MWK"));
        vec.push(new_market!("BTC", "MXN"));
        vec.push(new_market!("BTC", "MYR"));
        vec.push(new_market!("BTC", "MZN"));
        vec.push(new_market!("BTC", "NAD"));
        vec.push(new_market!("BTC", "NGN"));
        vec.push(new_market!("BTC", "NIO"));
        vec.push(new_market!("BTC", "NOK"));
        vec.push(new_market!("BTC", "NPR"));
        vec.push(new_market!("BTC", "NZD"));
        vec.push(new_market!("BTC", "OMR"));
        vec.push(new_market!("BTC", "PAB"));
        vec.push(new_market!("BTC", "PEN"));
        vec.push(new_market!("BTC", "PGK"));
        vec.push(new_market!("BTC", "PHP"));
        vec.push(new_market!("BTC", "PKR"));
        vec.push(new_market!("BTC", "PLN"));
        vec.push(new_market!("BTC", "PYG"));
        vec.push(new_market!("BTC", "QAR"));
        vec.push(new_market!("BTC", "RON"));
        vec.push(new_market!("BTC", "RSD"));
        vec.push(new_market!("BTC", "RUB"));
        vec.push(new_market!("BTC", "RWF"));
        vec.push(new_market!("BTC", "SAR"));
        vec.push(new_market!("BTC", "SBD"));
        vec.push(new_market!("BTC", "SCR"));
        vec.push(new_market!("BTC", "SDG"));
        vec.push(new_market!("BTC", "SEK"));
        vec.push(new_market!("BTC", "SGD"));
        vec.push(new_market!("BTC", "SHP"));
        vec.push(new_market!("BTC", "SLL"));
        vec.push(new_market!("BTC", "SOS"));
        vec.push(new_market!("BTC", "SRD"));
        vec.push(new_market!("BTC", "SSP"));
        vec.push(new_market!("BTC", "STN"));
        vec.push(new_market!("BTC", "SVC"));
        vec.push(new_market!("BTC", "SYP"));
        vec.push(new_market!("BTC", "SZL"));
        vec.push(new_market!("BTC", "THB"));
        vec.push(new_market!("BTC", "TJS"));
        vec.push(new_market!("BTC", "TMT"));
        vec.push(new_market!("BTC", "TND"));
        vec.push(new_market!("BTC", "TOP"));
        vec.push(new_market!("BTC", "TRY"));
        vec.push(new_market!("BTC", "TTD"));
        vec.push(new_market!("BTC", "TWD"));
        vec.push(new_market!("BTC", "TZS"));
        vec.push(new_market!("BTC", "UAH"));
        vec.push(new_market!("BTC", "UGX"));
        vec.push(new_market!("BTC", "USD"));
        vec.push(new_market!("BTC", "UYU"));
        vec.push(new_market!("BTC", "UZS"));
        vec.push(new_market!("BTC", "VEF"));
        vec.push(new_market!("BTC", "VND"));
        vec.push(new_market!("BTC", "VUV"));
        vec.push(new_market!("BTC", "WST"));
        vec.push(new_market!("BTC", "XAF"));
        vec.push(new_market!("BTC", "XCD"));
        vec.push(new_market!("BTC", "XOF"));
        vec.push(new_market!("BTC", "XPF"));
        vec.push(new_market!("BTC", "YER"));
        vec.push(new_market!("BTC", "ZAR"));
        vec.push(new_market!("BTC", "ZMW"));
        vec.push(new_market!("BTC", "ZWL"));
        vec.push(new_market!("BTM", "BTC"));
        vec.push(new_market!("BZC", "BTC"));
        vec.push(new_market!("CASH2", "BTC"));
        vec.push(new_market!("CHA", "BTC"));
        vec.push(new_market!("CLOAK", "BTC"));
        vec.push(new_market!("CRCL", "BTC"));
        vec.push(new_market!("CRDS", "BTC"));
        vec.push(new_market!("CROAT", "BTC"));
        vec.push(new_market!("DAI", "BTC"));
        vec.push(new_market!("DARX", "BTC"));
        vec.push(new_market!("DASH", "BTC"));
        vec.push(new_market!("DCR", "BTC"));
        vec.push(new_market!("DOGE", "BTC"));
        vec.push(new_market!("DOI", "BTC"));
        vec.push(new_market!("DRGL", "BTC"));
        vec.push(new_market!("DST", "BTC"));
        vec.push(new_market!("DXO", "BTC"));
        vec.push(new_market!("ETC", "BTC"));
        vec.push(new_market!("ETH", "BTC"));
        vec.push(new_market!("ETHS", "BTC"));
        vec.push(new_market!("FJC", "BTC"));
        vec.push(new_market!("FRTY", "BTC"));
        vec.push(new_market!("GALI", "BTC"));
        vec.push(new_market!("GBYTE", "BTC"));
        vec.push(new_market!("GMCN", "BTC"));
        vec.push(new_market!("GRIN", "BTC"));
        vec.push(new_market!("HATCH", "BTC"));
        vec.push(new_market!("HLM", "BTC"));
        vec.push(new_market!("IDA", "BTC"));
        vec.push(new_market!("IRD", "BTC"));
        vec.push(new_market!("KEK", "BTC"));
        vec.push(new_market!("LCP", "BTC"));
        vec.push(new_market!("LTC", "BTC"));
        vec.push(new_market!("LTZ", "BTC"));
        vec.push(new_market!("LYTX", "BTC"));
        vec.push(new_market!("MAI", "BTC"));
        vec.push(new_market!("MASK", "BTC"));
        vec.push(new_market!("MBGL", "BTC"));
        vec.push(new_market!("MILE", "BTC"));
        vec.push(new_market!("MOX", "BTC"));
        vec.push(new_market!("MQX", "BTC"));
        vec.push(new_market!("MUE", "BTC"));
        vec.push(new_market!("NAV", "BTC"));
        vec.push(new_market!("NEOS", "BTC"));
        vec.push(new_market!("NMC", "BTC"));
        vec.push(new_market!("NOR", "BTC"));
        vec.push(new_market!("ONION", "BTC"));
        vec.push(new_market!("PINK", "BTC"));
        vec.push(new_market!("PIVX", "BTC"));
        vec.push(new_market!("PLE", "BTC"));
        vec.push(new_market!("PRSN", "BTC"));
        vec.push(new_market!("PZDC", "BTC"));
        vec.push(new_market!("QBS", "BTC"));
        vec.push(new_market!("QMCoin", "BTC"));
        vec.push(new_market!("QWC", "BTC"));
        vec.push(new_market!("RADS", "BTC"));
        vec.push(new_market!("RMX", "BTC"));
        vec.push(new_market!("RYO", "BTC"));
        vec.push(new_market!("SCP", "BTC"));
        vec.push(new_market!("SF", "BTC"));
        vec.push(new_market!("SPACE", "BTC"));
        vec.push(new_market!("SUB1X", "BTC"));
        vec.push(new_market!("TRTL", "BTC"));
        vec.push(new_market!("TUSD", "BTC"));
        vec.push(new_market!("UCC", "BTC"));
        vec.push(new_market!("UNO", "BTC"));
        vec.push(new_market!("USDC", "BTC"));
        vec.push(new_market!("VEIL", "BTC"));
        vec.push(new_market!("VXV", "BTC"));
        vec.push(new_market!("WEB", "BTC"));
        vec.push(new_market!("WRKZ", "BTC"));
        vec.push(new_market!("XCP", "BTC"));
        vec.push(new_market!("XDR0", "BTC"));
        vec.push(new_market!("XMR", "BTC"));
        vec.push(new_market!("XRC", "BTC"));
        vec.push(new_market!("XSPEC", "BTC"));
        vec.push(new_market!("XZC", "BTC"));
        vec.push(new_market!("ZEC", "BTC"));
        vec.push(new_market!("ZEL", "BTC"));
        vec.push(new_market!("ZEN", "BTC"));
        vec.push(new_market!("ZER", "BTC"));
        vec
    };
}
