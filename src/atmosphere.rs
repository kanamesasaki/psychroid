/// Calculates the atmosphere temperature at a given altitude according to
/// U.S. Standard Atmosphere 1976, Eq (23)
///
/// # Arguments
/// * `altitude` - Altitude \[m\]
///
/// # Returns
/// * Temperature \[°C\]
/// * Returns NaN if altitude is out of valid range (0-84852m)
///
/// # Example
/// ```
/// use psychroid::atmosphere_temperature;
///
/// let temp = atmosphere_temperature(1000.0);
/// assert!((temp - 8.5).abs() < 0.1); // Should be around 8.5°C at 1000m
/// ```
pub fn atmosphere_temperature(altitude: f64) -> f64 {
    if !(0.0..=84852.0).contains(&altitude) {
        return f64::NAN;
    }

    match altitude {
        h if h <= 11000.0 => 15.0 - 0.0065 * h,
        h if h <= 20000.0 => -56.5,
        h if h <= 32000.0 => -56.5 + 0.001 * (h - 20000.0),
        h if h <= 47000.0 => -44.5 + 0.0028 * (h - 32000.0),
        h if h <= 51000.0 => -2.5,
        h if h <= 71000.0 => -2.5 - 0.0028 * (h - 51000.0),
        h if h <= 84852.0 => -58.5,
        _ => f64::NAN,
    }
}

// Physical constants
const GRAVITY: f64 = 9.80665; // Gravitational acceleration [m/s²]
const MOLAR_MASS: f64 = 0.0289644; // Molar mass of Earth's air [kg/mol]
const GAS_CONSTANT: f64 = 8.31447; // Universal gas constant [J/(mol·K)]

/// Calculates atmospheric pressure at a given altitude according to
/// U.S. Standard Atmosphere 1976, Eq (33a, 33b)
///
/// # Arguments
/// * `altitude` - Altitude \[m\]
///
/// # Returns
/// * Pressure \[Pa\]
/// * Returns NaN if altitude is out of valid range (0-84852m)
pub fn atmosphere_pressure(altitude: f64) -> f64 {
    if !(0.0..=84852.0).contains(&altitude) {
        return f64::NAN;
    }

    match altitude {
        h if h <= 11000.0 => p0(h),
        h if h <= 20000.0 => p1(h),
        h if h <= 32000.0 => p2(h),
        h if h <= 47000.0 => p3(h),
        h if h <= 51000.0 => p4(h),
        h if h <= 71000.0 => p5(h),
        h if h <= 84852.0 => p6(h),
        _ => f64::NAN,
    }
}

fn p0(altitude: f64) -> f64 {
    let l_mb = -0.0065;
    let t_mb = 15.0 + 273.15;
    let p_mb = 101325.0;
    let h_b = 0.0;
    p_mb * (t_mb / (t_mb + l_mb * (altitude - h_b)))
        .powf(GRAVITY * MOLAR_MASS / (GAS_CONSTANT * l_mb))
}

fn p1(altitude: f64) -> f64 {
    // let l_mb = 0.0;
    let t_mb = -56.5 + 273.15;
    let h_b = 11000.0;
    let p_mb = p0(h_b);
    p_mb * (-GRAVITY * MOLAR_MASS * (altitude - h_b) / (GAS_CONSTANT * t_mb)).exp()
}

fn p2(altitude: f64) -> f64 {
    let l_mb = 0.001;
    let t_mb = -56.5 + 273.15;
    let h_b = 20000.0;
    let p_mb = p1(h_b);
    p_mb * (t_mb / (t_mb + l_mb * (altitude - h_b)))
        .powf(GRAVITY * MOLAR_MASS / (GAS_CONSTANT * l_mb))
}

fn p3(altitude: f64) -> f64 {
    let l_mb = 0.0028;
    let t_mb = -44.5 + 273.15;
    let h_b = 32000.0;
    let p_mb = p2(h_b);
    p_mb * (t_mb / (t_mb + l_mb * (altitude - h_b)))
        .powf(GRAVITY * MOLAR_MASS / (GAS_CONSTANT * l_mb))
}

fn p4(altitude: f64) -> f64 {
    // let l_mb = 0.0;
    let t_mb = -2.5 + 273.15;
    let h_b = 47000.0;
    let p_mb = p3(h_b);
    p_mb * (-GRAVITY * MOLAR_MASS * (altitude - h_b) / (GAS_CONSTANT * t_mb)).exp()
}

fn p5(altitude: f64) -> f64 {
    let l_mb = -0.0028;
    let t_mb = -2.5 + 273.15;
    let h_b = 51000.0;
    let p_mb = p4(h_b);
    p_mb * (t_mb / (t_mb + l_mb * (altitude - h_b)))
        .powf(GRAVITY * MOLAR_MASS / (GAS_CONSTANT * l_mb))
}

fn p6(altitude: f64) -> f64 {
    let l_mb = -0.0020;
    let t_mb = -58.5 + 273.15;
    let h_b = 71000.0;
    let p_mb = p5(h_b);
    p_mb * (t_mb / (t_mb + l_mb * (altitude - h_b)))
        .powf(GRAVITY * MOLAR_MASS / (GAS_CONSTANT * l_mb))
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;

    #[test]
    fn test_atmosphere_temperature() {
        assert_eq!(atmosphere_temperature(0.0), 15.0);
        assert_eq!(atmosphere_temperature(1000.0), 8.5);
        assert_eq!(atmosphere_temperature(11000.0), -56.5);
        assert_eq!(atmosphere_temperature(20000.0), -56.5);
        assert_eq!(atmosphere_temperature(32000.0), -44.5);
        assert_eq!(atmosphere_temperature(47000.0), -2.5);
        assert_eq!(atmosphere_temperature(51000.0), -2.5);
        assert_eq!(atmosphere_temperature(71000.0), -58.5);
        assert_eq!(atmosphere_temperature(84852.0), -58.5);
    }

    #[test]
    fn test_atmosphere_temperature_out_of_range() {
        assert!(atmosphere_temperature(-1.0).is_nan());
        assert!(atmosphere_temperature(84853.0).is_nan());
    }

    #[test]
    fn test_atmosphere_pressure() {
        assert_eq!(atmosphere_pressure(0.0), 101325.0);
        assert_abs_diff_eq!(atmosphere_pressure(1000.0), 89.875E+03, epsilon = 1.0);
        assert_abs_diff_eq!(atmosphere_pressure(11000.0), 22.632E+03, epsilon = 1.0);
    }
}
