use serde::Deserialize;
use crate::util;

/// Locates the system using either the IP or GPS-Module
/// # Parameters:
/// - locator : &[`std::str`] -> locator to be used
/// - client : &[`reqwest::Client`] -> Internet client to be used
/// # Returns:
/// [`GeoLocation`] Geo location of the system in a standardized Format
pub async fn locate(locator: &str, client: &reqwest::Client) -> GeoLocation {
    let url = match locator {
        "IP" => "http://ip-api.com/json",
        "GPS" => todo!("Locator is not implemented yet!"),
        _ => panic!("Unknown locator!")
    };

    let response = util::network::retrying_get(client, url).await;

    match locator {
        "IP" => response.json::<GeoLocation>().await.unwrap(),
        _ => panic!("Coudln't parse location data")
    }

}

/// A simple location representation using latitude and longitude
/// # Fields:
/// - latitude: [`std::f32`]
/// - longitude [`std::f32`]
#[derive(Deserialize, Debug)]
pub struct GeoLocation {
    pub lat: f32,
    pub lon: f32,
}