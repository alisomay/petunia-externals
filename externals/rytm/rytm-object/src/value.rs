use crate::{error::RytmObjectError, parse::types::Number};
use median::{
    atom::{Atom, AtomType, AtomValue},
    symbol::SymbolRef,
};
use std::ffi::CString;

#[derive(Debug)]
pub struct RytmValueList(Vec<RytmValue>);

impl std::fmt::Display for RytmValueList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut string = String::new();
        for value in &self.0 {
            string.push_str(&format!("{} ", value));
        }
        write!(f, "{}", string)
    }
}

impl std::ops::Deref for RytmValueList {
    type Target = Vec<RytmValue>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for RytmValueList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<Vec<RytmValue>> for RytmValueList {
    fn from(v: Vec<RytmValue>) -> Self {
        Self(v)
    }
}

impl TryFrom<Vec<Atom>> for RytmValueList {
    type Error = ();
    fn try_from(v: Vec<Atom>) -> Result<Self, ()> {
        v.into_iter()
            .filter_map(|a| {
                if let Some(AtomType::Object) | None = a.get_type() {
                    None
                } else {
                    Some(RytmValue::try_from(a))
                }
            })
            .collect::<Result<Vec<RytmValue>, ()>>()
            .map(Self)
    }
}

impl TryFrom<&[Atom]> for RytmValueList {
    type Error = ();
    fn try_from(v: &[Atom]) -> Result<Self, ()> {
        v.iter()
            .filter_map(|a| {
                if let Some(AtomType::Object) | None = a.get_type() {
                    None
                } else {
                    Some(RytmValue::try_from(a))
                }
            })
            .collect::<Result<Vec<RytmValue>, ()>>()
            .map(Self)
    }
}

impl TryFrom<Vec<AtomValue>> for RytmValueList {
    type Error = ();
    fn try_from(v: Vec<AtomValue>) -> Result<Self, ()> {
        v.into_iter()
            .filter_map(|a| {
                if let AtomValue::Object(_) = a {
                    None
                } else {
                    Some(RytmValue::try_from(a))
                }
            })
            .collect::<Result<Vec<RytmValue>, ()>>()
            .map(Self)
    }
}

impl TryFrom<&[AtomValue]> for RytmValueList {
    type Error = ();
    fn try_from(v: &[AtomValue]) -> Result<Self, ()> {
        v.iter()
            .filter_map(|a| {
                if let AtomValue::Object(_) = a {
                    None
                } else {
                    Some(RytmValue::try_from(a))
                }
            })
            .collect::<Result<Vec<RytmValue>, ()>>()
            .map(Self)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum RytmValue {
    Float(f64),
    Int(isize),
    // We can not drive very fast with elektron and don't mind the extra allocation here.
    Symbol(String),
    // For rytm we don't need to handle objects. E.g. AtomValue::Object(*mut c_void)
    // Because we have no use for them.
}

impl From<Number> for RytmValue {
    fn from(v: Number) -> Self {
        match v {
            Number::Float(v) => Self::Float(v),
            Number::Int(v) => Self::Int(v),
        }
    }
}

impl std::fmt::Display for RytmValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Float(v) => write!(f, "{}", v),
            Self::Int(v) => write!(f, "{}", v),
            Self::Symbol(v) => write!(f, "{}", v),
        }
    }
}

impl From<String> for RytmValue {
    fn from(v: String) -> Self {
        Self::Symbol(v)
    }
}

impl From<&String> for RytmValue {
    fn from(v: &String) -> Self {
        Self::Symbol(v.clone())
    }
}

impl From<&mut String> for RytmValue {
    fn from(v: &mut String) -> Self {
        Self::Symbol(v.clone())
    }
}

impl From<&str> for RytmValue {
    fn from(v: &str) -> Self {
        Self::Symbol(v.to_string())
    }
}

impl From<f64> for RytmValue {
    fn from(v: f64) -> Self {
        Self::Float(v)
    }
}

impl From<&f64> for RytmValue {
    fn from(v: &f64) -> Self {
        Self::Float(*v)
    }
}

