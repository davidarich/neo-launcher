use dirs::data_local_dir;

/// Per-user data directory for the launcher:
/// `~/.local/share/neo-launcher` on Linux, `%LOCALAPPDATA%/neo-launcher` on Windows.
pub fn data_dir() -> Option<std::path::PathBuf> {
    data_local_dir().map(|dir| dir.join("neo-launcher"))
}

/// Full path to the cache file, e.g. `~/.local/share/neo-launcher/cache.yaml`.
pub fn cache_path() -> Option<std::path::PathBuf> {
    data_dir().map(|dir| dir.join("cache.yaml"))
}

/// Directory containing the launcher's scripts.
pub fn scripts_dir() -> Option<std::path::PathBuf> {
    data_dir().map(|dir| dir.join("scripts"))
}

/// Full path to a launcher script by name.
pub fn script_path(script_name: String) -> Option<std::path::PathBuf> {
    scripts_dir().map(|dir| dir.join(script_name + ".bat"))
}

/// Writes a script file to disk.
pub fn write_script(script_path: std::path::PathBuf, contents: String) -> std::io::Result<()> {
    std::fs::write(script_path, contents)?;

    Ok(())
}
