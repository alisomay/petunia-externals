use median::max_sys;
use rytm_object::error::RytmObjectError;

/// Wrapper error type for all rytm errors.
#[derive(thiserror::Error, Debug)]
#[non_exhaustive]
pub enum RytmExternalError {
    #[error("{0}")]
    Custom(String),
    #[error(transparent)]
    StringConversionError(#[from] std::str::Utf8Error),
    #[error(transparent)]
    RytmObject(#[from] RytmObjectError),

    #[error("Not implemented, if you need this api open an issue in https://github.com/alisomay/rytm-external.")]
    NotYetImplemented,
}

impl From<&str> for RytmExternalError {
    fn from(s: &str) -> Self {
        Self::Custom(s.to_string())
    }
}

impl From<String> for RytmExternalError {
    fn from(s: String) -> Self {
        Self::Custom(s)
    }
}

impl RytmExternalError {
    pub fn obj_post(&self, obj: *mut max_sys::t_object) {
        match self {
            Self::Custom(err) => median::object::error(obj, err.to_string()),
            Self::StringConversionError(err) => median::object::error(obj, err.to_string()),
            Self::RytmObject(err) => err.obj_post(obj),
            Self::NotYetImplemented => {
                median::object::error(obj, "Not yet implemented.".to_string());
            }
        }
    }

    pub fn post(&self) {
        match self {
            Self::Custom(err) => median::error(err.to_string()),
            Self::StringConversionError(err) => median::error(err.to_string()),
            Self::RytmObject(err) => err.post(),
            Self::NotYetImplemented => median::error("Not yet implemented.".to_string()),
        }
    }
}
