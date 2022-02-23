use crate::error::{Error, Result};
use parseit::reader;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

/// Cache data to store on the file system.
#[derive(Debug, Serialize, Deserialize)]
pub struct CacheData<Data> {
    /// Cache data.
    pub data: Data,
    /// Timestamp of the data.
    pub timestamp: u64,
}

impl<Data> CacheData<Data> {
    /// Constructs a new instance.
    pub fn new(data: Data, path: &Path) -> Result<Self> {
        Ok(Self {
            data,
            timestamp: Self::get_timestamp(path)?,
        })
    }

    /// Returns the last modification date of given file as UNIX timestamp.
    pub fn get_timestamp(path: &Path) -> Result<u64> {
        Ok(fs::metadata(&path)?
            .modified()?
            .duration_since(SystemTime::UNIX_EPOCH)?
            .as_secs())
    }
}

/// Cache manager for handling the R/W operations of labeled data.
#[derive(Debug)]
pub struct Cache {
    /// Cache directory.
    cache_dir: PathBuf,
}

impl Cache {
    /// Initializes the cache storage.
    pub fn init() -> Result<Self> {
        Ok(Self {
            cache_dir: dirs_next::cache_dir().ok_or_else(|| {
                Error::CacheError(String::from("cannot access the cache directory"))
            })?,
        })
    }

    /// Returns the path of given labeled data.
    fn get_cache_path(&self, label: &str) -> PathBuf {
        self.cache_dir
            .join(env!("CARGO_PKG_NAME"))
            .join(label)
            .with_extension("json")
    }

    /// Returns `true` if the labeled data is present in the cache.
    pub fn exists(&self, label: &str) -> bool {
        self.get_cache_path(label).exists()
    }

    /// Reads the given labeled data from the cache.
    pub fn read<T: DeserializeOwned>(&self, label: &str) -> Result<CacheData<T>> {
        let raw_data = reader::read_to_string(self.get_cache_path(label))?;
        Ok(serde_json::from_str(&raw_data)?)
    }

    /// Writes the given data to the cache.
    pub fn write<T: Serialize>(&self, data: CacheData<T>, label: &str) -> Result<()> {
        let cache_path = self.get_cache_path(label);
        if !cache_path.exists() {
            fs::create_dir_all(self.cache_dir.join(env!("CARGO_PKG_NAME")))?;
        };
        let mut file = File::create(&cache_path)?;
        file.write_all(serde_json::to_string(&data)?.as_bytes())?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_cache() -> Result<()> {
        let cache = Cache::init()?;
        let data = String::from("cache_test");
        let cache_data = CacheData::new(
            &data,
            &Path::new(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml"),
        )?;
        cache.write(cache_data, "data")?;
        assert!(cache.exists("data"));
        assert_eq!(data, cache.read::<String>("data")?.data);
        Ok(())
    }
}
