use median::max_sys;
use rytm_object::error::RytmObjectError;
// use rytm_rs::error::RytmError;

// #[derive(thiserror::Error, Debug)]
// #[non_exhaustive]
// pub enum QueryError {
//     #[error("Invalid query selector: Query selector must be one of pattern, pattern_wb, kit, kit_wb, global, global_wb, sound, sound_wb or settings.")]
//     InvalidSelector,
//     #[error("Invalid query format: The right format should be, <selector> [<index>]. Example: query pattern_wb or query pattern 0")]
//     InvalidFormat,
//     #[error("Invalid index type: Index must be an integer.")]
//     InvalidIndexType,
// }

// #[derive(thiserror::Error, Debug)]
// #[non_exhaustive]
// pub enum SendError {
//     #[error("Invalid send format: The right format should be, <selector> [<index>]. Example: send pattern_wb or send pattern 0.")]
//     InvalidFormat,
// }

// #[derive(thiserror::Error, Debug)]
// #[non_exhaustive]
// pub enum GetError {
//     #[error("Invalid getter format: {0}")]
//     InvalidFormat(String),

//     #[error(
//         "Invalid getter format: \"{0}\".
//         Accepted formats:
//             get sound <index> <identifier> [<parameter>]
//             get sound <index> <enum> [<parameter>]"
//     )]
//     InvalidSoundGetterFormat(String),

//     #[error(
//         "Invalid getter format: \"{0}\".
//         Accepted formats:
//             get settings <identifier> [<parameter>]
//             get settings <enum>"
//     )]
//     InvalidSettingsGetterFormat(String),

//     #[error(
//         "Invalid getter format: \"{0}\".
//         Accepted formats:
//             get global <index> <identifier> [<parameter>]
//             get global <index> <enum>"
//     )]
//     InvalidGlobalGetterFormat(String),
//     #[error(
//         "Invalid getter format: \"{0}\".
//         Accepted formats:
//             get global_wb <identifier> [<parameter>]
//             get global_wb <enum>"
//     )]
//     InvalidGlobalWbGetterFormat(String),

//     #[error(
//         "Invalid getter format: \"{0}\".
//         Accepted formats:
//             get kit <index> <identifier>
//             get kit <index> <enum>
//             get kit <index> <element> <element-index>
//             get kit <index> sound <sound-index> <identifier> [<parameter>]
//             get kit <index> sound <sound-index> <enum> [<parameter>]"
//     )]
//     InvalidKitGetterFormat(String),
//     #[error(
//         "Invalid getter format: \"{0}\".
//         Accepted formats:
//             get kit_wb <identifier>
//             get kit_wb <enum>
//             get kit_wb <element> <element-index>
//             get kit_wb sound <sound-index> <identifier> [<parameter>]
//             get kit_wb sound <sound-index> <enum> [<parameter>]"
//     )]
//     InvalidKitWbGetterFormat(String),

//     #[error(
//         "Invalid getter format: \"{0}\".
//         Accepted formats:
//             get pattern <index> <identifier>
//             get pattern <index> <enum>
//             get pattern <index> <track-index> <identifier>
//             get pattern <index> <track-index> <enum>
//             get pattern <index> <track-index> <trig-index> <identifier>
//             get pattern <index> <track-index> <trig-index> <enum>
//             get pattern <index> <track-index> <trig-index> plockget <identifier>
//             get pattern <index> <track-index> <trig-index> plockget <enum>"
//     )]
//     InvalidPatternGetterFormat(String),

//     #[error(
//         "Invalid getter format: \"{0}\".
//         Accepted formats:
//             get pattern_wb <identifier>
//             get pattern_wb <enum>
//             get pattern_wb <track-index> <identifier>
//             get pattern_wb <track-index> <enum>
//             get pattern_wb <track-index> <trig-index> <identifier>
//             get pattern_wb <track-index> <trig-index> <enum>
//             get pattern_wb <track-index> <trig-index> plockget <identifier>
//             get pattern_wb <track-index> <trig-index> plockget <enum>"
//     )]
//     InvalidPatternWbGetterFormat(String),
// }

// #[derive(thiserror::Error, Debug)]
// #[non_exhaustive]
// pub enum SetError {
//     #[error("Invalid setter format: {0}")]
//     InvalidFormat(String),

//     #[error(
//         "Invalid setter format: \"{0}\".
//         Accepted formats:
//             set sound <index> <identifier> <parameter> [<parameter>]
//             set sound <index> <enum> [<parameter>]"
//     )]
//     InvalidSoundSetterFormat(String),

//     #[error(
//         "Invalid setter format: \"{0}\".
//         Accepted formats:
//             set settings <identifier> <parameter>,
//             set settings <enum>"
//     )]
//     InvalidSettingsSetterFormat(String),

//     #[error(
//         "Invalid setter format: \"{0}\".
//         Accepted formats:
//             set global <index> <identifier> <parameter>
//             set global <index> <enum> [<parameter>]"
//     )]
//     InvalidGlobalSetterFormat(String),
//     #[error(
//         "Invalid setter format: \"{0}\".
//         Accepted formats:
//             set global_wb <identifier> <parameter>
//             set global_wb <enum> [<parameter>]"
//     )]
//     InvalidGlobalWbSetterFormat(String),

