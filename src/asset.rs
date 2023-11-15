use crate::hash::hash_big_file;
use serde::de::DeserializeSeed;
use serde::{Deserialize, Deserializer, Serialize};
use std::fs::read_to_string;
use std::path::PathBuf;

#[derive(Debug, Serialize)]
pub struct ReferenceAsset {
    #[serde(skip_serializing)]
    pub origin: PathBuf,
    pub name: String,
    #[serde(skip_serializing)]
    pub asset_hash: u64,
}

impl ReferenceAsset {
    pub fn from_json(path: &PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        let json = read_to_string(&path)?;
        let seed = ReferenceAssetSeed {
            origin: path.clone(),
        };
        let asset: ReferenceAsset = from_str_seed(&json, seed)?;
        Ok(asset)
    }
}

fn from_str_seed<'a, S>(s: &'a str, seed: S) -> Result<ReferenceAsset, serde_json::Error>
where
    S: DeserializeSeed<'a, Value = ReferenceAsset>,
{
    let mut deserializer = serde_json::Deserializer::from_str(s);
    seed.deserialize(&mut deserializer)
}

struct ReferenceAssetSeed {
    pub origin: PathBuf,
}

impl<'de> DeserializeSeed<'de> for ReferenceAssetSeed {
    type Value = ReferenceAsset;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct TempReferenceAsset {
            name: String,
        }

        let temp_asset = TempReferenceAsset::deserialize(deserializer)?;

        let asset_hash = match hash_big_file(&self.origin.to_string_lossy()) {
            Ok(hash) => hash,
            Err(e) => return Err(serde::de::Error::custom(e.to_string())),
        };

        Ok(ReferenceAsset {
            origin: self.origin,
            name: temp_asset.name,
            asset_hash,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use uuid::Uuid;

    #[test]
    fn test_from_json_valid_path() {
        let dir = Uuid::new_v4().to_string();
        let path = PathBuf::from(format!(
            "{}/resources/test-area/{}/referense-info.json",
            env!("CARGO_MANIFEST_DIR"),
            dir
        ));
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        let mut file = File::create(&path).unwrap();
        writeln!(file, r#"{{"name": "test"}}"#).unwrap();

        let asset = ReferenceAsset::from_json(&path).unwrap();
        assert_eq!(asset.origin, path);
        assert_eq!(asset.name, "test");

        std::fs::remove_dir_all(path.parent().unwrap()).unwrap();
    }

    #[test]
    fn test_from_json_invalid_path() {
        let dir = Uuid::new_v4().to_string();
        let path = PathBuf::from(format!(
            "{}/resources/test-area/{}/non_existent.json",
            env!("CARGO_MANIFEST_DIR"),
            dir
        ));
        let result = ReferenceAsset::from_json(&path);
        assert!(result.is_err());
    }

    #[test]
    fn test_from_json_invalid_json() {
        let dir = Uuid::new_v4().to_string();
        let path = PathBuf::from(format!(
            "{}/resources/test-area/{}/referense-info.json",
            env!("CARGO_MANIFEST_DIR"),
            dir
        ));
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        let mut file = File::create(&path).unwrap();
        writeln!(file, r#"{{"name": 123}}"#).unwrap();

        let result = ReferenceAsset::from_json(&path);
        assert!(result.is_err());

        std::fs::remove_dir_all(path.parent().unwrap()).unwrap();
    }

    #[test]
    fn test_from_json_empty_file() {
        let dir = Uuid::new_v4().to_string();
        let path = PathBuf::from(format!(
            "{}/resources/test-area/{}/referense-info.json",
            env!("CARGO_MANIFEST_DIR"),
            dir
        ));
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        File::create(&path).unwrap();

        let result = ReferenceAsset::from_json(&path);
        assert!(result.is_err());

        std::fs::remove_dir_all(path.parent().unwrap()).unwrap();
    }

    #[test]
    fn test_from_json_non_json_file() {
        let dir = Uuid::new_v4().to_string();
        let path = PathBuf::from(format!(
            "{}/resources/test-area/{}/test.txt",
            env!("CARGO_MANIFEST_DIR"),
            dir
        ));
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        let mut file = File::create(&path).unwrap();
        writeln!(file, "This is not a JSON file.").unwrap();

        let result = ReferenceAsset::from_json(&path);
        assert!(result.is_err());

        std::fs::remove_dir_all(path.parent().unwrap()).unwrap();
    }
}
