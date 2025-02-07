/// Enum for Unit Systems
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UnitSystem {
    SI,
    IP,
}

////////////////////////////////////////////////////////////////////////////////////////////////////////
// Constants
////////////////////////////////////////////////////////////////////////////////////////////////////////

// Zero degree Fahrenheit (°F) expressed as degree Rankine (°R).
const ZERO_FAHRENHEIT_AS_RANKINE: f64 = 459.67;

// Zero degree Celsius (°C) expressed as Kelvin (K).
const ZERO_CELSIUS_AS_KELVIN: f64 = 273.15;

// Universal gas constant for dry air (IP version) in ft∙lbf/lb_da/R.
const R_DA_IP: f64 = 53.350;

// Universal gas constant for dry air (SI version) in J/kg_da/K.
const R_DA_SI: f64 = 287.042;

// Ratio of molecular masses of water to dry air (non-dimension).
const MASS_RATIO_WATER_DRY_AIR: f64 = 0.62198;

// Invalid value.
const INVALID: f64 = -99999.0;

// Maximum number of iterations before exiting while loops.
const MAX_ITER_COUNT: usize = 100;

// Minimum acceptable humidity ratio used/returned by any functions.
// Any value above 0 or below the MIN_HUM_RATIO will be reset to this value.
const MIN_HUM_RATIO: f64 = 1e-7;

// Freezing point of water in Fahrenheit.
const FREEZING_POINT_WATER_IP: f64 = 32.0;

// Freezing point of water in Celsius.
const FREEZING_POINT_WATER_SI: f64 = 0.0;

// Triple point of water in Fahrenheit.
const TRIPLE_POINT_WATER_IP: f64 = 32.018;

// Triple point of water in Celsius.
const TRIPLE_POINT_WATER_SI: f64 = 0.01;

// Tolerance for SI and IP unit.
const TOLERANCE_SI: f64 = 0.001;
const TOLERANCE_IP: f64 = 0.001 * 9.0 / 5.0;

////////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversion between temperature units
////////////////////////////////////////////////////////////////////////////////////////////////////////

/// Convert Fahrenheit to Rankine
pub fn t_rankine_from_t_fahrenheit(t_f: f64) -> f64 {
    t_f + ZERO_FAHRENHEIT_AS_RANKINE
}

/// Convert Rankine to Fahrenheit
pub fn t_fahrenheit_from_t_rankine(t_r: f64) -> f64 {
    t_r - ZERO_FAHRENHEIT_AS_RANKINE
}

/// Convert Celsius to Kelvin
pub fn t_kelvin_from_t_celsius(t_c: f64) -> f64 {
    t_c + ZERO_CELSIUS_AS_KELVIN
}

/// Convert Kelvin to Celsius
pub fn t_celsius_from_t_kelvin(t_k: f64) -> f64 {
    t_k - ZERO_CELSIUS_AS_KELVIN
}

////////////////////////////////////////////////////////////////////////////////////////////////////////
// Moist Air
////////////////////////////////////////////////////////////////////////////////////////////////////////

/// Contains all calculated psychrometric values
#[derive(Debug)]
pub struct MoistAir {
    pub t_dry_bulb: f64,     // °C (SI) or °F (IP)
    pub humidity_ratio: f64, // kg_H₂O/kg_Air (SI) or lb_H₂O/lb_Air (IP)
    pub pressure: f64,       // Pa (SI) or Psi (IP)
    unit: UnitSystem,        // SI or IP
}

impl Default for MoistAir {
    fn default() -> Self {
        MoistAir {
            t_dry_bulb: 20.0,
            humidity_ratio: 0.00735,
            pressure: 101325.0,
            unit: UnitSystem::SI,
        }
    }
}

impl MoistAir {
    /// Create a new instance of MoistAir
    pub fn new(t_dry_bulb: f64, humidity_ratio: f64, pressure: f64, unit: UnitSystem) -> Self {
        MoistAir {
            t_dry_bulb,
            humidity_ratio,
            pressure,
            unit,
        }
    }

