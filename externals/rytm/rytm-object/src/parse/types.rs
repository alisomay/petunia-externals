use crate::{
    api::{
        object_type::*,
        plock_type::{PLOCK_CLEAR, PLOCK_GET, PLOCK_SET},
    },
    error::ParseError,
    value::RytmValue,
};
use std::str::FromStr;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum PlockOperation {
    Get,
    Set,
    Clear,
}

impl FromStr for PlockOperation {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            PLOCK_GET => Ok(Self::Get),
            PLOCK_SET => Ok(Self::Set),
            PLOCK_CLEAR => Ok(Self::Clear),
            other => Err(ParseError::InvalidPlockOperation(
                other.to_owned(),
                "Try using plockget, plockset or plockclear.".to_owned(),
            )),
        }
    }
}

impl std::fmt::Display for PlockOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PlockOperation::Get => write!(f, "{}", PLOCK_GET),
            PlockOperation::Set => write!(f, "{}", PLOCK_SET),
            PlockOperation::Clear => write!(f, "{}", PLOCK_CLEAR),
        }
    }
}

pub type ParseResult<T> = Result<T, ParseError>;

#[derive(Debug, PartialEq, Clone)]
pub enum ParsedValue {
    /// The object type with optional index
    ObjectType(ObjectTypeSelector),
    /// An identifier (e.g., "name", "index", etc.)
    Identifier(String),
    /// A parameter value (numeric)
    Parameter(Number),
    /// A string parameter (e.g., "hello")
    ParameterString(String),
    /// An enum with optional value (e.g., "speed:1x" or "speed:")
    Enum(String, Option<String>),
    /// The track index
    TrackIndex(usize),
    /// The trig index
    TrigIndex(usize),
    /// The sound index (for sound elements within kit)
    SoundIndex(usize),
    /// An element (e.g., "sound", "tracklevel", etc.)
    Element(String),
    /// The element index
    ElementIndex(usize),
    /// A plock operation (e.g., "plockget", "plockset", "plockclear")
    PlockOperation(PlockOperation),
    /// The target index for a copy operation
    CopySourceIndex(usize),
    /// The target index for a copy operation
    CopyTargetIndex(usize),
}

impl std::fmt::Display for ParsedValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParsedValue::ObjectType(object_type) => write!(f, "{}", object_type),
            ParsedValue::Identifier(s) => write!(f, "{}", s),
            ParsedValue::Parameter(Number::Int(i)) => write!(f, "{}", i),
            ParsedValue::Parameter(Number::Float(fl)) => write!(f, "{}", fl),
            ParsedValue::ParameterString(s) => write!(f, "{}", s),
            ParsedValue::Enum(s, Some(value)) => write!(f, "{}:{}", s, value),
            ParsedValue::Enum(s, None) => write!(f, "{}:", s),
            ParsedValue::TrackIndex(i) => write!(f, "{}", i),
            ParsedValue::TrigIndex(i) => write!(f, "{}", i),
            ParsedValue::SoundIndex(i) => write!(f, "{}", i),
            ParsedValue::Element(s) => write!(f, "{}", s),
            ParsedValue::ElementIndex(i) => write!(f, "{}", i),
            ParsedValue::PlockOperation(s) => write!(f, "{}", s),
            ParsedValue::CopyTargetIndex(i) => write!(f, "{}", i),
            ParsedValue::CopySourceIndex(i) => write!(f, "{}", i),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Number {
    Int(isize),
    Float(f64),
}

impl std::fmt::Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Int(i) => write!(f, "{}", i),
            Self::Float(fl) => write!(f, "{}", fl),
        }
    }
}

impl Number {
    pub fn get_bool_from_0_or_1(&self, identifier: &str) -> Result<bool, ParseError> {
        match self {
            Number::Int(value) => match value {
                0 => Ok(false),
                1 => Ok(true),
                _ => Err(ParseError::InvalidFormat(format!(
                    "Invalid parameter: {}. {} must be followed by a 0 or 1 (integer)",
                    value, identifier
                ))),
            },
            _ => Err(ParseError::InvalidFormat(format!(
                "Invalid parameter: {}. {} must be followed by a 0 or 1 (integer)",
                self, identifier
            ))),
        }
    }

    pub fn get_int(&self) -> isize {
        match self {
            Number::Int(i) => *i,
            Number::Float(fl) => *fl as isize,
        }
    }

    pub fn get_float(&self) -> f64 {
        match self {
            Number::Int(i) => *i as f64,
            Number::Float(fl) => *fl,
        }
    }
}

