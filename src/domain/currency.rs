use lazy_static::lazy_static;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum CurrencyType {
    Fiat,
    Crypto,
}

lazy_static! {
    static ref FIAT_LOWER: String = "fiat".to_string();
    static ref CRYPTO_LOWER: String = "crypto".to_string();
}
impl CurrencyType {
    pub fn to_lowercase(&self) -> &'static String {
        match self {
            CurrencyType::Fiat => &FIAT_LOWER,
            CurrencyType::Crypto => &CRYPTO_LOWER,
        }
    }

    fn bisq_internal_precision(&self) -> u32 {
        match self {
            CurrencyType::Fiat => 4,
            CurrencyType::Crypto => 8,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Currency {
    pub code: String,
    pub name: String,
    pub currency_type: CurrencyType,
}

impl Currency {
    pub fn bisq_internal_precision(&self) -> u32 {
        self.currency_type.bisq_internal_precision()
    }

    pub fn from_code(code: &str) -> Option<&'static Currency> {
        ALL.iter().find(|c| &c.code == code)
    }
}
impl FromStr for &Currency {
    type Err = ();
    fn from_str(code: &str) -> Result<Self, Self::Err> {
        Currency::from_code(code).ok_or(())
    }
}

lazy_static! {
    pub static ref ALL: Vec<Currency> = {
        let mut vec = Vec::with_capacity(249);
        vec.push(Currency {
            code: "ACM".to_string(),
            name: "Actinium".to_string(),
            currency_type: CurrencyType::Crypto,
        });
        vec.push(Currency {
            code: "ADE".to_string(),
            name: "Adeptio".to_string(),
            currency_type: CurrencyType::Crypto,
        });
        vec.push(Currency {
            code: "AED".to_string(),
            name: "United Arab Emirates Dirham".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "AEON".to_string(),
            name: "Aeon".to_string(),
            currency_type: CurrencyType::Crypto,
        });
        vec.push(Currency {
            code: "AEUR".to_string(),
            name: "Augmint Euro".to_string(),
            currency_type: CurrencyType::Crypto,
        });
        vec.push(Currency {
            code: "AFN".to_string(),
            name: "Afghan Afghani".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "ALL".to_string(),
            name: "Albanian Lek".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "AMD".to_string(),
            name: "Armenian Dram".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "AMIT".to_string(),
            name: "Amitycoin".to_string(),
            currency_type: CurrencyType::Crypto,
        });
        vec.push(Currency {
            code: "ANG".to_string(),
            name: "Netherlands Antillean Guilder".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "AOA".to_string(),
            name: "Angolan Kwanza".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "ARQ".to_string(),
            name: "Arqma".to_string(),
            currency_type: CurrencyType::Crypto,
        });
        vec.push(Currency {
            code: "ARS".to_string(),
            name: "Argentine Peso".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "ASK".to_string(),
            name: "Askcoin".to_string(),
            currency_type: CurrencyType::Crypto,
        });
        vec.push(Currency {
            code: "AUD".to_string(),
            name: "Australian Dollar".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "AUS".to_string(),
            name: "Australiacash".to_string(),
            currency_type: CurrencyType::Crypto,
        });
        vec.push(Currency {
            code: "AWG".to_string(),
            name: "Aruban Florin".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "AZN".to_string(),
            name: "Azerbaijani Manat".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "BAM".to_string(),
            name: "Bosnia-Herzegovina Convertible Mark".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "BBD".to_string(),
            name: "Barbadian Dollar".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "BDT".to_string(),
            name: "Bangladeshi Taka".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "BEAM".to_string(),
            name: "Beam".to_string(),
            currency_type: CurrencyType::Crypto,
        });
        vec.push(Currency {
            code: "BGN".to_string(),
            name: "Bulgarian Lev".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "BHD".to_string(),
            name: "Bahraini Dinar".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "BIF".to_string(),
            name: "Burundian Franc".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "BLUR".to_string(),
            name: "Blur".to_string(),
            currency_type: CurrencyType::Crypto,
        });
        vec.push(Currency {
            code: "BMD".to_string(),
            name: "Bermudan Dollar".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "BND".to_string(),
            name: "Brunei Dollar".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "BOB".to_string(),
            name: "Bolivian Boliviano".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "BRL".to_string(),
            name: "Brazilian Real".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "BSD".to_string(),
            name: "Bahamian Dollar".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "BSQ".to_string(),
            name: "BSQ".to_string(),
            currency_type: CurrencyType::Crypto,
        });
        vec.push(Currency {
            code: "BTC".to_string(),
            name: "Bitcoin".to_string(),
            currency_type: CurrencyType::Crypto,
        });
        vec.push(Currency {
            code: "BTM".to_string(),
            name: "Bitmark".to_string(),
            currency_type: CurrencyType::Crypto,
        });
        vec.push(Currency {
            code: "BTN".to_string(),
            name: "Bhutanese Ngultrum".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "BWP".to_string(),
            name: "Botswanan Pula".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "BYN".to_string(),
            name: "Belarusian Ruble".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "BZC".to_string(),
            name: "Bitzec".to_string(),
            currency_type: CurrencyType::Crypto,
        });
        vec.push(Currency {
            code: "BZD".to_string(),
            name: "Belize Dollar".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "CAD".to_string(),
            name: "Canadian Dollar".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "CASH2".to_string(),
            name: "Cash2".to_string(),
            currency_type: CurrencyType::Crypto,
        });
        vec.push(Currency {
            code: "CDF".to_string(),
            name: "Congolese Franc".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "CHA".to_string(),
            name: "Chaucha".to_string(),
            currency_type: CurrencyType::Crypto,
        });
        vec.push(Currency {
            code: "CHF".to_string(),
            name: "Swiss Franc".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "CLOAK".to_string(),
            name: "CloakCoin".to_string(),
            currency_type: CurrencyType::Crypto,
        });
        vec.push(Currency {
            code: "CLP".to_string(),
            name: "Chilean Peso".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "CNY".to_string(),
            name: "Chinese Yuan".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "COP".to_string(),
            name: "Colombian Peso".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "CRC".to_string(),
            name: "Costa Rican Col\u{00f3}n".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "CRCL".to_string(),
            name: "CRowdCLassic".to_string(),
            currency_type: CurrencyType::Crypto,
        });
        vec.push(Currency {
            code: "CRDS".to_string(),
            name: "Credits".to_string(),
            currency_type: CurrencyType::Crypto,
        });
        vec.push(Currency {
            code: "CROAT".to_string(),
            name: "Croat".to_string(),
            currency_type: CurrencyType::Crypto,
        });
        vec.push(Currency {
            code: "CUP".to_string(),
            name: "Cuban Peso".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "CVE".to_string(),
            name: "Cape Verdean Escudo".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "CZK".to_string(),
            name: "Czech Republic Koruna".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "DAI".to_string(),
            name: "Dai Stablecoin".to_string(),
            currency_type: CurrencyType::Crypto,
        });
        vec.push(Currency {
            code: "DARX".to_string(),
            name: "BitDaric".to_string(),
            currency_type: CurrencyType::Crypto,
        });
        vec.push(Currency {
            code: "DASH".to_string(),
            name: "Dash".to_string(),
            currency_type: CurrencyType::Crypto,
        });
        vec.push(Currency {
            code: "DCR".to_string(),
            name: "Decred".to_string(),
            currency_type: CurrencyType::Crypto,
        });
        vec.push(Currency {
            code: "DJF".to_string(),
            name: "Djiboutian Franc".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "DKK".to_string(),
            name: "Danish Krone".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "DOGE".to_string(),
            name: "Dogecoin".to_string(),
            currency_type: CurrencyType::Crypto,
        });
        vec.push(Currency {
            code: "DOI".to_string(),
            name: "Doichain".to_string(),
            currency_type: CurrencyType::Crypto,
        });
        vec.push(Currency {
            code: "DOP".to_string(),
            name: "Dominican Peso".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "DRGL".to_string(),
            name: "Dragonglass".to_string(),
            currency_type: CurrencyType::Crypto,
        });
        vec.push(Currency {
            code: "DST".to_string(),
            name: "DSTRA".to_string(),
            currency_type: CurrencyType::Crypto,
        });
        vec.push(Currency {
            code: "DXO".to_string(),
            name: "Dextro".to_string(),
            currency_type: CurrencyType::Crypto,
        });
        vec.push(Currency {
            code: "DZD".to_string(),
            name: "Algerian Dinar".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "EGP".to_string(),
            name: "Egyptian Pound".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "ERN".to_string(),
            name: "Eritrean Nakfa".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "ETB".to_string(),
            name: "Ethiopian Birr".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "ETC".to_string(),
            name: "Ether Classic".to_string(),
            currency_type: CurrencyType::Crypto,
        });
        vec.push(Currency {
            code: "ETH".to_string(),
            name: "Ether".to_string(),
            currency_type: CurrencyType::Crypto,
        });
        vec.push(Currency {
            code: "ETHS".to_string(),
            name: "EtherStone".to_string(),
            currency_type: CurrencyType::Crypto,
        });
        vec.push(Currency {
            code: "EUR".to_string(),
            name: "Euro".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "FJC".to_string(),
            name: "Fujicoin".to_string(),
            currency_type: CurrencyType::Crypto,
        });
        vec.push(Currency {
            code: "FJD".to_string(),
            name: "Fijian Dollar".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "FKP".to_string(),
            name: "Falkland Islands Pound".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "FRTY".to_string(),
            name: "FourtyTwo".to_string(),
            currency_type: CurrencyType::Crypto,
        });
        vec.push(Currency {
            code: "GALI".to_string(),
            name: "Galilel".to_string(),
            currency_type: CurrencyType::Crypto,
        });
        vec.push(Currency {
            code: "GBP".to_string(),
            name: "British Pound".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "GBYTE".to_string(),
            name: "Byte".to_string(),
            currency_type: CurrencyType::Crypto,
        });
        vec.push(Currency {
            code: "GEL".to_string(),
            name: "Georgian Lari".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "GHS".to_string(),
            name: "Ghanaian Cedi".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "GIP".to_string(),
            name: "Gibraltar Pound".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "GMCN".to_string(),
            name: "GambleCoin".to_string(),
            currency_type: CurrencyType::Crypto,
        });
        vec.push(Currency {
            code: "GMD".to_string(),
            name: "Gambian Dalasi".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "GNF".to_string(),
            name: "Guinean Franc".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "GRIN".to_string(),
            name: "Grin".to_string(),
            currency_type: CurrencyType::Crypto,
        });
        vec.push(Currency {
            code: "GTQ".to_string(),
            name: "Guatemalan Quetzal".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "GYD".to_string(),
            name: "Guyanaese Dollar".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "HATCH".to_string(),
            name: "Hatch".to_string(),
            currency_type: CurrencyType::Crypto,
        });
        vec.push(Currency {
            code: "HKD".to_string(),
            name: "Hong Kong Dollar".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "HLM".to_string(),
            name: "Helium".to_string(),
            currency_type: CurrencyType::Crypto,
        });
        vec.push(Currency {
            code: "HNL".to_string(),
            name: "Honduran Lempira".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "HRK".to_string(),
            name: "Croatian Kuna".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "HTG".to_string(),
            name: "Haitian Gourde".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "HUF".to_string(),
            name: "Hungarian Forint".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "IDA".to_string(),
            name: "IdaPay".to_string(),
            currency_type: CurrencyType::Crypto,
        });
        vec.push(Currency {
            code: "IDR".to_string(),
            name: "Indonesian Rupiah".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "ILS".to_string(),
            name: "Israeli New Sheqel".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "INR".to_string(),
            name: "Indian Rupee".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "IQD".to_string(),
            name: "Iraqi Dinar".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "IRD".to_string(),
            name: "Iridium".to_string(),
            currency_type: CurrencyType::Crypto,
        });
        vec.push(Currency {
            code: "IRR".to_string(),
            name: "Iranian Rial".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "ISK".to_string(),
            name: "Icelandic Kr\u{00f3}na".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "JMD".to_string(),
            name: "Jamaican Dollar".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "JOD".to_string(),
            name: "Jordanian Dinar".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "JPY".to_string(),
            name: "Japanese Yen".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "KEK".to_string(),
            name: "Kekcoin".to_string(),
            currency_type: CurrencyType::Crypto,
        });
        vec.push(Currency {
            code: "KES".to_string(),
            name: "Kenyan Shilling".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "KGS".to_string(),
            name: "Kyrgystani Som".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "KHR".to_string(),
            name: "Cambodian Riel".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "KMF".to_string(),
            name: "Comorian Franc".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "KPW".to_string(),
            name: "North Korean Won".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "KRW".to_string(),
            name: "South Korean Won".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "KWD".to_string(),
            name: "Kuwaiti Dinar".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "KYD".to_string(),
            name: "Cayman Islands Dollar".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "KZT".to_string(),
            name: "Kazakhstani Tenge".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "LAK".to_string(),
            name: "Laotian Kip".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "LBP".to_string(),
            name: "Lebanese Pound".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "LCP".to_string(),
            name: "LitecoinPlus".to_string(),
            currency_type: CurrencyType::Crypto,
        });
        vec.push(Currency {
            code: "LKR".to_string(),
            name: "Sri Lankan Rupee".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "LRD".to_string(),
            name: "Liberian Dollar".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "LSL".to_string(),
            name: "Lesotho Loti".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "LTC".to_string(),
            name: "Litecoin".to_string(),
            currency_type: CurrencyType::Crypto,
        });
        vec.push(Currency {
            code: "LTZ".to_string(),
            name: "LitecoinZ".to_string(),
            currency_type: CurrencyType::Crypto,
        });
        vec.push(Currency {
            code: "LYD".to_string(),
            name: "Libyan Dinar".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "LYTX".to_string(),
            name: "Lytix".to_string(),
            currency_type: CurrencyType::Crypto,
        });
        vec.push(Currency {
            code: "MAD".to_string(),
            name: "Moroccan Dirham".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "MAI".to_string(),
            name: "Starwels".to_string(),
            currency_type: CurrencyType::Crypto,
        });
        vec.push(Currency {
            code: "MASK".to_string(),
            name: "Mask".to_string(),
            currency_type: CurrencyType::Crypto,
        });
        vec.push(Currency {
            code: "MBGL".to_string(),
            name: "MobitGlobal".to_string(),
            currency_type: CurrencyType::Crypto,
        });
        vec.push(Currency {
            code: "MDL".to_string(),
            name: "Moldovan Leu".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "MGA".to_string(),
            name: "Malagasy Ariary".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "MILE".to_string(),
            name: "Mile".to_string(),
            currency_type: CurrencyType::Crypto,
        });
        vec.push(Currency {
            code: "MKD".to_string(),
            name: "Macedonian Denar".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "MMK".to_string(),
            name: "Myanmar Kyat".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "MNT".to_string(),
            name: "Mongolian Tugrik".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "MOP".to_string(),
            name: "Macanese Pataca".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "MOX".to_string(),
            name: "MoX".to_string(),
            currency_type: CurrencyType::Crypto,
        });
        vec.push(Currency {
            code: "MQX".to_string(),
            name: "MirQuiX".to_string(),
            currency_type: CurrencyType::Crypto,
        });
        vec.push(Currency {
            code: "MRO".to_string(),
            name: "Mauritanian Ouguiya".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "MUE".to_string(),
            name: "MonetaryUnit".to_string(),
            currency_type: CurrencyType::Crypto,
        });
        vec.push(Currency {
            code: "MUR".to_string(),
            name: "Mauritian Rupee".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "MVR".to_string(),
            name: "Maldivian Rufiyaa".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "MWK".to_string(),
            name: "Malawian Kwacha".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "MXN".to_string(),
            name: "Mexican Peso".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "MYR".to_string(),
            name: "Malaysian Ringgit".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "MZN".to_string(),
            name: "Mozambican Metical".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "NAD".to_string(),
            name: "Namibian Dollar".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "NAV".to_string(),
            name: "Navcoin".to_string(),
            currency_type: CurrencyType::Crypto,
        });
        vec.push(Currency {
            code: "NEOS".to_string(),
            name: "Neos".to_string(),
            currency_type: CurrencyType::Crypto,
        });
        vec.push(Currency {
            code: "NGN".to_string(),
            name: "Nigerian Naira".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "NIO".to_string(),
            name: "Nicaraguan C\u{00f3}rdoba".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "NMC".to_string(),
            name: "Namecoin".to_string(),
            currency_type: CurrencyType::Crypto,
        });
        vec.push(Currency {
            code: "NOK".to_string(),
            name: "Norwegian Krone".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "NOR".to_string(),
            name: "Noir".to_string(),
            currency_type: CurrencyType::Crypto,
        });
        vec.push(Currency {
            code: "NPR".to_string(),
            name: "Nepalese Rupee".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "NZD".to_string(),
            name: "New Zealand Dollar".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "OMR".to_string(),
            name: "Omani Rial".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "ONION".to_string(),
            name: "DeepOnion".to_string(),
            currency_type: CurrencyType::Crypto,
        });
        vec.push(Currency {
            code: "PAB".to_string(),
            name: "Panamanian Balboa".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "PEN".to_string(),
            name: "Peruvian Nuevo Sol".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "PGK".to_string(),
            name: "Papua New Guinean Kina".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "PHP".to_string(),
            name: "Philippine Peso".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "PINK".to_string(),
            name: "Pinkcoin".to_string(),
            currency_type: CurrencyType::Crypto,
        });
        vec.push(Currency {
            code: "PIVX".to_string(),
            name: "PIVX".to_string(),
            currency_type: CurrencyType::Crypto,
        });
        vec.push(Currency {
            code: "PKR".to_string(),
            name: "Pakistani Rupee".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "PLE".to_string(),
            name: "Plenteum".to_string(),
            currency_type: CurrencyType::Crypto,
        });
        vec.push(Currency {
            code: "PLN".to_string(),
            name: "Polish Zloty".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "PRSN".to_string(),
            name: "Persona".to_string(),
            currency_type: CurrencyType::Crypto,
        });
        vec.push(Currency {
            code: "PYG".to_string(),
            name: "Paraguayan Guarani".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "PZDC".to_string(),
            name: "PZDC".to_string(),
            currency_type: CurrencyType::Crypto,
        });
        vec.push(Currency {
            code: "QAR".to_string(),
            name: "Qatari Rial".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "QBS".to_string(),
            name: "Qbase".to_string(),
            currency_type: CurrencyType::Crypto,
        });
        vec.push(Currency {
            code: "QMCoin".to_string(),
            name: "QMCoin".to_string(),
            currency_type: CurrencyType::Crypto,
        });
        vec.push(Currency {
            code: "QWC".to_string(),
            name: "Qwertycoin".to_string(),
            currency_type: CurrencyType::Crypto,
        });
        vec.push(Currency {
            code: "RADS".to_string(),
            name: "Radium".to_string(),
            currency_type: CurrencyType::Crypto,
        });
        vec.push(Currency {
            code: "RMX".to_string(),
            name: "Remix".to_string(),
            currency_type: CurrencyType::Crypto,
        });
        vec.push(Currency {
            code: "RON".to_string(),
            name: "Romanian Leu".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "RSD".to_string(),
            name: "Serbian Dinar".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "RUB".to_string(),
            name: "Russian Ruble".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "RWF".to_string(),
            name: "Rwandan Franc".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "RYO".to_string(),
            name: "Ryo".to_string(),
            currency_type: CurrencyType::Crypto,
        });
        vec.push(Currency {
            code: "SAR".to_string(),
            name: "Saudi Riyal".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "SBD".to_string(),
            name: "Solomon Islands Dollar".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "SCP".to_string(),
            name: "SiaPrimeCoin".to_string(),
            currency_type: CurrencyType::Crypto,
        });
        vec.push(Currency {
            code: "SCR".to_string(),
            name: "Seychellois Rupee".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "SDG".to_string(),
            name: "Sudanese Pound".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "SEK".to_string(),
            name: "Swedish Krona".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "SF".to_string(),
            name: "Siafund".to_string(),
            currency_type: CurrencyType::Crypto,
        });
        vec.push(Currency {
            code: "SGD".to_string(),
            name: "Singapore Dollar".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "SHP".to_string(),
            name: "St. Helena Pound".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "SLL".to_string(),
            name: "Sierra Leonean Leone".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "SOS".to_string(),
            name: "Somali Shilling".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "SPACE".to_string(),
            name: "SpaceCash".to_string(),
            currency_type: CurrencyType::Crypto,
        });
        vec.push(Currency {
            code: "SRD".to_string(),
            name: "Surinamese Dollar".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "SSP".to_string(),
            name: "South Sudanese Pound".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "STN".to_string(),
            name: "S\u{00e3}o Tom\u{00e9} and Pr\u{00ed}ncipe Dobra".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "SUB1X".to_string(),
            name: "SUB1X".to_string(),
            currency_type: CurrencyType::Crypto,
        });
        vec.push(Currency {
            code: "SVC".to_string(),
            name: "Salvadoran Col\u{00f3}n".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "SYP".to_string(),
            name: "Syrian Pound".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "SZL".to_string(),
            name: "Swazi Lilangeni".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "THB".to_string(),
            name: "Thai Baht".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "TJS".to_string(),
            name: "Tajikistani Somoni".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "TMT".to_string(),
            name: "Turkmenistani Manat".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "TND".to_string(),
            name: "Tunisian Dinar".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "TOP".to_string(),
            name: "Tongan Pa\u{02bb}anga".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "TRTL".to_string(),
            name: "TurtleCoin".to_string(),
            currency_type: CurrencyType::Crypto,
        });
        vec.push(Currency {
            code: "TRY".to_string(),
            name: "Turkish Lira".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "TTD".to_string(),
            name: "Trinidad & Tobago Dollar".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "TUSD".to_string(),
            name: "TrueUSD".to_string(),
            currency_type: CurrencyType::Crypto,
        });
        vec.push(Currency {
            code: "TWD".to_string(),
            name: "New Taiwan Dollar".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "TZS".to_string(),
            name: "Tanzanian Shilling".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "UAH".to_string(),
            name: "Ukrainian Hryvnia".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "UCC".to_string(),
            name: "UnitedCommunityCoin".to_string(),
            currency_type: CurrencyType::Crypto,
        });
        vec.push(Currency {
            code: "UGX".to_string(),
            name: "Ugandan Shilling".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "UNO".to_string(),
            name: "Unobtanium".to_string(),
            currency_type: CurrencyType::Crypto,
        });
        vec.push(Currency {
            code: "USD".to_string(),
            name: "US Dollar".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "USDC".to_string(),
            name: "USD Coin".to_string(),
            currency_type: CurrencyType::Crypto,
        });
        vec.push(Currency {
            code: "UYU".to_string(),
            name: "Uruguayan Peso".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "UZS".to_string(),
            name: "Uzbekistani Som".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "VEF".to_string(),
            name: "Venezuelan Bol\u{00ed}var".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "VEIL".to_string(),
            name: "Veil".to_string(),
            currency_type: CurrencyType::Crypto,
        });
        vec.push(Currency {
            code: "VND".to_string(),
            name: "Vietnamese Dong".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "VUV".to_string(),
            name: "Vanuatu Vatu".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "VXV".to_string(),
            name: "VectorspaceAI".to_string(),
            currency_type: CurrencyType::Crypto,
        });
        vec.push(Currency {
            code: "WEB".to_string(),
            name: "Webchain".to_string(),
            currency_type: CurrencyType::Crypto,
        });
        vec.push(Currency {
            code: "WRKZ".to_string(),
            name: "WrkzCoin".to_string(),
            currency_type: CurrencyType::Crypto,
        });
        vec.push(Currency {
            code: "WST".to_string(),
            name: "Samoan Tala".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "XAF".to_string(),
            name: "Central African CFA Franc".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "XCD".to_string(),
            name: "East Caribbean Dollar".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "XCP".to_string(),
            name: "Counterparty".to_string(),
            currency_type: CurrencyType::Crypto,
        });
        vec.push(Currency {
            code: "XDR0".to_string(),
            name: "XDR".to_string(),
            currency_type: CurrencyType::Crypto,
        });
        vec.push(Currency {
            code: "XMR".to_string(),
            name: "Monero".to_string(),
            currency_type: CurrencyType::Crypto,
        });
        vec.push(Currency {
            code: "XOF".to_string(),
            name: "West African CFA Franc".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "XPF".to_string(),
            name: "CFP Franc".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "XRC".to_string(),
            name: "Bitcoin Rhodium".to_string(),
            currency_type: CurrencyType::Crypto,
        });
        vec.push(Currency {
            code: "XSPEC".to_string(),
            name: "Spectrecoin".to_string(),
            currency_type: CurrencyType::Crypto,
        });
        vec.push(Currency {
            code: "XZC".to_string(),
            name: "Zcoin".to_string(),
            currency_type: CurrencyType::Crypto,
        });
        vec.push(Currency {
            code: "YER".to_string(),
            name: "Yemeni Rial".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "ZAR".to_string(),
            name: "South African Rand".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "ZEC".to_string(),
            name: "Zcash".to_string(),
            currency_type: CurrencyType::Crypto,
        });
        vec.push(Currency {
            code: "ZEL".to_string(),
            name: "ZelCash".to_string(),
            currency_type: CurrencyType::Crypto,
        });
        vec.push(Currency {
            code: "ZEN".to_string(),
            name: "Horizen".to_string(),
            currency_type: CurrencyType::Crypto,
        });
        vec.push(Currency {
            code: "ZER".to_string(),
            name: "Zero".to_string(),
            currency_type: CurrencyType::Crypto,
        });
        vec.push(Currency {
            code: "ZMW".to_string(),
            name: "Zambian Kwacha".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec.push(Currency {
            code: "ZWL".to_string(),
            name: "Zimbabwean Dollar (2009)".to_string(),
            currency_type: CurrencyType::Fiat,
        });
        vec
    };
}
