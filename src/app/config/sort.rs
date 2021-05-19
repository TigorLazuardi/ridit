use serde::{Deserialize, Deserializer};
use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum Sort {
    Hot,
    New,
    Rising,
    Controversial,
    Top,
}

impl<'de> Deserialize<'de> for Sort {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)
            .unwrap_or_default()
            .to_lowercase();

        let sort = match s.as_str() {
            "new" => Sort::New,
            "rising" => Sort::Rising,
            "controversial" => Sort::Controversial,
            "top" => Sort::Top,
            _ => Sort::Hot,
        };
        Ok(sort)
    }
}

impl fmt::Display for Sort {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Hot => write!(f, "hot"),
            Self::New => write!(f, "new"),
            Self::Rising => write!(f, "rising"),
            Self::Controversial => write!(f, "controversial"),
            Self::Top => write!(f, "top"),
        }
    }
}
