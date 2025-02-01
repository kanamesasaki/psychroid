/// Enum for Unit Systems
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UnitSystem {
    IP,
    SI,
}

/// Constants
const ZERO_FAHRENHEIT_AS_RANKINE: f64 = 459.67;
const ZERO_CELSIUS_AS_KELVIN: f64 = 273.15;
const R_DA_IP: f64 = 53.350;
const R_DA_SI: f64 = 287.042;
const INVALID: f64 = -99999.0;
const MAX_ITER_COUNT: usize = 100;
const MIN_HUM_RATIO: f64 = 1e-7;
const FREEZING_POINT_WATER_IP: f64 = 32.0;
const FREEZING_POINT_WATER_SI: f64 = 0.0;
const TRIPLE_POINT_WATER_IP: f64 = 32.018;
const TRIPLE_POINT_WATER_SI: f64 = 0.01;
