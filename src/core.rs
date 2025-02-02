use std::f64::consts::E;

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

/// Calculate the dew point temperature from relative humidity
pub fn t_dew_point_from_relative_humidity(t_dry_bulb: f64, rel_hum: f64) -> f64 {
    assert!(
        rel_hum >= 0.0 && rel_hum <= 1.0,
        "Relative humidity out of range"
    );
    let vap_pres = vap_pres_from_rel_hum(t_dry_bulb, rel_hum);
    t_dew_point_from_vapor_pressure(t_dry_bulb, vap_pres)
}

/// Calculate vapor pressure from relative humidity
pub fn vap_pres_from_rel_hum(t_dry_bulb: f64, rel_hum: f64) -> f64 {
    assert!(
        rel_hum >= 0.0 && rel_hum <= 1.0,
        "Relative humidity out of range"
    );
    rel_hum * saturation_vapor_pressure(t_dry_bulb)
}

/// Get dew point temperature from vapor pressure
pub fn t_dew_point_from_vapor_pressure(t_dry_bulb: f64, vap_pres: f64, unit: UnitSystem) -> f64 {
    let mut t_dew = t_dry_bulb;
    let ln_vp = vap_pres.ln();
    let mut t_iter;
    let mut ln_vp_iter;
    let mut index = 1;
    loop {
        t_iter = t_dew;
        ln_vp_iter = saturation_vapor_pressure(t_iter).ln();
        let d_ln_vp = d_ln_pws(t_iter);
        t_dew = t_iter - (ln_vp_iter - ln_vp) / d_ln_vp;
        assert!(index <= MAX_ITER_COUNT, "Convergence not reached");
        if (t_dew - t_iter).abs()
            <= match unit {
                UnitSystem::SI => TOLERANCE_SI,
                UnitSystem::IP => TOLERANCE_IP,
            }
        {
            break;
        }
        index += 1;
    }
    t_dew.min(t_dry_bulb)
}

