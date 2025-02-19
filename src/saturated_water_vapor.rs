use crate::common::UnitSystem;
use crate::common::{t_celsius_to_t_kelvin, t_rankine_from_t_fahrenheit};
use crate::common::{TRIPLE_POINT_WATER_IP, TRIPLE_POINT_WATER_SI};

const C1_SI: f64 = -5.6745359E+03;
const C2_SI: f64 = 6.3925247E+00;
const C3_SI: f64 = -9.677843E-03;
const C4_SI: f64 = 6.2215701E-07;
const C5_SI: f64 = 2.0747825E-09;
const C6_SI: f64 = -9.4840240E-13;
const C7_SI: f64 = 4.1635019E+00;

const C8_SI: f64 = -5.8002206E+03;
const C9_SI: f64 = 1.3914993E+00;
const C10_SI: f64 = -4.8640239E-02;
const C11_SI: f64 = 4.1764768E-05;
const C12_SI: f64 = -1.4452093E-08;
const C13_SI: f64 = 6.5459673E+00;

const C1_IP: f64 = -1.0214165E+04;
const C2_IP: f64 = -4.8932428E+00;
const C3_IP: f64 = -5.3765794E-03;
const C4_IP: f64 = 1.9202377E-07;
const C5_IP: f64 = 3.5575832E-10;
const C6_IP: f64 = -9.0344688E-14;
const C7_IP: f64 = 4.1635019E+00;

const C8_IP: f64 = -1.0440397E+04;
const C9_IP: f64 = -1.1294650E+01;
const C10_IP: f64 = -2.7022355E-02;
const C11_IP: f64 = 1.2890360E-05;
const C12_IP: f64 = -2.4780681E-09;
const C13_IP: f64 = 6.5459673E+00;

/// <script src="https://cdn.jsdelivr.net/npm/mathjax@3/es5/tex-mml-chtml.js"></script>
#[derive(Debug)]
pub struct SaturatedWaterVapor {
    pub t_dry_bulb: f64,
    pub unit: UnitSystem,
}

impl Default for SaturatedWaterVapor {
    fn default() -> Self {
        SaturatedWaterVapor {
            t_dry_bulb: 20.0,
            unit: UnitSystem::SI,
        }
    }
}

