use serde::Deserialize;

/// JSON response format from open-meteo
#[derive(Deserialize, Debug)]
struct WeatherResponse {
    current_units: UnitData,
    current: CurrentData,
}

#[derive(Deserialize, Debug)]
struct CurrentData {
    temperature_2m: Option<f32>,
}

#[derive(Deserialize, Debug)]
struct UnitData {
    temperature_2m: Option<String>,
}