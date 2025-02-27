use reqwest::{Client, Response};
use crate::weather::MeteoWeatherResponse;
use tokio::runtime::Runtime;

mod cache;
mod weather;

mod util;

const CACHE_FILE_PATH: &str = "./cache.txt";


fn main () {
    try_cache()
}

fn try_fetch() {
    let latitude = 52.279911;
    let longitude = 8.0471719;

    let url = format!(
        "https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&current=temperature_2m",
        latitude,
        longitude
    );
    let client = Client::new();

    let response = get_new_data(&client, &*url);
}
fn get_new_data(client: &Client, url: &str) {

    let runtime = Runtime::new().unwrap();

    let response = runtime.block_on(async {
        let response = util::network::retrying_get(client, url)
            .await
            .unwrap();

        response.json::<MeteoWeatherResponse>()
            .await
            .unwrap()
    });

    println!("{:#?}", response);
}

fn try_cache() {
    let mut cache = cache::Cache::create(CACHE_FILE_PATH);
    cache.read_cache_file();
    cache.print_data();

    cache.write_data_to_cache(b"HalloWeltchen");
    cache.print_data();
}