    /// Init from wet bulb temperature
    pub fn from_t_wet_bulb(
        t_dry_bulb: f64,
        t_wet_bulb: f64,
        pressure: f64,
        unit: UnitSystem,
    ) -> Self {
        let humidity_ratio = humidity_ratio_from_t_wet_bulb(t_dry_bulb, t_wet_bulb, pressure, unit);
        MoistAir {
            t_dry_bulb,
            humidity_ratio,
            pressure,
            unit,
        }
    }

    /// Creates a new MoistAir instance from dry-bulb temperature, relative humidity, and pressure
    /// ASHRAE Handbook - Fundamentals (2017) Ch. 1-8 SITUATION 3.
    ///
    /// # Arguments
    /// * `t_dry_bulb` - Dry-bulb temperature [°C] (SI) or [°F] (IP)
    /// * `relative_humidity` - Relative humidity [0-1]
    /// * `pressure` - Atmospheric pressure [Pa] (SI) or [Psi] (IP)
    /// * `unit` - Unit system (SI or IP)
    ///
    /// # Returns
    /// Returns a new MoistAir instance
    ///
    /// # Example
    /// ```
    /// use psychroid::{MoistAir, UnitSystem};
    ///
    /// let air = MoistAir::from_relative_humidity(
    ///     25.0,     // 25°C
    ///     0.5,      // 50% RH
    ///     101325.0, // Standard atmospheric pressure
    ///     UnitSystem::SI
    /// );
    /// ```
    pub fn from_relative_humidity(
        t_dry_bulb: f64,
        relative_humidity: f64,
        pressure: f64,
        unit: UnitSystem,
    ) -> Self {
        let humidity_ratio =
            humidity_ratio_from_relative_humidity(t_dry_bulb, relative_humidity, pressure, unit);
        MoistAir {
            t_dry_bulb,
            humidity_ratio,
            pressure,
            unit,
        }
    }

    /// Return the specific enthalpy of the moist air
    pub fn specific_enthalpy(&self) -> f64 {
        match self.unit {
            UnitSystem::SI => {
                self.t_dry_bulb * 0.240 + self.humidity_ratio * (1061.0 + 0.444 * self.t_dry_bulb)
            }
            UnitSystem::IP => {
                self.t_dry_bulb * 1.006 + self.humidity_ratio * (28.58 + 0.24 * self.t_dry_bulb)
            }
        }
    }
}

fn humidity_ratio_from_t_wet_bulb(
    t_dry_bulb: f64,
    t_wet_bulb: f64,
    pressure: f64,
    unit: UnitSystem,
) -> f64 {
    let saturated_water = SaturatedWater::new(t_wet_bulb, unit);
    let pw = saturated_water.saturation_pressure();
    let ws: f64 = MASS_RATIO_WATER_DRY_AIR * pw / (pressure - pw);
    let humidity_ratio: f64 = match unit {
        UnitSystem::SI => calculate_humidity_ratio_si(t_dry_bulb, t_wet_bulb, ws),
        UnitSystem::IP => calculate_humidity_ratio_ip(t_dry_bulb, t_wet_bulb, ws),
    };
    humidity_ratio.max(MIN_HUM_RATIO)
}

/// ASHRAE Handbook - Fundamentals (2013) IP Ch. 1 Eq. (35) and (37)
fn calculate_humidity_ratio_ip(t_dry_bulb: f64, t_wet_bulb: f64, wsstar: f64) -> f64 {
    match t_wet_bulb >= FREEZING_POINT_WATER_IP {
        true => {
            ((1093.0 - 0.556 * t_wet_bulb) * wsstar - 0.240 * (t_dry_bulb - t_wet_bulb))
                / (1093.0 + 0.444 * t_dry_bulb - t_wet_bulb)
        }
        false => {
            ((1220.0 - 0.04 * t_wet_bulb) * wsstar - 0.240 * (t_dry_bulb - t_wet_bulb))
                / (1220.0 + 0.444 * t_dry_bulb - 0.48 * t_wet_bulb)
        }
    }
}

