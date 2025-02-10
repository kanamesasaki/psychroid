use crate::common::UnitSystem;
use crate::common::{t_celsius_to_t_kelvin, t_rankine_from_t_fahrenheit};
use crate::common::{TRIPLE_POINT_WATER_IP, TRIPLE_POINT_WATER_SI};

/// <script src="https://cdn.jsdelivr.net/npm/mathjax@3/es5/tex-mml-chtml.js"></script>
#[derive(Debug)]
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
        if !(-100.0..200.0).contains(&t_dry_bulb) {
            panic!("Dry bulb temperature is out of range");
        }
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
                if !((-100.0..200.0).contains(&self.t_dry_bulb)) {
                    panic!("Dry bulb temperature is out of range");
                }
                self.ln_saturation_pressure_si()
            }
        };
        f64::exp(ln_pws)
    }

    pub fn deriv_saturation_pressure(&self) -> f64 {
        match self.unit {
            UnitSystem::IP => {
                if !((-138.0..392.0).contains(&self.t_dry_bulb)) {
                    panic!("Dry bulb temperature is out of range");
                }
                f64::exp(self.ln_saturation_pressure_ip()) * self.deriv_ln_saturation_pressure_ip()
            }
            UnitSystem::SI => {
                if !((-100.0..200.0).contains(&self.t_dry_bulb)) {
                    panic!("Dry bulb temperature is out of range");
                }
                f64::exp(self.ln_saturation_pressure_si()) * self.deriv_ln_saturation_pressure_si()
            }
        }
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
        let t_k: f64 = t_celsius_to_t_kelvin(self.t_dry_bulb);
        match self.t_dry_bulb < TRIPLE_POINT_WATER_SI {
            true => {
                -5.6745359E+03 / t_k + 6.3925247E+00 - 9.677843E-03 * t_k
                    + 6.2215701E-07 * t_k.powi(2)
                    + 2.0747825E-09 * t_k.powi(3)
                    - 9.4840240E-13 * t_k.powi(4)
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

    fn deriv_ln_saturation_pressure_ip(&self) -> f64 {
        let t_r: f64 = t_rankine_from_t_fahrenheit(self.t_dry_bulb);
        match self.t_dry_bulb >= TRIPLE_POINT_WATER_IP {
            true => {
                1.0214165E+04 / t_r.powi(2) - 5.3765794E-03
                    + 2.0 * 1.9202377E-07 * t_r
                    + 3.0 * 3.5575832E-10 * t_r.powi(2)
                    - 4.0 * 9.0344688E-14 * t_r.powi(3)
            }
            false => {
                1.0440397E+04 / t_r.powi(2) - 2.7022355E-02 + 2.0 * 1.2890360E-05 * t_r
                    - 3.0 * 2.4780681E-09 * t_r.powi(2)
            }
        }
    }

    fn deriv_ln_saturation_pressure_si(&self) -> f64 {
        let t_k: f64 = t_celsius_to_t_kelvin(self.t_dry_bulb);
        match self.t_dry_bulb >= TRIPLE_POINT_WATER_SI {
            true => {
                5.6745359E+03 / t_k.powi(2) - 9.677843E-03
                    + 2.0 * 6.2215701E-07 * t_k
                    + 3.0 * 2.0747825E-09 * t_k.powi(2)
                    - 4.0 * 9.4840240E-13 * t_k.powi(3)
            }
            false => {
                5.8002206E+03 / t_k.powi(2) - 4.8640239E-02 + 2.0 * 4.1764768E-05 * t_k
                    - 3.0 * 1.4452093E-08 * t_k.powi(2)
            }
        }
    }

    /// Calculates the specific enthalpy of saturated water vapor
    ///
    /// # Returns
    /// The specific enthalpy of saturated water vapor:
    /// - \\( \\mathrm{kJ/kg} \\) for SI units
    /// - \\( \\mathrm{Btu/lb} \\) for IP units
    ///
    /// # Formula
    /// $$
    /// \\begin{align}
    /// \\mathrm{SI~units:}\\quad h_g &= 2501.0 + 1.860~t \\\\
    /// \\mathrm{IP~units:}\\quad h_g &= 1061.0 + 0.444~t
    /// \\end{align}
    /// $$
    /// where:
    /// - \\(t\\) is the saturation temperature in \\(^\\circ \\mathrm{C}\\) or \\(^\\circ \\mathrm{F}\\)
    ///
    /// Reference: ASHRAE Fundamentals Handbook (2017) Chapter 1
    pub fn specific_enthalpy(&self) -> f64 {
        match self.unit {
            UnitSystem::SI => 2501.0 + 1.860 * self.t_dry_bulb,
            UnitSystem::IP => 1061.0 + 0.444 * self.t_dry_bulb,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;

    #[test]
    fn test_saturation_pressure_positive_si() {
        let mut wsat = SaturatedWater::new(1.0, UnitSystem::SI);
        assert_abs_diff_eq!(wsat.saturation_pressure(), 0.6571E+03, epsilon = 0.1);

        wsat.t_dry_bulb = 10.0;
        assert_abs_diff_eq!(wsat.saturation_pressure(), 1.2282E+03, epsilon = 0.5);

        wsat.t_dry_bulb = 20.0;
        assert_abs_diff_eq!(wsat.saturation_pressure(), 2.3392E+03, epsilon = 0.5);

        wsat.t_dry_bulb = 30.0;
        assert_abs_diff_eq!(wsat.saturation_pressure(), 4.2467E+03, epsilon = 0.7);

        wsat.t_dry_bulb = 40.0;
        assert_abs_diff_eq!(wsat.saturation_pressure(), 7.3844E+03, epsilon = 1.0);

        wsat.t_dry_bulb = 50.0;
        assert_abs_diff_eq!(wsat.saturation_pressure(), 12.3513E+03, epsilon = 2.0);

        wsat.t_dry_bulb = 100.0;
        assert_abs_diff_eq!(wsat.saturation_pressure(), 101.4180E+03, epsilon = 0.8);

        wsat.t_dry_bulb = 150.0;
        assert_abs_diff_eq!(wsat.saturation_pressure(), 476.1014E+03, epsilon = 97.0);
    }

    #[test]
    fn test_saturation_pressure_negative_si() {
        let mut wsat = SaturatedWater::new(0.0, UnitSystem::SI);
        assert_abs_diff_eq!(wsat.saturation_pressure(), 0.61115E+03, epsilon = 0.01);

        wsat.t_dry_bulb = -10.0;
        assert_abs_diff_eq!(wsat.saturation_pressure(), 0.25987E+03, epsilon = 0.04);

        wsat.t_dry_bulb = -20.0;
        assert_abs_diff_eq!(wsat.saturation_pressure(), 0.10324E+03, epsilon = 0.03);

        wsat.t_dry_bulb = -30.0;
        assert_abs_diff_eq!(wsat.saturation_pressure(), 0.03801E+03, epsilon = 0.01);

        wsat.t_dry_bulb = -40.0;
        assert_abs_diff_eq!(wsat.saturation_pressure(), 0.01284E+03, epsilon = 0.01);

        wsat.t_dry_bulb = -50.0;
        assert_abs_diff_eq!(wsat.saturation_pressure(), 0.00394E+03, epsilon = 0.01);

        wsat.t_dry_bulb = -60.0;
        assert_abs_diff_eq!(wsat.saturation_pressure(), 0.00108E+03, epsilon = 0.01);
    }
}