impl From<isize> for RytmValue {
    fn from(v: isize) -> Self {
        Self::Int(v)
    }
}

impl From<&isize> for RytmValue {
    fn from(v: &isize) -> Self {
        Self::Int(*v)
    }
}

impl From<i64> for RytmValue {
    fn from(v: i64) -> Self {
        Self::Int(v as isize)
    }
}

impl From<&i64> for RytmValue {
    fn from(v: &i64) -> Self {
        Self::Int(*v as isize)
    }
}

impl TryFrom<SymbolRef> for RytmValue {
    type Error = ();
    fn try_from(v: SymbolRef) -> Result<Self, ()> {
        v.to_string().map(Self::Symbol).map_err(|_| ())
    }
}

impl TryFrom<&SymbolRef> for RytmValue {
    type Error = ();
    fn try_from(v: &SymbolRef) -> Result<Self, ()> {
        v.to_string().map(Self::Symbol).map_err(|_| ())
    }
}

impl RytmValue {
    pub fn get_atom_type(&self) -> AtomType {
        match self {
            Self::Float(_) => AtomType::Float,
            Self::Int(_) => AtomType::Int,
            Self::Symbol(_) => AtomType::Symbol,
        }
    }

    pub fn as_atom_value(&self) -> AtomValue {
        match self {
            Self::Float(v) => AtomValue::Float(*v),
            Self::Int(v) => AtomValue::Int(*v),
            Self::Symbol(v) => AtomValue::Symbol(SymbolRef::from(
                CString::new(
                    v.as_bytes()
                        .iter()
                        .filter(|&&b| b != 0)
                        .copied()
                        .collect::<Vec<u8>>(),
                )
                .expect(
                    "Failed to convert to CString, we don't expect this to happen in this context.",
                ),
            )),
        }
    }

    pub fn as_atom(&self) -> Atom {
        Atom::from(self.as_atom_value())
    }
}

impl TryFrom<Atom> for RytmValue {
    type Error = ();
    fn try_from(v: Atom) -> Result<Self, ()> {
        v.get_value() // AtomValue
            .map(Self::try_from)
            .unwrap_or(Err(()))
    }
}

impl TryFrom<&Atom> for RytmValue {
    type Error = ();
    fn try_from(v: &Atom) -> Result<Self, ()> {
        v.get_value() // AtomValue
            .map(Self::try_from)
            .unwrap_or(Err(()))
    }
}

impl TryFrom<AtomValue> for RytmValue {
    type Error = ();
    fn try_from(v: AtomValue) -> Result<Self, ()> {
        match v {
            AtomValue::Int(v) => Ok(Self::Int(v)),
            AtomValue::Float(v) => Ok(Self::Float(v)),
            AtomValue::Symbol(v) => v.to_string().map(Self::Symbol).map_err(|_| ()),
            AtomValue::Object(_) => Err(()),
        }
    }
}

impl TryFrom<&AtomValue> for RytmValue {
    type Error = ();
    fn try_from(v: &AtomValue) -> Result<Self, ()> {
        match v {
            AtomValue::Int(v) => Ok(Self::Int(*v)),
            AtomValue::Float(v) => Ok(Self::Float(*v)),
            AtomValue::Symbol(v) => v.to_string().map(Self::Symbol).map_err(|_| ()),
            AtomValue::Object(_) => Err(()),
        }
    }
}

impl RytmValue {
    pub fn expect_number(&self) -> Result<(), RytmObjectError> {
        match self {
            Self::Int(_) | Self::Float(_) => Ok(()),
            _ => Err(format!(
                "Invalid parameter: Allowed parameters are integers or floats. {}",
                self
            )
            .into()),
        }
    }

    pub fn try_extract_bool(&self) -> Result<bool, RytmObjectError> {
        match self {
            Self::Int(v) => match v {
                0 => Ok(false),
                1 => Ok(true),
                _ => Err(
                    format!("Invalid parameter: {}. Must be followed by a 0 or 1.", self).into(),
                ),
            },
            _ => Err(format!("Invalid parameter: {}. Must be followed by a 0 or 1.", self).into()),
        }
    }
}