/// ASHRAE Handbook - Fundamentals (2017) SI Ch. 1 Eq. (33) and (35)
fn calculate_humidity_ratio_si(t_dry_bulb: f64, t_wet_bulb: f64, wsstar: f64) -> f64 {
    match t_wet_bulb >= FREEZING_POINT_WATER_SI {
        true => {
            ((2501.0 - 2.326 * t_wet_bulb) * wsstar - 1.006 * (t_dry_bulb - t_wet_bulb))
                / (2501.0 + 1.86 * t_dry_bulb - 4.186 * t_wet_bulb)
        }
        false => {
            ((2830. - 0.24 * t_wet_bulb) * wsstar - 1.006 * (t_dry_bulb - t_wet_bulb))
                / (2830.0 + 1.86 * t_dry_bulb - 2.1 * t_wet_bulb)
        }
    }
}

fn humidity_ratio_from_relative_humidity(
    t_dry_bulb: f64,
    relative_humidity: f64,
    pressure: f64,
    unit: UnitSystem,
) -> f64 {
    // calculate vapor pressure from relative humidity
    let water_vapor = SaturatedWater::new(t_dry_bulb, unit);
    let pws = water_vapor.saturation_pressure();
    let pw = relative_humidity * pws;
    MASS_RATIO_WATER_DRY_AIR * pw / (pressure - pw)
}

////////////////////////////////////////////////////////////////////////////////////////////////////////
// Thermodynamic Properties of Water at Saturation
////////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct SaturatedWater {
    pub t_dry_bulb: f64,
    pub unit: UnitSystem,
}

impl Default for SaturatedWater {
    fn default() -> Self {
        SaturatedWater {
            t_dry_bulb: 20.0,
            unit: UnitSystem::SI,
        }
    }
}

impl SaturatedWater {
    pub fn new(t_dry_bulb: f64, unit: UnitSystem) -> Self {
        SaturatedWater { t_dry_bulb, unit }
    }

    pub fn saturation_pressure(&self) -> f64 {
        let ln_pws = match self.unit {
            UnitSystem::IP => {
                if !((-138.0..392.0).contains(&self.t_dry_bulb)) {
                    panic!("Dry bulb temperature is out of range");
                }
                self.ln_saturation_pressure_ip()
            }
            UnitSystem::SI => {
                if !((-200.0..200.0).contains(&self.t_dry_bulb)) {
                    panic!("Dry bulb temperature is out of range");
                }
                self.ln_saturation_pressure_si()
            }
        };
        f64::exp(ln_pws)
    }

    fn ln_saturation_pressure_ip(&self) -> f64 {
        let t_r: f64 = t_rankine_from_t_fahrenheit(self.t_dry_bulb);
        match self.t_dry_bulb >= TRIPLE_POINT_WATER_IP {
            true => {
                -1.0214165E+04 / t_r - 4.8932428 - 5.3765794E-03 * t_r
                    + 1.9202377E-07 * t_r.powi(2)
                    + 3.5575832E-10 * t_r.powi(3)
                    - 9.0344688E-14 * t_r.powi(4)
                    + 4.1635019 * t_r.ln()
            }
            false => {
                -1.0440397E+04 / t_r - 1.1294650E+01 - 2.7022355E-02 * t_r
                    + 1.2890360E-05 * t_r.powi(2)
                    - 2.4780681E-09 * t_r.powi(3)
                    + 6.5459673 * t_r.ln()
            }
        }
    }

    fn ln_saturation_pressure_si(&self) -> f64 {
        let t_k: f64 = t_kelvin_from_t_celsius(self.t_dry_bulb);
        match self.t_dry_bulb >= TRIPLE_POINT_WATER_SI {
            true => {
                -5.6745359E+03 / t_k + 6.3925247E+00 - 9.677843E-03 * t_k
                    + 6.2215701E-07 * t_k.powi(2)
                    + 2.0747825E-09 * t_k.powi(3)
                    - 9.484024E-13 * t_k.powi(4)
                    + 4.1635019 * t_k.ln()
            }
            false => {
                -5.8002206E+03 / t_k + 1.3914993E+00 - 4.8640239E-02 * t_k
                    + 4.1764768E-05 * t_k.powi(2)
                    - 1.4452093E-08 * t_k.powi(3)
                    + 6.5459673 * t_k.ln()
            }
        }
    }
}
