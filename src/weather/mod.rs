use reqwest::Client;
use serde::{Deserialize, Serialize};

mod meteo;

#[derive(Deserialize)]
pub struct GeoLocation {
    pub lat: f32,
    pub lon: f32,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct UniversalTemperatureData {
    temperature: f32,
    unit: TemperatureUnit,
}


#[derive(Serialize, Deserialize, Clone)]
pub enum TemperatureUnit {
    Celsius,
    Kelvin,
    Fahrenheit,
}

pub struct WeatherFetcher {
    client: Client,
}

impl WeatherFetcher {
    pub fn new() -> Self { WeatherFetcher { client: Client::new(), } }

    pub async fn fetch_data(&self)-> Result<UniversalTemperatureData, Box<dyn std::error::Error>> {
        let geolocation = self.fetch_location().await?;
        let meteo_data = self.fetch_weather(geolocation).await?;

        meteo_data.into_universal().map_err(|e| e.into())
    }

    async fn fetch_weather(&self, geo_location: GeoLocation) -> Result<meteo::MeteoWeatherResponse, reqwest::Error> {
        let url = format!(
            "https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&current=temperature_2m",
            geo_location.lat, geo_location.lon,
        );

        let response = self.client.get(url).send().await?;
        let json = response.json::<meteo::MeteoWeatherResponse>().await?;

        Ok(json)
    }

    async fn fetch_location(&self) -> Result<GeoLocation, reqwest::Error> {
        let url = "http://ip-api.com/json/";
        let response = self.client.get(url).send().await?;
        let json = response.json::<GeoLocation>().await?;
        Ok(json)
    }


}

impl UniversalTemperatureData {
    pub fn new(temperature: f32, unit: TemperatureUnit) -> Self {
        Self { temperature, unit }
    }

    pub fn print(&self) {
        println!("Temperature: {} °{}", self.temperature, match self.unit {
            TemperatureUnit::Celsius => "C",
            TemperatureUnit::Kelvin => "K",
            TemperatureUnit::Fahrenheit => "F",
        });
    }
}