/// Helper function to calculate derivative of natural log of saturation vapor pressure
fn d_ln_pws(t_dry_bulb: f64) -> f64 {
    let t = if is_ip() {
        t_rankine_from_t_fahrenheit(t_dry_bulb)
    } else {
        t_kelvin_from_t_celsius(t_dry_bulb)
    };
    if is_ip() {
        if t_dry_bulb <= TRIPLE_POINT_WATER_IP {
            1.0214165E+04 / t.powi(2) - 5.3765794E-03 + 2.0 * 1.9202377E-07 * t
        } else {
            1.0440397E+04 / t.powi(2) - 2.7022355E-02 + 2.0 * 1.2890360E-05 * t
        }
    } else {
        if t_dry_bulb <= TRIPLE_POINT_WATER_SI {
            5.6745359E+03 / t.powi(2) - 9.677843E-03 + 2.0 * 6.2215701E-07 * t
        } else {
            5.8002206E+03 / t.powi(2) - 4.8640239E-02 + 2.0 * 4.1764768E-05 * t
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////
// Saturated Air Calculations
////////////////////////////////////////////////////////////////////////////////////////////////////////

/// Calculates the saturation vapor pressure of water vapor in moist air
///
/// Uses ASHRAE Fundamentals (2017) Chapter 1, Equation 5 & 6
/// for temperatures above and below the triple point of water.
///
/// # Arguments
/// * `t_dry_bulb` - Dry bulb temperature in °C (SI) or °F (IP)
///
/// # Returns
/// * Saturation vapor pressure in Pa (SI) or Psi (IP)
///
/// # Note
/// The function automatically handles SI/IP unit systems based on the current configuration.
/// Triple point of water is 0.01°C (SI) or 32.018°F (IP)
///
fn saturation_vapor_pressure(t_dry_bulb: f64) -> f64 {
    let t = if is_ip() {
        t_rankine_from_t_fahrenheit(t_dry_bulb)
    } else {
        t_kelvin_from_t_celsius(t_dry_bulb)
    };
    let ln_pws = if is_ip() {
        if t_dry_bulb <= TRIPLE_POINT_WATER_IP {
            -1.0214165E+04 / t - 4.8932428
        } else {
            -1.0440397E+04 / t - 1.1294650E+01
        }
    } else {
        if t_dry_bulb <= TRIPLE_POINT_WATER_SI {
            -5.6745359E+03 / t + 6.3925247
        } else {
            -5.8002206E+03 / t + 1.3914993
        }
    };
    E.powf(ln_pws)
}

/// Return humidity ratio of saturated air given dry-bulb temperature and pressure.
pub fn saturated_air_humidity_ratio(t_dry_bulb: f64, pressure: f64) -> f64 {
    let sat_vapor_pres = saturation_vapor_pressure(t_dry_bulb);
    let sat_hum_ratio = 0.621945 * sat_vapor_pres / (pressure - sat_vapor_pres);
    sat_hum_ratio.max(MIN_HUM_RATIO)
}

/// Return saturated air enthalpy given dry-bulb temperature and pressure.
pub fn saturation_air_enthalpy(t_dry_bulb: f64, pressure: f64) -> f64 {
    moist_air_enthalpy(
        t_dry_bulb,
        saturation_air_humidity_ratio(t_dry_bulb, pressure),
    )
}

////////////////////////////////////////////////////////////////////////////////////////////////////////
// Moist Air Calculations
////////////////////////////////////////////////////////////////////////////////////////////////////////

/// Calculates the vapor pressure deficit in moist air
///
/// Reference: Oke (1987) equation 2.13a
///
/// # Arguments
/// * `t_dry_bulb` - Dry bulb temperature in °C (SI) or °F (IP)
/// * `humidity_ratio` - Humidity ratio in kg_H₂O/kg_Air (SI) or lb_H₂O/lb_Air (IP)
/// * `pressure` - Atmospheric pressure in Pa (SI) or Psi (IP)
///
/// # Returns
/// * Vapor pressure deficit in Pa (SI) or Psi (IP)
///
pub fn vapor_pressure_deficit(t_dry_bulb: f64, humidity_ratio: f64, pressure: f64) -> f64 {
    assert!(humidity_ratio >= 0.0, "Humidity ratio cannot be negative");

    let rel_humidity = relative_humidity_from_humidity_ratio(t_dry_bulb, humidity_ratio, pressure);
    saturation_vapor_pressure(t_dry_bulb) * (1.0 - rel_humidity)
}

/// Calculates the degree of saturation in moist air
///
/// The degree of saturation is the ratio of the humidity ratio of the air to the
/// humidity ratio of the air at saturation at the same temperature and pressure.
///
/// Reference: ASHRAE Handbook - Fundamentals (2009) ch. 1 eqn. 12
/// Note: This definition is absent from the 2017 Handbook
///
/// # Arguments
/// * `t_dry_bulb` - Dry bulb temperature in °C (SI) or °F (IP)
/// * `humidity_ratio` - Humidity ratio in kg_H₂O/kg_Air (SI) or lb_H₂O/lb_Air (IP)
/// * `pressure` - Atmospheric pressure in Pa (SI) or Psi (IP)
///
/// # Returns
/// * Degree of saturation (dimensionless)
///
pub fn degree_of_saturation(t_dry_bulb: f64, humidity_ratio: f64, pressure: f64) -> f64 {
    assert!(humidity_ratio >= 0.0, "Humidity ratio cannot be negative");

    let bounded_humidity_ratio = humidity_ratio.max(MIN_HUM_RATIO);
    bounded_humidity_ratio / saturated_air_humidity_ratio(t_dry_bulb, pressure)
}

/// Calculates moist air enthalpy
///
/// Reference: ASHRAE Handbook - Fundamentals (2017) ch. 1 eqn. 30
///
/// # Arguments
/// * `t_dry_bulb` - Dry bulb temperature in °C (SI) or °F (IP)
/// * `humidity_ratio` - Humidity ratio in kg_H₂O/kg_Air (SI) or lb_H₂O/lb_Air (IP)
///
/// # Returns
/// * Moist air enthalpy in J/kg (SI) or Btu/lb (IP)
///
pub fn moist_air_enthalpy(t_dry_bulb: f64, humidity_ratio: f64) -> f64 {
    assert!(humidity_ratio >= 0.0, "Humidity ratio cannot be negative");

    let bounded_humidity_ratio = humidity_ratio.max(MIN_HUM_RATIO);

    if is_ip() {
        0.240 * t_dry_bulb + bounded_humidity_ratio * (1061.0 + 0.444 * t_dry_bulb)
    } else {
        (1.006 * t_dry_bulb + bounded_humidity_ratio * (2501.0 + 1.86 * t_dry_bulb)) * 1000.0
    }
}

/// Calculates moist air specific volume
///
/// Reference: ASHRAE Handbook - Fundamentals (2017) ch. 1 eqn. 26
/// Note: In IP units, R_DA_IP / 144 equals 0.370486 (coefficient in eqn 26)
/// The factor 144 converts Psi (lb/in²) to lb/ft²
///
/// # Arguments
/// * `t_dry_bulb` - Dry bulb temperature in °C (SI) or °F (IP)
/// * `humidity_ratio` - Humidity ratio in kg_H₂O/kg_Air (SI) or lb_H₂O/lb_Air (IP)
/// * `pressure` - Atmospheric pressure in Pa (SI) or Psi (IP)
///
/// # Returns
/// * Specific volume in m³/kg (SI) or ft³/lb (IP)
///
pub fn moist_air_volume(t_dry_bulb: f64, humidity_ratio: f64, pressure: f64) -> f64 {
    assert!(humidity_ratio >= 0.0, "Humidity ratio cannot be negative");

    let bounded_humidity_ratio = humidity_ratio.max(MIN_HUM_RATIO);

    if is_ip() {
        R_DA_IP
            * t_rankine_from_t_fahrenheit(t_dry_bulb)
            * (1.0 + 1.607858 * bounded_humidity_ratio)
            / (144.0 * pressure)
    } else {
        R_DA_SI * t_kelvin_from_t_celsius(t_dry_bulb) * (1.0 + 1.607858 * bounded_humidity_ratio)
            / pressure
    }
}

/// Calculates dry-bulb temperature from moist air volume, humidity ratio, and pressure
///
/// Reference: ASHRAE Handbook - Fundamentals (2017) ch. 1 eqn 26
/// Note: In IP units, R_DA_IP / 144 equals 0.370486 (coefficient in eqn 26)
/// The factor 144 converts Psi (lb/in²) to lb/ft²
///
/// # Arguments
/// * `moist_air_volume` - Specific volume in m³/kg (SI) or ft³/lb (IP)
/// * `humidity_ratio` - Humidity ratio in kg_H₂O/kg_Air (SI) or lb_H₂O/lb_Air (IP)
/// * `pressure` - Atmospheric pressure in Pa (SI) or Psi (IP)
///
/// # Returns
/// * Dry-bulb temperature in °C (SI) or °F (IP)
///
pub fn t_dry_bulb_from_moist_air_volume_and_humidity_ratio(
    moist_air_volume: f64,
    humidity_ratio: f64,
    pressure: f64,
) -> f64 {
    assert!(humidity_ratio >= 0.0, "Humidity ratio cannot be negative");

    let bounded_humidity_ratio = humidity_ratio.max(MIN_HUM_RATIO);

    if is_ip() {
        t_fahrenheit_from_t_rankine(
            moist_air_volume * (144.0 * pressure)
                / (R_DA_IP * (1.0 + 1.607858 * bounded_humidity_ratio)),
        )
    } else {
        t_celsius_from_t_kelvin(
            moist_air_volume * pressure / (R_DA_SI * (1.0 + 1.607858 * bounded_humidity_ratio)),
        )
    }
}

/// Calculates moist air density
///
/// Reference: ASHRAE Handbook - Fundamentals (2017) ch. 1 eqn. 11
///
/// # Arguments
/// * `t_dry_bulb` - Dry bulb temperature in °C (SI) or °F (IP)
/// * `humidity_ratio` - Humidity ratio in kg_H₂O/kg_Air (SI) or lb_H₂O/lb_Air (IP)
/// * `pressure` - Atmospheric pressure in Pa (SI) or Psi (IP)
///
/// # Returns
/// * Moist air density in kg/m³ (SI) or lb/ft³ (IP)
///
pub fn moist_air_density(t_dry_bulb: f64, humidity_ratio: f64, pressure: f64) -> f64 {
    assert!(humidity_ratio >= 0.0, "Humidity ratio cannot be negative");

    let bounded_humidity_ratio = humidity_ratio.max(MIN_HUM_RATIO);
    (1.0 + bounded_humidity_ratio) / moist_air_volume(t_dry_bulb, bounded_humidity_ratio, pressure)
}

////////////////////////////////////////////////////////////////////////////////////////////////////////
// Standard atmosphere
////////////////////////////////////////////////////////////////////////////////////////////////////////

/// Calculates standard atmosphere barometric pressure at given elevation
///
/// Reference: ASHRAE Handbook - Fundamentals (2017) ch. 1 eqn 3
///
/// # Arguments
/// * `altitude` - Elevation in m (SI) or ft (IP)
///
/// # Returns
/// * Standard atmosphere pressure in Pa (SI) or Psi (IP)
///
pub fn standard_atmosphere_pressure(altitude: f64) -> f64 {
    if is_ip() {
        14.696 * (1.0 - 6.8754e-06 * altitude).powf(5.2559)
    } else {
        101325.0 * (1.0 - 2.25577e-05 * altitude).powf(5.2559)
    }
}

/// Calculates standard atmosphere temperature at given elevation
///
/// Reference: ASHRAE Handbook - Fundamentals (2017) ch. 1 eqn 4
///
/// # Arguments
/// * `altitude` - Elevation in m (SI) or ft (IP)
///
/// # Returns
/// * Standard atmosphere temperature in °C (SI) or °F (IP)
///
pub fn standard_atmosphere_temperature(altitude: f64) -> f64 {
    if is_ip() {
        59.0 - 0.00356620 * altitude
    } else {
        15.0 - 0.0065 * altitude
    }
}

/// Calculates sea level pressure from station conditions
///
/// Reference: Hess SL, Introduction to theoretical meteorology (1959)
/// and Stull RB, Meteorology for scientists and engineers, 2nd edition (2000)
///
/// Note: US standard procedure uses average of current and 12-hour old temperature
///
/// # Arguments
/// * `station_pressure` - Observed station pressure in Pa (SI) or Psi (IP)
/// * `altitude` - Altitude above sea level in m (SI) or ft (IP)
/// * `t_dry_bulb` - Dry bulb temperature in °C (SI) or °F (IP)
///
/// # Returns
/// * Sea level pressure in Pa (SI) or Psi (IP)
///
pub fn sea_level_pressure(station_pressure: f64, altitude: f64, t_dry_bulb: f64) -> f64 {
    let (t_column, h) = if is_ip() {
        // Lapse rate 3.6 °F/1000ft
        let t_col = t_dry_bulb + 0.0036 * altitude / 2.0;
        let scale_h = 53.351 * t_rankine_from_t_fahrenheit(t_col);
        (t_col, scale_h)
    } else {
        // Lapse rate 6.5 °C/km
        let t_col = t_dry_bulb + 0.0065 * altitude / 2.0;
        let scale_h = 287.055 * t_kelvin_from_t_celsius(t_col) / 9.807;
        (t_col, scale_h)
    };

    station_pressure * (altitude / h).exp()
}

/// Calculates station pressure from sea level pressure
///
/// Reference: Inverse of sea_level_pressure calculation
///
/// # Arguments
/// * `sea_level_pressure` - Sea level pressure in Pa (SI) or Psi (IP)
/// * `altitude` - Altitude above sea level in m (SI) or ft (IP)
/// * `t_dry_bulb` - Dry bulb temperature in °C (SI) or °F (IP)
///
/// # Returns
/// * Station pressure in Pa (SI) or Psi (IP)
///
pub fn station_pressure(sea_level_pressure: f64, altitude: f64, t_dry_bulb: f64) -> f64 {
    sea_level_pressure / sea_level_pressure(1.0, altitude, t_dry_bulb)
}

////////////////////////////////////////////////////////////////////////////////////////////////////////
// Functions to set all psychrometric values
////////////////////////////////////////////////////////////////////////////////////////////////////////

/// Contains all calculated psychrometric values
#[derive(Debug)]
pub struct MoistAir {
    pub dry_bulb_temperature: f64, // °C (SI) or °F (IP)
    pub humidity_ratio: f64,       // kg_H₂O/kg_Air (SI) or lb_H₂O/lb_Air (IP)
    pub pressure: f64,             // Pa (SI) or Psi (IP)
    pub unit: UnitSystem,          // SI or IP
}

impl Default for MoistAir {
    fn default() -> Self {
        MoistAir {
            dry_bulb_temperature: 20.0,
            humidity_ratio: 0.00735,
            pressure: 101325.0,
            unit: UnitSystem::SI,
        }
    }
}

impl MoistAir {
    /// Create a new instance of MoistAir
    pub fn new(
        dry_bulb_temperature: f64,
        humidity_ratio: f64,
        pressure: f64,
        unit: UnitSystem,
    ) -> Self {
        MoistAir {
            dry_bulb_temperature,
            humidity_ratio,
            pressure,
            unit,
        }
    }

    /// Init from wet bulb temperature
    pub fn new_from_wet_bulb(
        dry_bulb_temperature: f64,
        wet_bulb_temperature: f64,
        pressure: f64,
        unit: UnitSystem,
    ) -> Self {
        let humidity_ratio =
            humidity_ratio_from_wet_bulb(dry_bulb_temperature, wet_bulb_temperature, pressure);
        MoistAir {
            dry_bulb_temperature,
            humidity_ratio,
            pressure,
            unit,
        }
    }
}

fn humidity_ratio_from_wet_bulb(
    t_dry_bulb: f64,
    t_wet_bulb: f64,
    pressure: f64,
    unit: UnitSystem,
) -> f64 {
    let wsstar: f64 = saturation_humidity_ratio(t_wet_bulb, pressure);
    let humidity_ratio: f64 = match unit {
        UnitSystem::SI => calculate_humidity_ratio_si(t_dry_bulb, t_wet_bulb, wsstar),
        UnitSystem::IP => calculate_humidity_ratio_ip(t_dry_bulb, t_wet_bulb, wsstar),
    };
    humidity_ratio.max(MIN_HUM_RATIO)
}

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

fn calculate_humidity_ratio_si(t_dry_bulb: f64, t_wet_bulb: f64, wsstar: f64) -> f64 {
    match t_wet_bulb >= FREEZING_POINT_WATER_SI {
        true => {
            ((2501. - 2.326 * t_wet_bulb) * wsstar - 1.006 * (t_dry_bulb - t_wet_bulb))
                / (2501. + 1.86 * t_dry_bulb - 4.186 * t_wet_bulb)
        }
        false => {
            ((2830. - 0.24 * t_wet_bulb) * wsstar - 1.006 * (t_dry_bulb - t_wet_bulb))
                / (2830. + 1.86 * t_dry_bulb - 2.1 * t_wet_bulb)
        }
    }
}

/// Calculate psychrometric values from wet-bulb temperature
pub fn calc_psychrometrics_from_wet_bulb(
    t_dry_bulb: f64,
    t_wet_bulb: f64,
    pressure: f64,
) -> PsychrometricValues {
    assert!(
        t_wet_bulb <= t_dry_bulb,
        "Wet bulb temperature cannot exceed dry bulb temperature"
    );

    let humidity_ratio = humidity_ratio_from_wet_bulb(t_dry_bulb, t_wet_bulb, pressure);
    PsychrometricValues {
        humidity_ratio,
        wet_bulb_temp: t_wet_bulb,
        dew_point: dew_point_from_humidity_ratio(t_dry_bulb, humidity_ratio, pressure),
        relative_humidity: relative_humidity_from_humidity_ratio(
            t_dry_bulb,
            humidity_ratio,
            pressure,
        ),
        vapor_pressure: vapor_pressure_from_humidity_ratio(humidity_ratio, pressure),
        moist_air_enthalpy: moist_air_enthalpy(t_dry_bulb, humidity_ratio),
        moist_air_volume: moist_air_volume(t_dry_bulb, humidity_ratio, pressure),
        degree_of_saturation: degree_of_saturation(t_dry_bulb, humidity_ratio, pressure),
    }
}

pub fn calc_psychrometrics_from_dew_point(
    t_dry_bulb: f64,
    t_dew_point: f64,
    pressure: f64,
) -> PsychrometricValues {
    assert!(
        t_dew_point <= t_dry_bulb,
        "Dew point temperature cannot exceed dry bulb temperature"
    );

    let humidity_ratio = humidity_ratio_from_dew_point(t_dew_point, pressure);
    PsychrometricValues {
        humidity_ratio,
        wet_bulb_temp: wet_bulb_from_humidity_ratio(t_dry_bulb, humidity_ratio, pressure),
        dew_point: t_dew_point,
        relative_humidity: relative_humidity_from_humidity_ratio(
            t_dry_bulb,
            humidity_ratio,
            pressure,
        ),
        vapor_pressure: vapor_pressure_from_humidity_ratio(humidity_ratio, pressure),
        moist_air_enthalpy: moist_air_enthalpy(t_dry_bulb, humidity_ratio),
        moist_air_volume: moist_air_volume(t_dry_bulb, humidity_ratio, pressure),
        degree_of_saturation: degree_of_saturation(t_dry_bulb, humidity_ratio, pressure),
    }
}

pub fn calc_psychrometrics_from_relative_humidity(
    t_dry_bulb: f64,
    relative_humidity: f64,
    pressure: f64,
) -> PsychrometricValues {
    assert!(
        (0.0..=1.0).contains(&relative_humidity),
        "Relative humidity must be between 0 and 1"
    );

    let humidity_ratio =
        humidity_ratio_from_relative_humidity(t_dry_bulb, relative_humidity, pressure);
    PsychrometricValues {
        humidity_ratio,
        wet_bulb_temp: wet_bulb_from_humidity_ratio(t_dry_bulb, humidity_ratio, pressure),
        dew_point: dew_point_from_humidity_ratio(t_dry_bulb, humidity_ratio, pressure),
        relative_humidity,
        vapor_pressure: vapor_pressure_from_humidity_ratio(humidity_ratio, pressure),
        moist_air_enthalpy: moist_air_enthalpy(t_dry_bulb, humidity_ratio),
        moist_air_volume: moist_air_volume(t_dry_bulb, humidity_ratio, pressure),
        degree_of_saturation: degree_of_saturation(t_dry_bulb, humidity_ratio, pressure),
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////
// Saturated Water
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
    /// Create a new instance of SaturatedAir
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

        E.powf(ln_pws)
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
