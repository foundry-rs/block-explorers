use alloy_primitives::U64;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::{fmt, str::FromStr};

/// A block number or tag.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub enum BlockNumber {
    /// Latest block
    #[default]
    Latest,
    /// Finalized block accepted as canonical
    Finalized,
    /// Safe head block
    Safe,
    /// Earliest block (genesis)
    Earliest,
    /// Pending block (not yet part of the blockchain)
    Pending,
    /// Block by number from canon chain
    Number(U64),
}

impl BlockNumber {
    /// Returns the numeric block number if explicitly set
    pub fn as_number(&self) -> Option<U64> {
        match *self {
            Self::Number(num) => Some(num),
            _ => None,
        }
    }

    /// Returns `true` if a numeric block number is set
    pub fn is_number(&self) -> bool {
        matches!(self, Self::Number(_))
    }

    /// Returns `true` if it's "latest"
    pub fn is_latest(&self) -> bool {
        matches!(self, Self::Latest)
    }

    /// Returns `true` if it's "finalized"
    pub fn is_finalized(&self) -> bool {
        matches!(self, Self::Finalized)
    }

    /// Returns `true` if it's "safe"
    pub fn is_safe(&self) -> bool {
        matches!(self, Self::Safe)
    }

    /// Returns `true` if it's "pending"
    pub fn is_pending(&self) -> bool {
        matches!(self, Self::Pending)
    }

    /// Returns `true` if it's "earliest"
    pub fn is_earliest(&self) -> bool {
        matches!(self, Self::Earliest)
    }
}

impl<T: Into<U64>> From<T> for BlockNumber {
    fn from(num: T) -> Self {
        Self::Number(num.into())
    }
}

impl Serialize for BlockNumber {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            Self::Number(ref x) => serializer.serialize_str(&format!("0x{x:x}")),
            Self::Latest => serializer.serialize_str("latest"),
            Self::Finalized => serializer.serialize_str("finalized"),
            Self::Safe => serializer.serialize_str("safe"),
            Self::Earliest => serializer.serialize_str("earliest"),
            Self::Pending => serializer.serialize_str("pending"),
        }
    }
}

impl<'de> Deserialize<'de> for BlockNumber {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?.to_lowercase();
        s.parse().map_err(serde::de::Error::custom)
    }
}

impl FromStr for BlockNumber {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "latest" => Ok(Self::Latest),
            "finalized" => Ok(Self::Finalized),
            "safe" => Ok(Self::Safe),
            "earliest" => Ok(Self::Earliest),
            "pending" => Ok(Self::Pending),
            // hex
            n if n.starts_with("0x") => n.parse().map(Self::Number).map_err(|e| e.to_string()),
            // decimal
            n => n.parse::<u64>().map(|n| Self::Number(U64::from(n))).map_err(|e| e.to_string()),
        }
    }
}

impl fmt::Display for BlockNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Number(x) => format!("0x{x:x}").fmt(f),
            Self::Latest => f.write_str("latest"),
            Self::Finalized => f.write_str("finalized"),
            Self::Safe => f.write_str("safe"),
            Self::Earliest => f.write_str("earliest"),
            Self::Pending => f.write_str("pending"),
        }
    }
}