impl From<isize> for Number {
    fn from(value: isize) -> Self {
        Number::Int(value)
    }
}

impl From<f64> for Number {
    fn from(value: f64) -> Self {
        Number::Float(value)
    }
}

impl From<i32> for Number {
    fn from(value: i32) -> Self {
        Number::Int(value as isize)
    }
}

impl From<f32> for Number {
    fn from(value: f32) -> Self {
        Number::Float(value as f64)
    }
}

impl From<usize> for Number {
    fn from(value: usize) -> Self {
        Number::Int(value as isize)
    }
}

impl From<u32> for Number {
    fn from(value: u32) -> Self {
        Number::Int(value as isize)
    }
}

impl From<bool> for Number {
    fn from(value: bool) -> Self {
        Number::Int(value as isize)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ObjectTypeSelector {
    Pattern(usize),
    PatternWorkBuffer,
    Kit(usize),
    KitWorkBuffer,
    Sound(usize),
    SoundWorkBuffer(usize),
    Global(usize),
    GlobalWorkBuffer,
    Settings,
}

impl std::fmt::Display for ObjectTypeSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ObjectTypeSelector::Pattern(index) => write!(f, "pattern {}", index),
            ObjectTypeSelector::PatternWorkBuffer => write!(f, "pattern work buffer"),
            ObjectTypeSelector::Kit(index) => write!(f, "kit {}", index),
            ObjectTypeSelector::KitWorkBuffer => write!(f, "kit work buffer"),
            ObjectTypeSelector::Sound(index) => write!(f, "sound {}", index),
            ObjectTypeSelector::SoundWorkBuffer(index) => write!(f, "sound work buffer {}", index),
            ObjectTypeSelector::Global(index) => write!(f, "global {}", index),
            ObjectTypeSelector::GlobalWorkBuffer => write!(f, "global work buffer"),
            ObjectTypeSelector::Settings => write!(f, "settings"),
        }
    }
}

impl ObjectTypeSelector {
    pub const fn is_indexable(&self) -> bool {
        matches!(
            self,
            Self::Pattern(_)
                | Self::Kit(_)
                | Self::Sound(_)
                | Self::SoundWorkBuffer(_)
                | Self::Global(_)
        )
    }

    pub fn is_object_type_indexable(object_type: &RytmValue) -> bool {
        let object_type = object_type.to_string();

        matches!(
            object_type.as_str(),
            PATTERN | KIT | SOUND | SOUND_WORK_BUFFER | GLOBAL
        )
    }
}

impl TryFrom<(&RytmValue, Option<&RytmValue>)> for ObjectTypeSelector {
    type Error = ParseError;

    fn try_from((selector, index): (&RytmValue, Option<&RytmValue>)) -> Result<Self, Self::Error> {
        let selector_sym = match selector {
            RytmValue::Symbol(sym) => sym,
            _ => return Err(ParseError::InvalidSelector),
        };

        match selector_sym.as_str() {
            PATTERN => parse_indexed(index, 0..=127, Self::Pattern),
            PATTERN_WORK_BUFFER => Ok(Self::PatternWorkBuffer),
            KIT => parse_indexed(index, 0..=127, Self::Kit),
            KIT_WORK_BUFFER => Ok(Self::KitWorkBuffer),
            SOUND => parse_indexed(index, 0..=11, Self::Sound),
            SOUND_WORK_BUFFER => parse_indexed(index, 0..=11, Self::SoundWorkBuffer),
            GLOBAL => parse_indexed(index, 0..=3, Self::Global),
            GLOBAL_WORK_BUFFER => Ok(Self::GlobalWorkBuffer),
            SETTINGS => Ok(Self::Settings),
            _ => Err(ParseError::InvalidSelector),
        }
    }
}

fn parse_indexed<T>(
    index: Option<&RytmValue>,
    range: std::ops::RangeInclusive<usize>,
    constructor: impl Fn(usize) -> T,
) -> Result<T, ParseError> {
    let index = index.ok_or(ParseError::InvalidQueryFormat)?;
    match index {
        RytmValue::Int(i) => {
            let i = *i as usize;
            if range.contains(&i) {
                Ok(constructor(i))
            } else {
                Err(ParseError::InvalidIndexRange {
                    min: *range.start() as isize,
                    max: *range.end() as isize,
                    value: i as isize,
                })
            }
        }
        _ => Err(ParseError::InvalidIndexType),
    }
}