//     #[error(
//         "Invalid setter format: \"{0}\".
//         Accepted formats:
//             set kit <index> <identifier> <parameter>
//             set kit <index> <enum>
//             set kit <index> <element> <element-index> <enum>
//             set kit <index> sound <sound-index> <identifier> <parameter> [<parameter>]
//             set kit <index> sound <sound-index> <enum> [<parameter>]"
//     )]
//     InvalidKitSetterFormat(String),
//     #[error(
//         "Invalid setter format: \"{0}\".
//         Accepted formats:
//             set kit <identifier> <parameter>
//             set kit <enum>
//             set kit <element> <element-index> <enum>
//             set kit sound <sound-index> <identifier> <parameter> [<parameter>]
//             set kit sound <sound-index> <enum> [<parameter>]"
//     )]
//     InvalidKitWbSetterFormat(String),

//     #[error(
//         "Invalid setter format: \"{0}\".
//         Accepted formats:
//             set pattern <index> <identifier> <parameter>
//             set pattern <index> <enum>
//             set pattern <index> <track-index> <identifier> <parameter>
//             set pattern <index> <track-index> <enum>
//             set pattern <index> <track-index> <trig-index> <identifier> <parameter>
//             set pattern <index> <track-index> <trig-index> <enum>
//             set pattern <index> <track-index> <trig-index> plockset <identifier> <parameter>
//             set pattern <index> <track-index> <trig-index> plockset <enum>
//             set pattern <index> <track-index> <trig-index> plockclear <identifier>
//             set pattern <index> <track-index> <trig-index> plockclear <enum>"
//     )]
//     InvalidPatternSetterFormat(String),

//     #[error(
//         "Invalid setter format: \"{0}\".
//         Accepted formats:
//             get pattern_wb <identifier> <parameter>
//             get pattern_wb <enum>
//             get pattern_wb <track-index> <identifier> <parameter>
//             get pattern_wb <track-index> <enum>
//             get pattern_wb <track-index> <trig-index> <identifier> <parameter>
//             get pattern_wb <track-index> <trig-index> <enum>
//             get pattern_wb <track-index> <trig-index> plockset <identifier> <parameter>
//             get pattern_wb <track-index> <trig-index> plockset <enum>
//             set pattern_wb <track-index> <trig-index> plockclear <identifier>
//             set pattern_wb <track-index> <trig-index> plockclear <enum>"
//     )]
//     InvalidPatternWbSetterFormat(String),
// }

// #[derive(thiserror::Error, Debug)]
// #[non_exhaustive]
// pub enum EnumError {
//     #[error("Invalid enum type: {0}")]
//     InvalidEnumType(String),
// }

// #[derive(thiserror::Error, Debug)]
// #[non_exhaustive]
// pub enum IdentifierError {
//     #[error("Invalid identifier type: {0}")]
//     InvalidType(String),
//     #[error("Invalid parameter following {1}: {0}")]
//     InvalidParameter(String, String),
// }

/// Wrapper error type for all rytm errors.
#[derive(thiserror::Error, Debug)]
#[non_exhaustive]
pub enum RytmExternalError {
    #[error("{0}")]
    Custom(String),
    // #[error(transparent)]
    // Query(#[from] QueryError),
    // #[error(transparent)]
    // Send(#[from] SendError),
    // #[error(transparent)]
    // Get(#[from] GetError),
    // #[error(transparent)]
    // Set(#[from] SetError),
    // #[error(transparent)]
    // Enum(#[from] EnumError),
    // #[error(transparent)]
    // Identifier(#[from] IdentifierError),
    // #[error(transparent)]
    // RytmSdk(#[from] RytmError),
    #[error(transparent)]
    StringConversionError(#[from] std::str::Utf8Error),
    #[error(transparent)]
    RytmObject(#[from] RytmObjectError),

    #[error("Not implemented, if you need this api open an issue in https://github.com/alisomay/rytm-external.")]
    NotYetImplemented,
}

// impl From<rytm_rs::error::ConversionError> for RytmExternalError {
//     fn from(err: rytm_rs::error::ConversionError) -> Self {
//         Self::RytmSdk(err.into())
//     }
// }

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
            // Self::Query(err) => median::object::error(obj, err.to_string()),
            // Self::Send(err) => median::object::error(obj, err.to_string()),
            // Self::Get(err) => median::object::error(obj, err.to_string()),
            // Self::Set(err) => median::object::error(obj, err.to_string()),
            // Self::Enum(err) => median::object::error(obj, err.to_string()),
            // Self::Identifier(err) => median::object::error(obj, err.to_string()),
            // Self::RytmSdk(err) => median::object::error(obj, err.to_string()),
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
            // Self::Query(err) => median::error(err.to_string()),
            // Self::Send(err) => median::error(err.to_string()),
            // Self::Get(err) => median::error(err.to_string()),
            // Self::Set(err) => median::error(err.to_string()),
            // Self::Enum(err) => median::error(err.to_string()),
            // Self::Identifier(err) => median::error(err.to_string()),
            // Self::RytmSdk(err) => median::error(err.to_string()),
            Self::StringConversionError(err) => median::error(err.to_string()),
            Self::RytmObject(err) => err.post(),
            Self::NotYetImplemented => median::error("Not yet implemented.".to_string()),
        }
    }
}
