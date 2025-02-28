use tokio::runtime::Runtime;
use crate::cache::Cache;
use crate::weather::{UniversalTemperatureData, WeatherFetcher};
use crate::weather::TemperatureUnit::Celsius;

mod cache;
mod weather;


const CACHE_FILE_PATH: &str = "./cache.txt";
const MAX_CACHE_AGE: u64 = 30*60; // 30 Minuten

fn main () {
    let mut cache = Cache::new(CACHE_FILE_PATH);
    let weather_fetcher = WeatherFetcher::new();

    let cached_data = match cache.load_weather_data(MAX_CACHE_AGE) {
        Ok(Some(data))=> data,
        Ok(None) => {
            eprintln!("Data was too old!");

            Runtime::new().expect("Failed to create Runtime")
                .block_on(weather_fetcher.fetch_data())
                .unwrap_or_else(|err| {
                    eprintln!("{}", err);
                    UniversalTemperatureData::new(0.0f32, Celsius)
                })
        },
        Err(e)=> {
            eprintln!("Cache file could not be read: {}", e);

            UniversalTemperatureData::new(0.0f32, Celsius)
        }
    };

    cache.store_weather_data(&cached_data).unwrap();


    cached_data.print();

}
