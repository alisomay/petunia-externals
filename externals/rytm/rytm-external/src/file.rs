use median::{file::FilePath, max_sys};
use std::{
    ffi::{c_char, CStr, CString},
    str::FromStr,
};

// Borrowed from the file.rs in the median crate
pub const MAX_PATH_CHARS: usize = 2048;

/// Extension trait for `FilePath` to add save dialog and absolute path functionality
pub trait FilePathExt {
    /// Present the user with the standard save file dialog.
    ///
    /// # Arguments
    /// * `default_name` - Default filename to show in the dialog
    /// * `types` - An optional list of file types to show in the dialog
    fn save_dialog(default_name: &str, types: Option<&Vec<max_sys::t_fourcc>>) -> Option<FilePath>;

    /// Get the full pathname using basic Max path formatting
    fn to_full_path(&self) -> Option<CString>;

    /// Get the absolute system path (e.g. POSIX style on Mac)
    /// This is preferred when passing paths to external libraries or system calls
    fn to_absolute_system_path(&self) -> Option<CString>;
}

impl FilePathExt for FilePath {
    fn save_dialog(default_name: &str, types: Option<&Vec<max_sys::t_fourcc>>) -> Option<FilePath> {
        let (types_ptr, len) = types.map_or((std::ptr::null(), 0), |t| (t.as_ptr(), t.len()));

        let mut file_name = [0 as c_char; MAX_PATH_CHARS];
        let mut vol: std::os::raw::c_short = 0;
        let mut typ: max_sys::t_fourcc = 0;

        // Copy default name into buffer
        let name = CString::new(default_name).ok()?;
        let name_bytes = name.as_bytes_with_nul();
        for (f, n) in file_name.iter_mut().zip(name_bytes) {
            *f = *n as i8;
        }

        unsafe {
            if max_sys::saveasdialog_extended(
                file_name.as_mut_ptr(),
                &mut vol,
                &mut typ,
                types_ptr.cast_mut(),
                len as _,
            ) == 0
            {
                Some(Self {
                    file_name: CStr::from_ptr(file_name.as_ptr()).to_owned(),
                    vol,
                    typ,
                })
            } else {
                None
            }
        }
    }

    fn to_full_path(&self) -> Option<CString> {
        let mut full_path = [0 as c_char; MAX_PATH_CHARS];
        unsafe {
            if max_sys::path_topathname(self.vol, self.file_name.as_ptr(), full_path.as_mut_ptr())
                == 0
            {
                Some(CStr::from_ptr(full_path.as_ptr()).to_owned())
            } else {
                None
            }
        }
    }

    fn to_absolute_system_path(&self) -> Option<CString> {
        let mut full_path = [0 as c_char; MAX_PATH_CHARS];
        unsafe {
            if max_sys::path_toabsolutesystempath(
                self.vol,
                self.file_name.as_ptr(),
                full_path.as_mut_ptr(),
            ) == 0
            {
                Some(CStr::from_ptr(full_path.as_ptr()).to_owned())
            } else {
                None
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RytmProjectFileType {
    Sysex,
    // This is just JSON in disguise
    Rytm,
}

impl FromStr for RytmProjectFileType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            ".sysex" | "sysex" => Ok(Self::Sysex),
            ".rytm" | "rytm" => Ok(Self::Rytm),
            _ => Err(()),
        }
    }
}
