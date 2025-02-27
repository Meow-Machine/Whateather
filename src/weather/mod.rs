use reqwest::Client;
use serde::Deserialize;

mod meteo {
    use serde::Deserialize;
    use crate::weather::{TemperatureUnit, UniversalTemperatureData};

    /// JSON response format from open-meteo
    #[derive(Deserialize, Debug)]
    pub struct MeteoWeatherResponse {
        current_units: UnitData,
        current: CurrentData,
    }

    #[derive(Deserialize, Debug)]
    pub struct CurrentData {
        temperature_2m: Option<f32>,
    }

    #[derive(Deserialize, Debug)]
    pub struct UnitData {
        temperature_2m: Option<String>,
    }

    impl MeteoWeatherResponse {
        pub fn into_universal(self) -> Result<UniversalTemperatureData,String> {

            let unit = self.current_units.into_temperature_unit()?;
            let temperature = self.current.into_f32()?;

            Ok(UniversalTemperatureData::new(temperature, unit))
        }
    }

    impl UnitData {
        pub fn into_temperature_unit(self) -> Result<TemperatureUnit, String> {
            let unit = self.temperature_2m.as_ref().ok_or_else(|| "Meteo did not Provide Unit-Data".to_string())?.as_str();
            Self::convert_unit(unit)
        }
        fn convert_unit(unit: &str) -> Result<TemperatureUnit, String> {
            match unit {
                "°C" => Ok(TemperatureUnit::Celsius),
                _ => {Err(format!("Unit translation not possible for {}", unit))}
            }
        }

    }
    impl CurrentData {
        pub fn into_f32(self) -> Result<f32, String> {
            self.temperature_2m.ok_or_else(|| "Meteo did not Provide Temperature-Data".to_string())
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct GeoLocation {
    pub lat: f32,
    pub lon: f32,
}
#[derive(Debug)]
pub struct UniversalTemperatureData {
    temperature: f32,
    unit: TemperatureUnit,
}
#[derive(Debug)]
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
}
