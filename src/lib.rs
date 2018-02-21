use std::error::Error;

/// Top level function for kleio-rs that transcribes data regarding Rust's history. When called it
/// will grab any data available and either update what's already there or grab from new sources if
/// they were added.
pub fn record() -> Result<(), Box<Error>> {
    Ok(())
}
