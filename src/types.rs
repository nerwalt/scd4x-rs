//! Types for the SCD4x sensor.

/// SCD4X measurement data
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct SensorData {
    /// CO2 in units of PPM
    pub co2: u16,
    /// Temperature in units of °C
    pub temperature: f32,
    /// Relative humidity in units of %
    pub humidity: f32,
}

/// SCD4X raw measurement data
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct RawSensorData {
    pub co2: u16,
    pub temperature: u16,
    pub humidity: u16,
}

/// SCD4X Sensor variant
#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u16)]
pub enum SensorVariant {
    /// SCD40
    SCD40 = 0,
    /// SCD41
    SCD41 = 1,
    /// Unkown variant. Likely an error.
    Unknown(u16),
}

impl From<u16> for SensorVariant {
    fn from(value: u16) -> SensorVariant {
        match value {
            0 => Self::SCD40,
            1 => Self::SCD41,
            _ => Self::Unknown(value),
        }
    }
}

