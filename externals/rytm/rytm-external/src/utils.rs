use camino::Utf8PathBuf;
use tilde_expand::tilde_expand;

pub fn make_utf8_path_buf_respect_tilde(path_candidate: &str) -> Utf8PathBuf {
    Utf8PathBuf::from(
        String::from_utf8_lossy(&tilde_expand(path_candidate.as_bytes())).into_owned(),
    )
}
