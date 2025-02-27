use colored::Colorize;
use serde::Deserialize;
use crate::{locator, util};

/// JSON response format from open-meteo
#[derive(Deserialize, Debug)]
struct WeatherResponse {
    current_units: UnitData,
    current: CurrentData,
}
#[derive(Deserialize, Debug)]
struct UnitData {
    temperature_2m: Option<String>,
}
#[derive(Deserialize, Debug)]
struct CurrentData {
    temperature_2m: Option<f32>,
}

/// Universal Temperature Data
#[derive(Debug)]
pub struct TemperatureData {
    temperature: f32,
    unit: TemperatureUnit,
}
#[derive(Debug)]
pub enum TemperatureUnit {
    CELSIUS,
    KELVIN,
    FAHRENHEIT,
}

impl TemperatureData {
    pub fn new(temperature: f32, unit: TemperatureUnit) -> Self {

        Self {
           temperature,
           unit,
        }
    }

    /// Converts the current temperature to a different one
    /// # Parameters:
    /// - new_unit : [`TemperatureUnit`] -> the Unit to be converted to
    pub fn convert(&mut self, new_unit: TemperatureUnit) {
        match self.unit {
            TemperatureUnit::CELSIUS => {
                match new_unit {
                    TemperatureUnit::KELVIN => {
                        self.temperature = self.temperature + 273.15f32
                    },
                    TemperatureUnit::FAHRENHEIT => {
                        self.temperature = self.temperature * ( 9f32 / 5f32 ) + 32.0f32;
                    },
                    _ => {},
                }
            },
            TemperatureUnit::KELVIN => {
                match new_unit {
                    TemperatureUnit::CELSIUS => {
                        self.temperature = self.temperature - 273.15f32
                    },
                    TemperatureUnit::FAHRENHEIT => {
                        self.temperature = self.temperature * (9f32 / 5f32) - 459.67f32;
                    },
                    _ => {},
                }
            },
            TemperatureUnit::FAHRENHEIT => {
                match new_unit {
                    TemperatureUnit::CELSIUS => {
                        self.temperature = (self.temperature - 32f32) * ( 5f32 / 9f32 );
                    },
                    TemperatureUnit::KELVIN => {
                        self.temperature = (self.temperature + 459.67f32) * ( 5f32 / 9f32 );
                    },
                    _ => {},
                }
            },
        }
        // We can now change the unit
        self.unit = new_unit;
    }
}

/// Retrieves the Temperature of a GeoLocation using the open-meteo API (Free for non-commercial)
/// # Parameters:
/// - location : &[`locator::GeoLocation`] -> Location in Question
/// - client : &[`reqwest::Client`] -> Internet client to be used
/// # Returns:
/// - [`TemperatureData`] -> Data gathered
pub async fn get_temperature(location: &locator::GeoLocation, client: &reqwest::Client) -> TemperatureData {
    // Create request url
    let url = format!(
        "https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&current=temperature_2m",
        location.lat,
        location.lon,
    );

    // request data
    let data = util::network::retrying_get(client, &*url)
        .await
        .json::<WeatherResponse>()
        .await
        .unwrap();

    let response_unit = data.current_units.temperature_2m.unwrap();

    let unit = if response_unit.eq("°C") {
        TemperatureUnit::CELSIUS
    } else {
        panic!("{} Temperature unit not Supported yet! {}", "[TODO]".green(), response_unit);
    };

    // Return gathered data
    TemperatureData::new(data.current.temperature_2m.unwrap(), unit)
}