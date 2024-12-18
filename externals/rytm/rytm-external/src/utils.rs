use crate::{traits::Post, RytmExternal};
use camino::Utf8PathBuf;
use homedir::my_home;
use median::object::MaxObj;
use tracing::{debug, instrument, warn};

impl RytmExternal {
    #[instrument(
        skip(self),
        fields(
            path_candidate,
            home_dir,
            home_dir_str,
            path_without_tilde,
            expanded_path
        )
    )]
    pub fn make_utf8_path_buf_respect_tilde(&self, path_candidate: &str) -> Utf8PathBuf {
        let span = tracing::Span::current();
        if path_candidate.starts_with('~') {
            // Attempt to get the user's home directory
            if let Some(home_dir) = my_home().ok().flatten() {
                span.record("home_dir", home_dir.to_string_lossy().to_string());

                if let Some(home_dir_str) = home_dir.to_str() {
                    span.record("home_dir_str", home_dir_str);
                    // Replace the leading '~' with the home directory

                    let path_without_tilde = path_candidate.trim_start_matches('~');
                    span.record("path_without_tilde", path_without_tilde);

                    let expanded_path = format!("{home_dir_str}{path_without_tilde}");
                    span.record("expanded_path", &expanded_path);

                    debug!("Expanded path with home directory");
                    return Utf8PathBuf::from(expanded_path);
                }
            }

            let warning = "Failed to get home directory, the path will be returned as is";
            warn!("{}", warning);
            warning.obj_warn(self.max_obj());
            self.send_status_warning();

            // If we can't get the home directory, return the original path
            return Utf8PathBuf::from(path_candidate);
        }

        debug!("Path does not start with '~', returning as is");
        Utf8PathBuf::from(path_candidate)
    }
}
