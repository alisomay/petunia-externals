use crate::error::RytmExternalError;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SaveTarget {
    NotProvided,
    Pattern,
    Kit,
    Sound,
    Global,
    Settings,
    // TODO:
    // Song,
}

impl std::fmt::Display for SaveTarget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Pattern => write!(f, "pattern"),
            Self::Kit => write!(f, "kit"),
            Self::Sound => write!(f, "sound"),
            Self::Global => write!(f, "global"),
            Self::Settings => write!(f, "settings"),
            Self::NotProvided => write!(f, "not provided"),
        }
    }
}

impl FromStr for SaveTarget {
    type Err = RytmExternalError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "pattern" => Ok(Self::Pattern),
            "kit" => Ok(Self::Kit),
            "sound" => Ok(Self::Sound),
            "global" => Ok(Self::Global),
            "settings" => Ok(Self::Settings),
            "" => Ok(Self::NotProvided),
            _ => Err(RytmExternalError::Custom(format!(
                "Save Error: Invalid save target provided as the first argument {s}. Please try one of the following: pattern, kit, sound, global, settings. Or leave it empty."
            ))),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SaveTargetIndex {
    Some(usize),
    NotNecessary,
    Ignore,
}

impl std::fmt::Display for SaveTargetIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Some(index) => write!(f, "{index}"),
            Self::NotNecessary => write!(f, "not necessary"),
            Self::Ignore => write!(f, "ignored"),
        }
    }
}
