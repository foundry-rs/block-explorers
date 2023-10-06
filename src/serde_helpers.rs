use alloy_primitives::{U256, U64};
use serde::{Deserialize, Deserializer};
use std::{
    convert::{TryFrom, TryInto},
    str::FromStr,
};

/// Helper type to parse numeric strings, `u64` and `U256`
#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum StringifiedNumeric {
    String(String),
    U256(U256),
    Num(serde_json::Number),
}

impl TryFrom<StringifiedNumeric> for U256 {
    type Error = String;

    fn try_from(value: StringifiedNumeric) -> Result<Self, Self::Error> {
        match value {
            StringifiedNumeric::U256(n) => Ok(n),
            StringifiedNumeric::Num(n) => {
                Ok(U256::from_str(&n.to_string()).map_err(|err| err.to_string())?)
            }
            StringifiedNumeric::String(s) => {
                if let Ok(val) = s.parse::<u128>() {
                    Ok(U256::from(val))
                } else if s.starts_with("0x") {
                    U256::from_str_radix(&s, 16).map_err(|err| err.to_string())
                } else {
                    U256::from_str(&s).map_err(|err| err.to_string())
                }
            }
        }
    }
}

impl TryFrom<StringifiedNumeric> for U64 {
    type Error = String;

    fn try_from(value: StringifiedNumeric) -> Result<Self, Self::Error> {
        let value = U256::try_from(value)?;
        Ok(value.wrapping_to::<U64>())
    }
}

/// Supports parsing u64
///
/// See <https://github.com/gakonst/ethers-rs/issues/1507>
pub fn deserialize_stringified_u64<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: Deserializer<'de>,
{
    let num = StringifiedNumeric::deserialize(deserializer)?;
    let num: U256 = num.try_into().map_err(serde::de::Error::custom)?;
    num.try_into().map_err(serde::de::Error::custom)
}
