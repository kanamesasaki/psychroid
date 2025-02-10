/// Unit system for psychrometric calculations
/// <script src="https://cdn.jsdelivr.net/npm/mathjax@3/es5/tex-mml-chtml.js"></script>
///
/// # Variants
///
/// * `SI` - International System of Units:
///   - Temperature: \\(^\\circ\\mathrm{C}\\) (Celsius)
///   - Pressure: \\(\\mathrm{Pa}\\) (Pascal)
///   - Specific Volume: \\(\\mathrm{m^3 / kg_{da}}\\)
///   - Humidity Ratio: \\(\\mathrm{kg_w / kg_{da}}\\)
///   - Enthalpy: \\(\\mathrm{kJ / kg_{da}}\\)
///
/// * `IP` - Imperial System:
///   - Temperature: \\(^\\circ\\mathrm{F}\\) (Fahrenheit)
///   - Pressure: \\(\\mathrm{Psi}\\) (Pound per square inch)
///   - Specific Volume: \\(\\mathrm{ft^3 / lb_{da}}\\)
///   - Humidity Ratio: \\(\\mathrm{lb_w / lb_{da}}\\)
///   - Enthalpy: \\(\\mathrm{Btu / lb_{da}}\\)
///
/// # Example
/// ```
/// use psychroid::UnitSystem;
///
/// let unit = UnitSystem::SI;
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UnitSystem {
    SI,
    IP,
}

////////////////////////////////////////////////////////////////////////////////////////////////////////
// Constants
////////////////////////////////////////////////////////////////////////////////////////////////////////

// Zero degree Fahrenheit (°F) expressed as degree Rankine (°R).
pub const ZERO_FAHRENHEIT_AS_RANKINE: f64 = 459.67;

// Zero degree Celsius (°C) expressed as Kelvin (K).
pub const ZERO_CELSIUS_AS_KELVIN: f64 = 273.15;

// Universal gas constant for dry air (IP version) in ft∙lbf/lb_da/R.
pub const R_DA_IP: f64 = 53.350;

// Universal gas constant for dry air (SI version) in J/kg_da/K.
pub const R_DA_SI: f64 = 287.042;

// Ratio of molecular masses of water to dry air (non-dimension).
pub const MASS_RATIO_WATER_DRY_AIR: f64 = 0.621945;

// Invalid value.
pub const INVALID: f64 = -99999.0;

// Maximum number of iterations before exiting while loops.
pub const MAX_ITER_COUNT: usize = 100;

// Minimum acceptable humidity ratio used/returned by any functions.
// Any value above 0 or below the MIN_HUM_RATIO will be reset to this value.
pub const MIN_HUM_RATIO: f64 = 1e-7;

// Freezing point of water in Fahrenheit.
pub const FREEZING_POINT_WATER_IP: f64 = 32.0;

// Freezing point of water in Celsius.
pub const FREEZING_POINT_WATER_SI: f64 = 0.0;

// Triple point of water in Fahrenheit.
pub const TRIPLE_POINT_WATER_IP: f64 = 32.018;

// Triple point of water in Celsius.
pub const TRIPLE_POINT_WATER_SI: f64 = 0.01;

// Tolerance for SI and IP unit.
pub const TOLERANCE_SI: f64 = 0.001;
pub const TOLERANCE_IP: f64 = 0.001 * 9.0 / 5.0;

////////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversion between temperature units
////////////////////////////////////////////////////////////////////////////////////////////////////////

/// Convert Fahrenheit to Rankine
pub fn t_rankine_from_t_fahrenheit(t_f: f64) -> f64 {
    t_f + ZERO_FAHRENHEIT_AS_RANKINE
}

/// Convert Rankine to Fahrenheit
pub fn t_rankine_to_t_fahrenheit(t_r: f64) -> f64 {
    t_r - ZERO_FAHRENHEIT_AS_RANKINE
}

/// Convert Celsius to Kelvin
pub fn t_celsius_to_t_kelvin(t_c: f64) -> f64 {
    t_c + ZERO_CELSIUS_AS_KELVIN
}

/// Convert Kelvin to Celsius
pub fn t_kelvin_to_t_celsius(t_k: f64) -> f64 {
    t_k - ZERO_CELSIUS_AS_KELVIN
}

/// Convert Celsius to Fahrenheit
pub fn t_celsius_to_t_fahrenheit(t_c: f64) -> f64 {
    t_c * 1.8 + 32.0
}

/// Convert Fahrenheit to Celsius
pub fn t_fahrenheit_to_t_celsius(t_f: f64) -> f64 {
    (t_f - 32.0) / 1.8
}
