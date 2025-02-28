use serde::Deserialize;
use crate::weather::{TemperatureUnit, UniversalTemperatureData};

/// JSON response format from open-meteo
#[derive(Deserialize)]
pub struct MeteoWeatherResponse {
    current_units: UnitData,
    current: CurrentData,
}

#[derive(Deserialize)]
pub struct CurrentData {
    temperature_2m: Option<f32>,
}

#[derive(Deserialize)]
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