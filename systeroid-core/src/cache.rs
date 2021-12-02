use crate::error::{Error, Result};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;
use systeroid_parser::reader;

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
    pub fn read<Data: DeserializeOwned>(&self, label: &str) -> Result<Data> {
        let raw_data = reader::read_to_string(self.get_cache_path(label))?;
        Ok(serde_json::from_str(&raw_data)?)
    }

    /// Writes the given data to the cache.
    pub fn write<Data: ?Sized + Serialize>(&self, data: &Data, label: &str) -> Result<()> {
        let cache_path = self.get_cache_path(label);
        if !cache_path.exists() {
            fs::create_dir_all(self.cache_dir.join(env!("CARGO_PKG_NAME")))?;
        };
        let mut file = File::create(&cache_path)?;
        file.write_all(serde_json::to_string(data)?.as_bytes())?;
        Ok(())
    }
}
