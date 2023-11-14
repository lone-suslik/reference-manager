use dirs;
use serde::Deserialize;
use std::fs::File;
use std::io::{self, Read};
use std::path::{Path, PathBuf};

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
/// This function attempts to load the configuration from a TOML file located at one of the following paths, in order provided by get_config_paths().
///
/// The TOML file is expected to contain a `path` key with a string value, which represents the path to some directory on the file system. Look more in the configuration docs.
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
    let paths = get_config_paths();
    for pathbuf in paths {
        if let Ok(contents) = read_file(pathbuf.as_path()) {
            if let Ok(config) = parse_config(&contents) {
                return Ok(config);
            }
        }
    }
    Err(Box::new(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        "No configuration file found",
    )))
}

/// This function hardcodes a `Vec<PathBuf>` containing the paths to the configuration file in the following order:
///
/// 1. `.refman.toml` in the current directory.
/// 2. `~/.config/refman/config.toml` in the user's home directory.
/// 3. `/etc/refman.toml` in the system's etc directory.
///
/// # Returns
///
/// A `Vec<PathBuf>` containing the paths to the configuration file.
///
fn get_config_paths() -> Vec<PathBuf> {
    vec![
        PathBuf::from(".refman.toml"),
        dirs::home_dir()
            .unwrap()
            .join(".config")
            .join("refman")
            .join("config.toml"),
        PathBuf::from("/etc/refman.toml"),
    ]
}

fn read_file(path: &Path) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn parse_config(contents: &str) -> Result<Config, toml::de::Error> {
    toml::from_str(contents)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_config_valid() {
        let contents = r#"base_path = "/tmp""#;
        let config = parse_config(contents).unwrap();
        assert_eq!(config.base_path, "/tmp");
    }

    #[test]
    fn test_parse_config_invalid_toml() {
        let contents = r#"base_path = /tmp"#; // Missing quotes around /tmp
        let result = parse_config(contents);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_config_missing_key() {
        let contents = r#"other_key = "/tmp""#;
        let result = parse_config(contents);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_config_wrong_type() {
        let contents = r#"base_path = 123"#; // base_path should be a string, not a number
        let result = parse_config(contents);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_config_empty() {
        let contents = "";
        let result = parse_config(contents);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_config_extra_keys() {
        let contents = r#"base_path = "/tmp", extra_key = "value""#;
        let config = parse_config(contents).unwrap();
        assert_eq!(config.base_path, "/tmp");
    }

    #[test]
    fn test_parse_config_whitespace() {
        let contents = r#"
            base_path = "/tmp"
        "#;
        let config = parse_config(contents).unwrap();
        assert_eq!(config.base_path, "/tmp");
    }

    #[test]
    fn test_parse_config_no_value() {
        let contents = r#"base_path = ""#;
        let config = parse_config(contents).unwrap();
        assert_eq!(config.base_path, "");
    }
}
