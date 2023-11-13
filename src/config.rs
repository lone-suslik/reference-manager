use dirs;
use serde::Deserialize;
use std::fs;
use std::io::Read;
use std::path::PathBuf;

/// Represents the application configuration.
///
/// This struct is used to deserialize the TOML configuration file. It has a single field, `base_path`, which is a `String`.
///
/// The `base_path` field represents the path to some directory on the file system. It is populated with the value of the `path` key from the TOML file.
///
#[derive(Deserialize)]
pub struct Config {
    pub base_path: String,
}

/// Loads the application configuration from a TOML file.
///
/// This function attempts to load the configuration from a TOML file located at one of the following paths, in order:
///
/// 1. `.refman.toml` in the current directory.
/// 2. `~/.config/refman/config.toml` in the user's home directory.
/// 3. `/etc/refman.toml` in the system's etc directory.
///
/// The function iterates over these paths and tries to open and read each file. If it successfully reads a file, it breaks out of the loop.
///
/// The TOML file is expected to contain a `path` key with a string value, which represents the path to some directory on the file system.
///
/// # Errors
///
/// If no configuration file is found at any of the paths, or if the file cannot be read, the function returns an error.
///
/// If the contents of the file cannot be parsed as a TOML document, or if the `base_path` key is not found or is not a string, the function returns an error.
///
/// # Returns
///
/// If successful, the function returns a `Config` struct containing the parsed `base_path` value from the TOML file.
///
pub fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    // Define the paths to the config files
    let paths = vec![
        PathBuf::from(".refman.toml"),
        dirs::home_dir()
            .unwrap()
            .join(".config")
            .join("refman")
            .join("config.toml"),
        PathBuf::from("/etc/refman.toml"),
    ];

    let mut contents = String::new();

    // Iterate over the paths and try to read the file
    for path in paths {
        if let Ok(mut file) = fs::File::open(&path) {
            file.read_to_string(&mut contents)?;
            break;
        }
    }

    // If no file was read, return an error
    if contents.is_empty() {
        return Err(From::from("No configuration file found"));
    }

    // Parse and return the configuration
    let config = toml::from_str(&contents)?;

    Ok(config)
}