impl SaturatedWaterVapor {
    pub fn new(t_dry_bulb: f64, unit: UnitSystem) -> Self {
        if !(-100.0..200.0).contains(&t_dry_bulb) {
            panic!("Dry bulb temperature is out of range");
        }
        SaturatedWaterVapor { t_dry_bulb, unit }
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
                C1_IP / t_r
                    + C2_IP
                    + C3_IP * t_r
                    + C4_IP * t_r.powi(2)
                    + C5_IP * t_r.powi(3)
                    + C6_IP * t_r.powi(4)
                    + C7_IP * t_r.ln()
            }
            false => {
                C8_IP / t_r
                    + C9_IP
                    + C10_IP * t_r
                    + C11_IP * t_r.powi(2)
                    + C12_IP * t_r.powi(3)
                    + C13_IP * t_r.ln()
            }
        }
    }

    fn ln_saturation_pressure_si(&self) -> f64 {
        let t_k: f64 = t_celsius_to_t_kelvin(self.t_dry_bulb);
        match self.t_dry_bulb < TRIPLE_POINT_WATER_SI {
            true => {
                C1_SI / t_k
                    + C2_SI
                    + C3_SI * t_k
                    + C4_SI * t_k.powi(2)
                    + C5_SI * t_k.powi(3)
                    + C6_SI * t_k.powi(4)
                    + C7_SI * t_k.ln()
            }
            false => {
                C8_SI / t_k
                    + C9_SI
                    + C10_SI * t_k
                    + C11_SI * t_k.powi(2)
                    + C12_SI * t_k.powi(3)
                    + C13_SI * t_k.ln()
            }
        }
    }

    fn deriv_ln_saturation_pressure_ip(&self) -> f64 {
        let t_r: f64 = t_rankine_from_t_fahrenheit(self.t_dry_bulb);
        match self.t_dry_bulb >= TRIPLE_POINT_WATER_IP {
            true => {
                -C1_IP / t_r.powi(2)
                    + C3_IP
                    + 2.0 * C4_IP * t_r
                    + 3.0 * C5_IP * t_r.powi(2)
                    + 4.0 * C6_IP * t_r.powi(3)
                    + C7_IP / t_r
            }
            false => {
                -C8_IP / t_r.powi(2)
                    + C10_IP
                    + 2.0 * C11_IP * t_r
                    + 3.0 * C12_IP * t_r.powi(2)
                    + C13_IP / t_r
            }
        }
    }

    fn deriv_ln_saturation_pressure_si(&self) -> f64 {
        let t_k: f64 = t_celsius_to_t_kelvin(self.t_dry_bulb);
        match self.t_dry_bulb >= TRIPLE_POINT_WATER_SI {
            true => {
                -C1_SI / t_k.powi(2)
                    + C3_SI
                    + 2.0 * C4_SI * t_k
                    + 3.0 * C5_SI * t_k.powi(2)
                    + 4.0 * C6_SI * t_k.powi(3)
                    + C7_SI / t_k
            }
            false => {
                -C8_SI / t_k.powi(2)
                    + C10_SI
                    + 2.0 * C11_SI * t_k
                    + 3.0 * C12_SI * t_k.powi(2)
                    + C13_SI / t_k
            }
        }
    }

    pub fn deriv_saturation_pressure_ip(&self) -> f64 {
        f64::exp(self.ln_saturation_pressure_ip()) * self.deriv_ln_saturation_pressure_ip()
    }

    pub fn deriv_saturation_pressure_si(&self) -> f64 {
        f64::exp(self.ln_saturation_pressure_si()) * self.deriv_ln_saturation_pressure_si()
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
    use approx::assert_relative_eq;

    #[test]
    fn test_saturation_pressure_si() {
        let wsat = SaturatedWaterVapor::new(-60.0, UnitSystem::SI);
        assert_abs_diff_eq!(wsat.saturation_pressure(), 1.08, epsilon = 0.01);

        let wsat = SaturatedWaterVapor::new(-20.0, UnitSystem::SI);
        assert_relative_eq!(wsat.saturation_pressure(), 103.24, max_relative = 0.0003);

        let wsat = SaturatedWaterVapor::new(-5.0, UnitSystem::SI);
        assert_relative_eq!(wsat.saturation_pressure(), 401.74, max_relative = 0.0003);

        let wsat = SaturatedWaterVapor::new(5.0, UnitSystem::SI);
        assert_relative_eq!(wsat.saturation_pressure(), 872.6, max_relative = 0.0003);

        let wsat = SaturatedWaterVapor::new(25.0, UnitSystem::SI);
        assert_relative_eq!(wsat.saturation_pressure(), 3169.7, max_relative = 0.0003);

        let wsat = SaturatedWaterVapor::new(50.0, UnitSystem::SI);
        assert_relative_eq!(wsat.saturation_pressure(), 12351.3, max_relative = 0.0003);

        let wsat = SaturatedWaterVapor::new(100.0, UnitSystem::SI);
        assert_relative_eq!(wsat.saturation_pressure(), 101418.0, max_relative = 0.0003);

        let wsat = SaturatedWaterVapor::new(150.0, UnitSystem::SI);
        assert_relative_eq!(wsat.saturation_pressure(), 476101.4, max_relative = 0.0003);
    }

    #[test]
    fn test_saturation_pressure_negative_si() {
        let mut wsat = SaturatedWaterVapor::new(0.0, UnitSystem::SI);
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

    #[test]
    fn test_around_zero() {
        for i in -100..100 {
            let wsat = SaturatedWaterVapor::new(0.1 * i as f64, UnitSystem::SI);
            let p = wsat.saturation_pressure();
            let dp = wsat.deriv_saturation_pressure();
            println!("t = {:.1}, p = {:.2}, dp = {:.2}", 0.1 * i as f64, p, dp);
        }
    }
}
