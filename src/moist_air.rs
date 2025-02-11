use crate::common::UnitSystem;
use crate::common::{t_celsius_to_t_fahrenheit, t_fahrenheit_to_t_celsius};
use crate::common::{FREEZING_POINT_WATER_IP, FREEZING_POINT_WATER_SI, MASS_RATIO_WATER_DRY_AIR};
use crate::saturated_water::SaturatedWater;
use roots::{find_root_newton_raphson, SimpleConvergency};

////////////////////////////////////////////////////////////////////////////////////////////////////////
// Moist Air
////////////////////////////////////////////////////////////////////////////////////////////////////////

/// Contains all calculated psychrometric values
/// <script src="https://cdn.jsdelivr.net/npm/mathjax@3/es5/tex-mml-chtml.js"></script>
#[derive(Debug)]
pub struct MoistAir {
    t_dry_bulb: f64,     // °C (SI) or °F (IP)
    humidity_ratio: f64, // kg_H₂O/kg_Air (SI) or lb_H₂O/lb_Air (IP)
    pressure: f64,       // Pa (SI) or Psi (IP)
    unit: UnitSystem,    // SI or IP
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
        if humidity_ratio < 0.0 {
            panic!("Humidity ratio must be positive");
        }
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

    pub fn get_humidity_ratio(&self) -> f64 {
        self.humidity_ratio
    }

