use anyhow::Result;
use std::fs;
use std::path::Path;

// Re-export cmd_lib macros for use in other modules
pub use cmd_lib::run_cmd;

/// Append line to file if it doesn't exist
pub fn append_if_missing(path: impl AsRef<Path>, line: &str) -> Result<()> {
    use std::io::Write;

    let path = path.as_ref();
    let content = fs::read_to_string(path).unwrap_or_default();

    if !content.contains(line) {
        let mut file = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)?;
        file.write_all(format!("\n{}", line).as_bytes())?;
    }
    Ok(())
}
