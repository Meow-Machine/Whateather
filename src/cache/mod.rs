use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Deserialize, Serialize};
use crate::weather::UniversalTemperatureData;

const CACHE_BUFFER_SIZE: usize = 64;

pub struct Cache {
    cache_file_path: String,
    data : Option<CachedWeather>
}

#[derive(Serialize, Deserialize, Clone)]
struct CachedWeather {
    timestamp: u64, // unix time
    data: UniversalTemperatureData,
}
impl CachedWeather {
    pub fn new(data: &UniversalTemperatureData) -> Self {
        Self {
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            data: data.clone(),
        }
    }

    pub fn has_expired(&self, max_age: u64) -> bool {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        now - self.timestamp > max_age
    }
}



impl Cache {
    /// Creates a Cache instance
    pub fn new(file_path: &str) -> Self {
        Self {
            cache_file_path: String::from(file_path),
            data: None
        }
    }

    /// Writes data into the Cache file and saves it locally inside of the Cache Instance.
    /// Returns Error if Data failed to be written
    pub fn store_weather_data(&mut self, data: &UniversalTemperatureData) -> std::io::Result<()> {
        let cache_entry = CachedWeather::new(data);

        self.update_memory(cache_entry.clone());
        self.save_to_file(&cache_entry)
    }

    /// Loads the Cached data from the Cache File.
    /// Returns Error if file could not be Read
    /// Returns None if Data was too old
    pub fn load_weather_data(&self, max_age: u64) -> std::io::Result<Option<UniversalTemperatureData>> {
        let contents = self.read_from_file()?;
        let cached_data = Self::deserialize_cache(&contents)?;
        Ok(Self::check_cache_validity(cached_data, max_age))
    }

    /// Returns the locally cached data only if it is still valid
    pub fn retrieve_weather_data(&self, max_age: u64) -> Option<UniversalTemperatureData> {
        Self::check_cache_validity(self.data.clone()?, max_age)
    }

    fn save_to_file(&self, cache_entry: &CachedWeather) -> std::io::Result<()> {
        let json_data = serde_json::to_string(cache_entry)?;
        fs::write(&self.cache_file_path, json_data)?;
        Ok(())
    }

    fn read_from_file(&self) -> std::io::Result<String> {
        let mut cache_file = File::open(&self.cache_file_path)?;
        let mut contents = String::new();
        cache_file.read_to_string(&mut contents)?;
        Ok(contents)
    }

    fn deserialize_cache(contents: &str) -> std::io::Result<CachedWeather> {
        serde_json::from_str(contents).map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
    }

    fn update_memory(&mut self, cache_entry: CachedWeather) {
        self.data = Some(cache_entry);
    }

    fn check_cache_validity(cached_data: CachedWeather, max_age: u64) -> Option<UniversalTemperatureData> {
        if cached_data.has_expired(max_age) { return None; }
        Some(cached_data.data)
    }

}