    /// Creates a new MoistAir instance from dry-bulb temperature, relative humidity, and pressure \\
    /// ASHRAE Handbook - Fundamentals (2017) Ch. 1-8 SITUATION 3.
    ///
    /// # Arguments
    /// * `t_dry_bulb` - Dry-bulb temperature  \\(^\\circ \\mathrm{C}\\) (SI) or  \\(^\\circ \\mathrm{F}\\) (IP)
    /// * `relative_humidity` - Relative humidity [0.0, 1.0]
    /// * `pressure` - Atmospheric pressure  \\(\\mathrm{Pa}\\) (SI) or  \\(\\mathrm{Psi}\\) (IP)
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
        if !(0.0..=1.0).contains(&relative_humidity) {
            panic!("Relative humidity must be between 0 and 1");
        }
        let humidity_ratio =
            humidity_ratio_from_relative_humidity(t_dry_bulb, relative_humidity, pressure, unit);
        MoistAir {
            t_dry_bulb,
            humidity_ratio,
            pressure,
            unit,
        }
    }

    /// Calculates the specific enthalpy of moist air
    ///
    /// # Returns
    /// The specific enthalpy \\(h\\):
    /// - \\( \\mathrm{kJ/kg_{da}} \\) for SI units
    /// - \\( \\mathrm{Btu/lb_{da}} \\) for IP units
    ///
    /// # Formula
    ///
    /// $$
    /// \\begin{align}
    /// \\mathrm{SI~units:}\\quad h &= 1.006~t + W (2501.0 + 1.86~t) \\\\
    /// \\mathrm{IP~units:}\\quad h &= 0.240~t + W (1061.0 + 0.444~t)
    /// \\end{align}
    /// $$
    ///
    /// where:
    /// - \\(t\\) is the dry bulb temperature in \\(^\\circ \\mathrm{C}\\) or \\(^\\circ \\mathrm{F}\\)
    /// - \\(W\\) is the humidity ratio in \\( \\mathrm{kg_w / kg_{da}} \\) or \\( \\mathrm{lb_w / lb_{da}} \\)
    ///
    /// Reference: ASHRAE Fundamentals Handbook (2017) Chapter 1
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

    /// Calculates the relative humidity from humidity ratio and pressure
    ///
    /// # Returns
    /// Relative humidity [0-1]
    ///
    /// # Formula
    /// $$
    /// \\phi = \\frac{p_w}{p_{ws}} = \\frac{p \\cdot W}{(0.621945 + W) \\cdot p_{ws}}
    /// $$
    /// where:
    /// - \\(\\phi\\) is relative humidity
    /// - \\(p_w\\) is partial pressure of water vapor
    /// - \\(p_{ws}\\) is saturation pressure of water vapor
    /// - \\(p\\) is total pressure
    /// - \\(W\\) is humidity ratio
    /// - 0.621945 is the ratio of molecular mass of water vapor to dry air
    ///
    /// # Example
    /// ```
    /// use psychroid::{MoistAir, UnitSystem};
    ///
    /// let air = MoistAir::new(
    ///     25.0,     // 25°C
    ///     0.007,    // humidity ratio
    ///     101325.0, // Pa
    ///     UnitSystem::SI
    /// );
    ///
    /// let rh = air.relative_humidity();
    /// assert!(rh >= 0.0 && rh <= 1.0);
    /// ```
    ///
    /// Reference: ASHRAE Fundamentals Handbook (2017) Chapter 1
    pub fn relative_humidity(&self) -> f64 {
        let water_pressure =
            self.pressure * self.humidity_ratio / (MASS_RATIO_WATER_DRY_AIR + self.humidity_ratio);
        let saturated_water = SaturatedWater::new(self.t_dry_bulb, self.unit);
        water_pressure / saturated_water.saturation_pressure()
    }

    /// Changes the unit system and converts all properties to the new unit system
    ///
    /// # Arguments
    /// * `unit` - The new unit system to convert to (SI or IP)
    ///
    /// # Conversions performed
    /// - Temperature: \\(^\\circ \\mathrm{F}\\) ↔ \\(^\\circ \\mathrm{C}\\)
    /// - Pressure: \\(\\mathrm{Psi}\\) ↔ \\(\\mathrm{Pa}\\)
    /// - Humidity ratio remains dimensionless
    ///
    /// # Example
    /// ```
    /// use psychroid::{MoistAir, UnitSystem};
    ///
    /// let mut air = MoistAir::new(
    ///     25.0,     // 25°C
    ///     0.007,    // humidity ratio
    ///     101325.0, // Pa
    ///     UnitSystem::SI
    /// );
    ///
    /// // Convert to IP units
    /// air.set_unit(UnitSystem::IP);
    /// // Now temperature is in °F, pressure in Psi
    /// ```
    pub fn set_unit(&mut self, unit: UnitSystem) {
        if self.unit != unit {
            self.unit = unit;
            self.t_dry_bulb = match unit {
                UnitSystem::SI => t_fahrenheit_to_t_celsius(self.t_dry_bulb),
                UnitSystem::IP => t_celsius_to_t_fahrenheit(self.t_dry_bulb),
            };
            self.pressure = match unit {
                UnitSystem::SI => self.pressure * 6894.75729, // Psi to Pa
                UnitSystem::IP => self.pressure / 6894.75729, // Pa to Psi
            };
        }
    }

    /// Calculates the heating energy required to change the dry-bulb temperature to a target temperature
    ///
    /// # Arguments
    /// * `mda` - Mass flow rate of dry air \\( \\mathrm{kg/s} \\) (SI) or \\( \\mathrm{lb/h} \\) (IP)
    /// * `t1` - Target dry-bulb temperature \\(^\\circ \\mathrm{C}\\)  (SI) or \\(^\\circ \\mathrm{F}\\)  (IP)
    ///
    /// # Returns
    /// The heating energy required:
    /// * \\(q~\\mathrm{kW}\\) for SI units
    /// * \\(q~\\mathrm{Btu/h}\\) for IP units
    ///
    /// # Example
    /// ```
    /// use psychroid::{MoistAir, UnitSystem};
    ///
    /// let mut air = MoistAir::new(
    ///     20.0,     // Initial temperature: 20°C
    ///     0.007,    // Humidity ratio
    ///     101325.0, // Pressure: 101.325 kPa
    ///     UnitSystem::SI
    /// );
    ///
    /// // Calculate energy required to heat air to 25°C with 1.0 kg/s flow rate
    /// let heating_energy = air.heating_t1(1.0, 25.0);
    /// ```
    ///
    /// # Note
    /// This method modifies the dry-bulb temperature of the instance to the target temperature
    pub fn heating_t1(&mut self, mda: f64, t1: f64) -> f64 {
        let h0 = self.specific_enthalpy();
        self.t_dry_bulb = t1;
        let h1 = self.specific_enthalpy();
        mda * (h1 - h0)
    }

    pub fn heating_dt(&mut self, mda: f64, dt: f64) -> f64 {
        let h0 = self.specific_enthalpy();
        self.t_dry_bulb += dt;
        let h1 = self.specific_enthalpy();
        mda * (h1 - h0)
    }

    /// Calculates the temperature change for a given heating energy input
    ///
    /// # Arguments
    /// * `mda` - Mass flow rate of dry air \\( \\mathrm{kg/s} \\) (SI) or \\( \\mathrm{lb/h} \\) (IP)
    /// * `q` - Heating energy input \\( \\mathrm{kW} \\) (SI) or \\( \\mathrm{Btu/h} \\) (IP)
    ///
    /// # Returns
    /// Estimated new dry-bulb temperature \\(^\\circ \\mathrm{C}\\) (SI) or \\(^\\circ \\mathrm{F}\\) (IP)
    ///
    /// # Formula
    /// Temperature change is estimated using:
    /// $$
    /// \begin{align}
    /// \\Delta T &= \\frac{\\Delta h}{1.006 + 1.860 W} \\quad &\\text{(SI)} \\\\
    /// \\Delta T &= \\frac{\\Delta h}{0.240 + 0.444 W} \\quad &\\text{(IP)}
    /// \end{align}
    /// $$
    /// where:
    /// - \\(\\Delta h = q/\\dot{m}_{da}\\) is the specific enthalpy change
    /// - \\(W\\) is the humidity ratio
    ///
    /// # Note
    /// This provides an initial estimate and may need iteration for precise results
    pub fn heating_q(&mut self, mda: f64, q: f64) {
        let dh = q / mda;
        let dt = match self.unit {
            UnitSystem::SI => dh / (1.006 + 1.860 * self.humidity_ratio),
            UnitSystem::IP => dh / (0.240 + 0.444 * self.humidity_ratio),
        };
        // new dry bulb temperature
        self.t_dry_bulb += dt;
    }

    /// Calculates the state change when adding water to moist air (adiabatic humidification)
    ///
    /// # Arguments
    /// * `mda` - Mass flow rate of dry air \\(\\mathrm{kg/s}\\) (SI) or \\(\\mathrm{lb/h}\\) (IP)
    /// * `water` - Mass of water added \\(\\mathrm{kg_w/s}\\) (SI) or \\(\\mathrm{lb_w/h}\\) (IP)
    ///
    /// # Description
    /// Calculates the temperature and humidity ratio changes when water is added to an air stream.
    /// The process is assumed to be adiabatic (constant enthalpy).
    ///
    /// # Formula
    /// It is assumed that the process is adiabatic and the enthalpy remains constant.
    /// Based on this assumption, the temperature after humidification is calculated by
    /// $$
    /// \begin{align}
    /// T_1 &= \frac{(1.006 + 1.860~W_0)~T_0 - 2501.0 (W_1 - W_0)}{1.006 + 1.860~W_1} \quad &\text{(SI)} \\\\
    /// T_1 &= \frac{(0.240 + 0.444~W_0)~T_0 - 1061.0 (W_1 - W_0)}{0.240 + 0.444~W_1} \quad &\text{(IP)}
    /// \end{align}
    /// $$
    /// where:
    /// - \\(T_0,~T_1\\) are initial and final temperatures
    /// - \\(W_0,~W_1\\) are initial and final humidity ratios
    ///
    /// # Note
    /// This method modifies both temperature and humidity ratio of the instance
    pub fn humidify(&mut self, mda: f64, water: f64) {
        let w0 = self.humidity_ratio;
        let w1 = self.humidity_ratio + water / mda;
        self.t_dry_bulb = match self.unit {
            UnitSystem::SI => {
                ((1.006 + 1.860 * w0) * self.t_dry_bulb - 2051.0 * (w1 - w0)) / (1.006 + 1.860 * w1)
            }
            UnitSystem::IP => {
                ((0.240 + 0.444 * w0) * self.t_dry_bulb - 1061.0 * (w1 - w0)) / (0.240 + 0.444 * w1)
            }
        };
        self.humidity_ratio = w1;
    }

    pub fn cooling_saturation(&mut self, mda: f64) -> f64 {
        let mut conv = SimpleConvergency {
            eps: 1e-9,
            max_iter: 100,
        };
        let t_saturated = find_root_newton_raphson(
            self.t_dry_bulb,
            |t| {
                let mut saturated_water = SaturatedWater::new(self.t_dry_bulb, self.unit);
                saturated_water.t_dry_bulb = t;
                let pws: f64 = saturated_water.saturation_pressure();
                self.humidity_ratio * (self.pressure - pws) - MASS_RATIO_WATER_DRY_AIR * pws
            },
            |t| {
                let mut saturated_water = SaturatedWater::new(self.t_dry_bulb, self.unit);
                saturated_water.t_dry_bulb = t;
                -(self.humidity_ratio + MASS_RATIO_WATER_DRY_AIR)
                    * saturated_water.deriv_saturation_pressure()
            },
            &mut conv,
        )
        .unwrap();
        let h0 = self.specific_enthalpy();
        self.t_dry_bulb = t_saturated;
        let h1 = self.specific_enthalpy();
        mda * (h1 - h0)
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
    humidity_ratio
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
                / (2501.0 + 1.860 * t_dry_bulb - 4.186 * t_wet_bulb)
        }
        false => {
            ((2830. - 0.24 * t_wet_bulb) * wsstar - 1.006 * (t_dry_bulb - t_wet_bulb))
                / (2830.0 + 1.860 * t_dry_bulb - 2.100 * t_wet_bulb)
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
