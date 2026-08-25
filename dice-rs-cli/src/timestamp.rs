/// Simple timestamp without external chrono dependency.
pub(crate) fn chrono_like_timestamp() -> String {
    use std::time::SystemTime;
    use std::time::UNIX_EPOCH;
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default();
    let secs = now.as_secs();
    let h = ((secs / 3600) % 24) as u8;
    let m = ((secs / 60) % 60) as u8;
    let s = (secs % 60) as u8;
    format!("{h:02}:{m:02}:{s:02}")
}
