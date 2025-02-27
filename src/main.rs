use tokio::runtime::Runtime;
use crate::weather::{WeatherFetcher};

mod cache;
mod weather;


const CACHE_FILE_PATH: &str = "./cache.txt";

fn main () {

    let weather_fetcher = WeatherFetcher::new();

    let data = Runtime::new().expect("Failed to create Runtime")
        .block_on(weather_fetcher.fetch_data());

    match data {
        Ok(data) => {
            println!("{:?}", data)
        },
        Err(err) => {
            eprintln!("{:?}", err)
        }
    }

}
