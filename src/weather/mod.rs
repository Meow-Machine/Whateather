use serde::Deserialize;

/// JSON response format from open-meteo
#[derive(Deserialize, Debug)]
pub struct MeteoWeatherResponse {
    pub current_units: UnitData,
    pub current: CurrentData,
}

#[derive(Deserialize, Debug)]
pub struct CurrentData {
    pub temperature_2m: Option<f32>,
}

#[derive(Deserialize, Debug)]
pub struct UnitData {
    pub temperature_2m: Option<String>,
}


// pub struct UniversalTemperatureData {
//     temperature: f32,
//     unit: TemperatureUnit,
// }
// pub enum TemperatureUnit {
//     Celsius,
//     Kelvin,
//     Fahrenheit,
// }
