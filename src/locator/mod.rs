use serde::Deserialize;
use crate::util;

/// Locates the user using their IP through the ip-api (free for non commercial)
/// # Parameters
/// - client : &[`reqwest::Client`] -> Internet client to be used
/// # Returns
/// - Location: [`GeoLocation`] -> Location of the user
pub async fn locate_via_ip(client : &reqwest::Client) -> GeoLocation {
    let url = "http://ip-api.com/json";

    util::network::retrying_get(client, url)
        .await
        .json::<GeoLocation>()
        .await
        .unwrap()
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