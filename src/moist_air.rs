use crate::common::UnitSystem;
use crate::common::{t_celsius_to_t_fahrenheit, t_fahrenheit_to_t_celsius};
use crate::common::{FREEZING_POINT_WATER_IP, FREEZING_POINT_WATER_SI, MASS_RATIO_WATER_DRY_AIR};
use crate::error::PsychroidError;
use crate::saturated_water_vapor::SaturatedWaterVapor;
use roots::{find_root_newton_raphson, SimpleConvergency};

const C14_SI: f64 = 6.54;
const C15_SI: f64 = 14.526;
const C16_SI: f64 = 0.7389;
const C17_SI: f64 = 0.09486;
const C18_SI: f64 = 0.4569;

const C14_IP: f64 = 100.45;
const C15_IP: f64 = 33.193;
const C16_IP: f64 = 2.319;
const C17_IP: f64 = 0.17074;
const C18_IP: f64 = 1.2063;

const TOLERANCE: f64 = 1e-8;

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
    unit: UnitSystem,
}

/// Create a new instance of MoistAir with default values
impl Default for MoistAir {
    fn default() -> Self {
        let t_dry_bulb = 20.0;
        let relative_humidity = 0.5;
        let pressure = 101325.0;
        let unit = UnitSystem::SI;
        // OK to unwrap because default values are within valid range
        let humidity_ratio =
            humidity_ratio_from_relative_humidity(t_dry_bulb, relative_humidity, pressure, unit)
                .unwrap();
        MoistAir {
            t_dry_bulb,
            humidity_ratio,
            pressure,
            unit,
        }
    }
}

impl MoistAir {
    pub fn from_t_dry_bulb_humidity_ratio(
        t_dry_bulb: f64,
        humidity_ratio: f64,
        pressure: f64,
        unit: UnitSystem,
    ) -> Result<Self, PsychroidError> {
        let relative_humidity: f64 =
            relative_humidity_from_humidity_ratio(t_dry_bulb, humidity_ratio, pressure, unit)?;
        if !(0.0..=1.0 + TOLERANCE).contains(&relative_humidity) {
            return Err(PsychroidError::InvalidRelativeHumidity(relative_humidity));
        }
        Ok(MoistAir {
            t_dry_bulb,
            humidity_ratio,
            pressure,
            unit,
        })
    }

    /// Init from wet bulb temperature
    pub fn from_t_dry_bulb_t_wet_bulb(
        t_dry_bulb: f64,
        t_wet_bulb: f64,
        pressure: f64,
        unit: UnitSystem,
    ) -> Result<Self, PsychroidError> {
        let humidity_ratio =
            humidity_ratio_from_t_wet_bulb(t_dry_bulb, t_wet_bulb, pressure, unit)?;
        Ok(MoistAir {
            t_dry_bulb,
            humidity_ratio,
            pressure,
            unit,
        })
    }

    /// Creates a new MoistAir instance from dry-bulb temperature and relative humidity
    /// ASHRAE Handbook - Fundamentals (2017) Ch. 1-8 SITUATION 3.
    ///
    /// # Arguments
    /// * `t_dry_bulb` - Dry-bulb temperature  \\(^\\circ \\mathrm{C}\\) (SI) or  \\(^\\circ \\mathrm{F}\\) (IP)
    /// * `relative_humidity` - Relative humidity [0.0, 1.0]
    /// * `pressure` - Atmospheric pressure  \\(\\mathrm{Pa}\\) (SI) or  \\(\\mathrm{Psi}\\) (IP)
    /// * `unit` - Unit system (SI or IP)
    pub fn from_t_dry_bulb_relative_humidity(
        t_dry_bulb: f64,
        relative_humidity: f64,
        pressure: f64,
        unit: UnitSystem,
    ) -> Result<Self, PsychroidError> {
        if !(0.0..=1.0 + TOLERANCE).contains(&relative_humidity) {
            return Err(PsychroidError::InvalidRelativeHumidity(relative_humidity));
        }
        let humidity_ratio =
            humidity_ratio_from_relative_humidity(t_dry_bulb, relative_humidity, pressure, unit)?;
        Ok(MoistAir {
            t_dry_bulb,
            humidity_ratio,
            pressure,
            unit,
        })
    }

    /// Creates a new MoistAir instance from dry-bulb and dew-point temperatures
    pub fn from_t_dry_bulb_t_dew_point(
        t_dry_bulb: f64,
        t_dew_point: f64,
        pressure: f64,
        unit: UnitSystem,
    ) -> Result<Self, PsychroidError> {
        let humidity_ratio = humidity_ratio_from_t_dew_point(t_dew_point, pressure, unit)?;
        Ok(MoistAir {
            t_dry_bulb,
            humidity_ratio,
            pressure,
            unit,
        })
    }

    /// Creates a new MoistAir instance from dry-bulb temperature and specific enthalpy
    pub fn from_t_dry_bulb_enthalpy(
        t_dry_bulb: f64,
        specific_enthalpy: f64,
        pressure: f64,
        unit: UnitSystem,
    ) -> Self {
        let humidity_ratio =
            humidity_ratio_from_specific_enthalpy(t_dry_bulb, specific_enthalpy, unit);
        MoistAir {
            t_dry_bulb,
            humidity_ratio,
            pressure,
            unit,
        }
    }

    /// Creates a new MoistAir instance from specific enthalpy and relative humidity
    pub fn from_specific_enthalpy_relative_humidity(
        specific_enthalpy: f64,
        relative_humidity: f64,
        pressure: f64,
        unit: UnitSystem,
    ) -> Result<Self, PsychroidError> {
        let t_dry_bulb = t_dry_bulb_from_specific_enthalpy_relative_humidity(
            specific_enthalpy,
            relative_humidity,
            pressure,
            unit,
        )?;
        let humidity_ratio =
            humidity_ratio_from_specific_enthalpy(t_dry_bulb, specific_enthalpy, unit);
        Ok(MoistAir {
            t_dry_bulb,
            humidity_ratio,
            pressure,
            unit,
        })
    }

    /// Returns the humidity ratio of moist air
    pub fn humidity_ratio(&self) -> f64 {
        self.humidity_ratio
    }

    /// Returns the dry bulb temperature of moist air
    pub fn t_dry_bulb(&self) -> f64 {
        self.t_dry_bulb
    }

    /// Returns the specific enthalpy of moist air
    ///
    /// # Returns
    /// Specific enthalpy \\(h\\), in \\( \\mathrm{kJ/kg_{da}} \\) (SI) or \\( \\mathrm{Btu/lb_{da}} \\) (IP)
    ///
    /// # Formula
    ///
    /// $$
    /// \\begin{align}
    /// h &= 1.006~t_\mathrm{da} + W (2501.0 + 1.860~t_\mathrm{da}) \\quad &\\text{(SI)} \\\\
    /// h &= 0.240~t_\mathrm{da} + W (1061.0 + 0.444~t_\mathrm{da}) \\quad &\\text{(IP)}
    /// \\end{align}
    /// $$
    ///
    /// where:
    /// - \\(t_\mathrm{da}\\) - dry bulb temperature in \\(^\\circ \\mathrm{C}\\) (SI) or \\(^\\circ \\mathrm{F}\\) (IP)
    /// - \\(W\\) - humidity ratio in \\( \\mathrm{kg_w / kg_{da}} \\) (SI) or \\( \\mathrm{lb_w / lb_{da}} \\) (IP)
    ///
    /// Reference: ASHRAE Fundamentals Handbook (2017) Chapter 1
    pub fn specific_enthalpy(&self) -> f64 {
        specific_enthalpy_from_humidity_ratio(self.t_dry_bulb, self.humidity_ratio, self.unit)
    }

    /// Returns the relative humidity of moist air
    ///
    /// # Returns
    /// Relative humidity \\(\\phi\\) [0.0 - 1.0]
    ///
    /// # Formula
    /// $$
    /// \\phi = \\frac{p_\mathrm{w}}{p_\mathrm{ws}} = \\frac{pW}{(0.621945 + W) p_\mathrm{ws}}
    /// $$
    ///
    /// where:
    /// - \\(p_\mathrm{w}\\) - partial pressure of water vapor
    /// - \\(p_\mathrm{ws}\\) - saturation pressure of water vapor
    /// - \\(p\\) - total pressure
    /// - \\(W\\) - humidity ratio (non-dimensional)
    /// - 0.621945 - ratio of molecular mass (non-dimensional) of water vapor to dry air
    ///
    pub fn relative_humidity(&self) -> Result<f64, PsychroidError> {
        let value = relative_humidity_from_humidity_ratio(
            self.t_dry_bulb,
            self.humidity_ratio,
            self.pressure,
            self.unit,
        )?;
        if !(0.0..=1.0 + TOLERANCE).contains(&value) {
            return Err(PsychroidError::InvalidRelativeHumidity(value));
        }
        Ok(value)
    }

    /// Returns the dew point temperature of moist air
    pub fn t_dew_point(&self) -> Result<f64, PsychroidError> {
        t_dew_point_from_humidity_ratio(self.humidity_ratio, self.pressure, self.unit)
    }

    /// Returns the wet bulb temperature of moist air
    pub fn t_wet_bulb(&self) -> Result<f64, PsychroidError> {
        t_wet_bulb_from_humidity_ratio(
            self.t_dry_bulb,
            self.humidity_ratio,
            self.pressure,
            self.unit,
        )
    }

    /// Returns the specific volume of moist air
    ///
    /// # Returns
    /// Specific volume \\(v\\) is the specific volume \\(V/M_{da}\\) in \\( \\mathrm{m^3/kg_{da}} \\) (SI) or \\( \\mathrm{ft^3/lb_{da}} \\) (IP)
    ///
    /// # Formula
    /// $$
    /// \\begin{align}
    /// v = 0.287042 (t_\mathrm{db} + 273.15) (1 + 1.607858 W) / p \\quad &\\text{(SI)} \\\\
    /// v = 0.370486 (t_\mathrm{db} + 459.67) (1 + 1.607858 W) / p \\quad &\\text{(IP)}
    /// \\end{align}
    /// $$
    /// where:
    /// -
    /// - \\(t_\\mathrm{db}\\) - dry bulb temperature in \\(^\\circ \\mathrm{C}\\) (SI) or \\(^\\circ \\mathrm{F}\\) (IP)
    /// - \\(W\\) -  humidity ratio in \\( \\mathrm{kg_w / kg_{da}} \\) (SI) or \\( \\mathrm{lb_w / lb_{da}} \\) (IP)
    /// - \\(p\\) - total pressure in \\( \\mathrm{kPa} \\) (SI) or \\( \\mathrm{Psi} \\) (IP)
    ///
    pub fn density(&self) -> f64 {
        let specific_volume = match self.unit {
            UnitSystem::SI => {
                // specific volume in m³/kg_da, pressure in kPa
                0.287042 * (self.t_dry_bulb + 273.15) * (1.0 + 1.607858 * self.humidity_ratio)
                    / (self.pressure * 0.001)
            }
            UnitSystem::IP => {
                // specific volume in ft³/lb_da
                0.370486 * (self.t_dry_bulb + 459.67) * (1.0 + 1.607858 * self.humidity_ratio)
                    / self.pressure
            }
        };
        1.0 / specific_volume * (1.0 + self.humidity_ratio)
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
    /// This method modifies the dry-bulb temperature of the instance to the target temperature
    ///
    /// # Arguments
    /// * `mda` - Mass flow rate of dry air \\( \\mathrm{kg/s} \\) (SI) or \\( \\mathrm{lb/h} \\) (IP)
    /// * `t1` - Target dry-bulb temperature \\(^\\circ \\mathrm{C}\\)  (SI) or \\(^\\circ \\mathrm{F}\\)  (IP)
    ///
    /// # Returns
    /// Heating energy \\(q\\) required in \\( \\mathrm{kW} \\) (SI) or \\( \\mathrm{Btu/h} \\) (IP)
    ///
    pub fn heating_t1(&mut self, mda: f64, t1: f64) -> f64 {
        let h0 = self.specific_enthalpy();
        self.t_dry_bulb = t1;
        let h1 = self.specific_enthalpy();
        mda * (h1 - h0)
    }

    /// Calculates the heating energy required to change the dry-bulb temperature by a given amount
    /// This method modifies the dry-bulb temperature of the instance to the target temperature
    ///
    /// # Arguments
    /// * `mda` - Mass flow rate of dry air \\( \\mathrm{kg/s} \\) (SI) or \\( \\mathrm{lb/h} \\) (IP)
    /// * `dt` - Temperature change \\(^\\circ \\mathrm{C}\\) (SI) or \\(^\\circ \\mathrm{F}\\) (IP)
    ///
    /// # Returns
    /// Heating energy \\(q\\) required in \\( \\mathrm{kW} \\) (SI) or \\( \\mathrm{Btu/h} \\) (IP)
    ///
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
        let dh = q / mda; // kJ/kg_da (SI) or Btu/lb_da (IP)
        let dt = match self.unit {
            UnitSystem::SI => dh / (1.006 + 1.860 * self.humidity_ratio),
            UnitSystem::IP => dh / (0.240 + 0.444 * self.humidity_ratio),
        };
        // new dry bulb temperature
        self.t_dry_bulb += dt;
    }

    pub fn cooling_t1(&mut self, mda: f64, t1: f64) -> Result<f64, PsychroidError> {
        let t_dew_point = self.t_dew_point()?;
        let h0 = self.specific_enthalpy();
        if t1 < t_dew_point {
            self.humidity_ratio =
                humidity_ratio_from_relative_humidity(t1, 1.0, self.pressure, self.unit)?;
        }
        self.t_dry_bulb = t1;
        let h1 = self.specific_enthalpy();
        Ok(mda * (h0 - h1))
    }

    pub fn cooling_dt(&mut self, mda: f64, dt: f64) -> Result<f64, PsychroidError> {
        let t_dew_point = self.t_dew_point()?;
        let h0 = self.specific_enthalpy();
        let t1 = self.t_dry_bulb - dt;
        if t1 < t_dew_point {
            self.humidity_ratio =
                humidity_ratio_from_relative_humidity(t1, 1.0, self.pressure, self.unit)?;
        }
        self.t_dry_bulb = t1;
        let h1 = self.specific_enthalpy();
        Ok(mda * (h0 - h1))
    }

    /// Calculates the temperature change for a given cooling energy removal
    /// If the new temperature is expected to be below the dew point,
    /// the new dry-bulb temperature is calculated based on the specific enthalpy and relative humidity of 1.0.
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
    /// - \\(\\Delta h = -q/\\dot{m}_{da}\\) -  specific enthalpy change
    /// - \\(W\\) -  humidity ratio
    pub fn cooling_q(&mut self, mda: f64, q: f64) -> Result<(), PsychroidError> {
        let dh = q / mda; // kJ/s
        let h0 = self.specific_enthalpy();
        let h1 = h0 - dh;
        let dt = match self.unit {
            UnitSystem::SI => dh / (1.006 + 1.860 * self.humidity_ratio),
            UnitSystem::IP => dh / (0.240 + 0.444 * self.humidity_ratio),
        };
        let t1 = self.t_dry_bulb - dt;
        let t_dew_point = self.t_dew_point()?;
        if t1 < t_dew_point {
            self.t_dry_bulb = t_dry_bulb_from_specific_enthalpy_relative_humidity(
                h1,
                1.0,
                self.pressure,
                self.unit,
            )?;
            self.humidity_ratio = humidity_ratio_from_relative_humidity(
                self.t_dry_bulb,
                1.0,
                self.pressure,
                self.unit,
            )?;
        } else {
            self.t_dry_bulb = t1;
        }
        Ok(())
    }

    /// Calculates the state change when adding water to moist air (adiabatic humidification).
    /// This method modifies both temperature and humidity ratio of the instance.
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
    pub fn humidify_adiabatic(&mut self, mda: f64, water: f64) -> Result<(), PsychroidError> {
        let w0 = self.humidity_ratio;
        let w1 = w0 + water / mda;

        self.t_dry_bulb = match self.unit {
            UnitSystem::SI => {
                ((1.006 + 1.860 * w0) * self.t_dry_bulb - 2051.0 * (w1 - w0)) / (1.006 + 1.860 * w1)
            }
            UnitSystem::IP => {
                ((0.240 + 0.444 * w0) * self.t_dry_bulb - 1061.0 * (w1 - w0)) / (0.240 + 0.444 * w1)
            }
        };
        self.humidity_ratio = w1;
        self.relative_humidity()?;
        Ok(())
    }

    /// Calculates the state change when adding water to moist air (isothermal humidification)
    ///
    /// # Arguments
    /// * `mda` - Mass flow rate of dry air \\(\\mathrm{kg/s}\\) (SI) or \\(\\mathrm{lb/h}\\) (IP)
    /// * `water` - Mass of water added \\(\\mathrm{kg_w/s}\\) (SI) or \\(\\mathrm{lb_w/h}\\) (IP)
    ///
    /// # Description
    /// Calculates the temperature and humidity ratio changes when water is added to an air stream.
    /// The process is assumed to be isothermal (constant dry-bulb temperature).
    pub fn humidify_isothermal(&mut self, mda: f64, water: f64) -> Result<(), PsychroidError> {
        let w1 = self.humidity_ratio + water / mda;
        self.humidity_ratio = w1;
        self.relative_humidity()?;
        Ok(())
    }

    pub fn cooling_saturation(&mut self, mda: f64) -> Result<f64, PsychroidError> {
        let mut conv = SimpleConvergency {
            eps: 1e-9,
            max_iter: 100,
        };
        let t_saturated = find_root_newton_raphson(
            self.t_dry_bulb,
            |t| {
                let saturated_water = SaturatedWaterVapor::new_relaxed(t, self.unit);
                let pws: f64 = saturated_water.saturation_pressure();
                self.humidity_ratio * (self.pressure - pws) - MASS_RATIO_WATER_DRY_AIR * pws
            },
            |t| {
                let saturated_water = SaturatedWaterVapor::new_relaxed(t, self.unit);
                -(self.humidity_ratio + MASS_RATIO_WATER_DRY_AIR)
                    * saturated_water.deriv_saturation_pressure()
            },
            &mut conv,
        )?;
        let h0 = self.specific_enthalpy();
        self.t_dry_bulb = t_saturated;
        let h1 = self.specific_enthalpy();
        Ok(mda * (h1 - h0))
    }
}

// calculate humidity ratio from dry-bulb and wet-bulb temperatures
fn humidity_ratio_from_t_wet_bulb(
    t_dry_bulb: f64,
    t_wet_bulb: f64,
    pressure: f64,
    unit: UnitSystem,
) -> Result<f64, PsychroidError> {
    let saturated_water_vapor = SaturatedWaterVapor::new(t_wet_bulb, unit)?;
    let saturation_pressure: f64 = saturated_water_vapor.saturation_pressure();
    let saturation_humidity_ratio: f64 =
        MASS_RATIO_WATER_DRY_AIR * saturation_pressure / (pressure - saturation_pressure);
    let humidity_ratio: f64 = match unit {
        UnitSystem::SI => {
            humidity_ratio_from_t_wet_bulb_si(t_dry_bulb, t_wet_bulb, saturation_humidity_ratio)
        }
        UnitSystem::IP => {
            humidity_ratio_from_t_wet_bulb_ip(t_dry_bulb, t_wet_bulb, saturation_humidity_ratio)
        }
    };
    Ok(humidity_ratio)
}

/// ASHRAE Handbook - Fundamentals (2013) IP Ch. 1 Eq. (35) and (37)
fn humidity_ratio_from_t_wet_bulb_ip(
    t_dry_bulb: f64,
    t_wet_bulb: f64,
    saturation_humidity_ratio: f64,
) -> f64 {
    match t_wet_bulb >= FREEZING_POINT_WATER_IP {
        true => {
            ((1093.0 - 0.556 * t_wet_bulb) * saturation_humidity_ratio
                - 0.240 * (t_dry_bulb - t_wet_bulb))
                / (1093.0 + 0.444 * t_dry_bulb - t_wet_bulb)
        }
        false => {
            ((1220.0 - 0.04 * t_wet_bulb) * saturation_humidity_ratio
                - 0.240 * (t_dry_bulb - t_wet_bulb))
                / (1220.0 + 0.444 * t_dry_bulb - 0.48 * t_wet_bulb)
        }
    }
}

/// ASHRAE Handbook - Fundamentals (2017) SI Ch. 1 Eq. (33) and (35)
fn humidity_ratio_from_t_wet_bulb_si(
    t_dry_bulb: f64,
    t_wet_bulb: f64,
    saturation_humidity_ratio: f64,
) -> f64 {
    match t_wet_bulb >= FREEZING_POINT_WATER_SI {
        true => {
            ((2501.0 - 2.326 * t_wet_bulb) * saturation_humidity_ratio
                - 1.006 * (t_dry_bulb - t_wet_bulb))
                / (2501.0 + 1.860 * t_dry_bulb - 4.186 * t_wet_bulb)
        }
        false => {
            ((2830. - 0.24 * t_wet_bulb) * saturation_humidity_ratio
                - 1.006 * (t_dry_bulb - t_wet_bulb))
                / (2830.0 + 1.860 * t_dry_bulb - 2.100 * t_wet_bulb)
        }
    }
}

/// Calculate wet-bulb temperature from dry-bulb temperature and humidity ratio
fn t_wet_bulb_from_humidity_ratio(
    t_dry_bulb: f64,
    humidity_ratio: f64,
    pressure: f64,
    unit: UnitSystem,
) -> Result<f64, PsychroidError> {
    match unit {
        UnitSystem::SI => t_wet_bulb_from_humidity_ratio_si(t_dry_bulb, humidity_ratio, pressure),
        UnitSystem::IP => t_wet_bulb_from_humidity_ratio_ip(t_dry_bulb, humidity_ratio, pressure),
    }
}

/// Calculate wet-bulb temperature from dry-bulb temperature and humidity ratio
/// <script src="https://cdn.jsdelivr.net/npm/mathjax@3/es5/tex-mml-chtml.js"></script>
///
/// # Formula
/// The wet-bulb temperature for given dry-bulb temperature and humidity ratio shall satisfy the equation:
///
/// $$
/// \\begin{align}
/// f &= W (2501 + 1.86t - 4.186t^\*) - (2501 - 2.326t^\*) W_s^* + 1.006(t - t^\*) = 0, \\quad t \\geq 0 \\\\
/// f &= W (2830 + 1.86t - 2.100t^\*) - (2830 - 0.240t^\*) W_s^* + 1.006(t - t^\*) = 0, \\quad t < 0
/// \\end{align}
/// $$
///
/// The corresponding root of this equation is searched using Newton-Raphson method.
/// The derivative of the function is:
///
/// $$
/// \\begin{align}
/// f' &= -4.186W - 2501 \\frac{dW_s^\*}{dt^\*} + 2.326W_s^* + 2.326t^* \\frac{dW_s^\*}{dt^\*} - 1.006, \\quad t \\geq 0 \\\\
/// f' &= -2.100W - 2830 \\frac{dW_s^\*}{dt^\*} + 0.240W_s^* + 0.240t^* \\frac{dW_s^\*}{dt^\*} - 1.006, \\quad t < 0
/// \\end{align}
/// $$
///
fn t_wet_bulb_from_humidity_ratio_si(
    t_dry_bulb: f64,
    humidity_ratio: f64,
    pressure: f64,
) -> Result<f64, PsychroidError> {
    let f = |t_wet_bulb: f64| {
        let saturation_water_vapor = SaturatedWaterVapor::new_relaxed(t_wet_bulb, UnitSystem::SI);
        let saturation_pressure = saturation_water_vapor.saturation_pressure();
        let saturation_humidity_ratio =
            MASS_RATIO_WATER_DRY_AIR * saturation_pressure / (pressure - saturation_pressure);
        match t_wet_bulb >= FREEZING_POINT_WATER_SI {
            true => {
                humidity_ratio * (2501.0 + 1.860 * t_dry_bulb - 4.186 * t_wet_bulb)
                    - (2501.0 - 2.326 * t_wet_bulb) * saturation_humidity_ratio
                    + 1.006 * (t_dry_bulb - t_wet_bulb)
            }
            false => {
                humidity_ratio * (2830.0 + 1.860 * t_dry_bulb - 2.100 * t_wet_bulb)
                    - (2830.0 - 0.240 * t_wet_bulb) * saturation_humidity_ratio
                    + 1.006 * (t_dry_bulb - t_wet_bulb)
            }
        }
    };
    let d = |t_wet_bulb: f64| {
        let saturation_water_vapor = SaturatedWaterVapor::new_relaxed(t_wet_bulb, UnitSystem::SI);
        let saturation_pressure = saturation_water_vapor.saturation_pressure();
        let saturation_humidity_ratio =
            MASS_RATIO_WATER_DRY_AIR * saturation_pressure / (pressure - saturation_pressure);
        let deriv_saturation_humidity_ratio = MASS_RATIO_WATER_DRY_AIR
            * pressure
            * saturation_water_vapor.deriv_saturation_pressure()
            / (pressure - saturation_pressure).powi(2);
        match t_wet_bulb >= FREEZING_POINT_WATER_SI {
            true => {
                -4.186 * humidity_ratio - 2501.0 * deriv_saturation_humidity_ratio
                    + 2.326 * saturation_humidity_ratio
                    + 2.326 * t_wet_bulb * deriv_saturation_humidity_ratio
                    - 1.006
            }
            false => {
                -2.100 * humidity_ratio - 2830.0 * deriv_saturation_humidity_ratio
                    + 0.240 * saturation_humidity_ratio
                    + 0.240 * t_wet_bulb * deriv_saturation_humidity_ratio
                    - 1.006
            }
        }
    };
    let mut convergency = SimpleConvergency {
        eps: 1e-6f64,
        max_iter: 50,
    };
    let root = find_root_newton_raphson(t_dry_bulb, &f, &d, &mut convergency)?;
    Ok(root)
}

/// Calculate wet-bulb temperature from dry-bulb temperature and humidity ratio
/// <script src="https://cdn.jsdelivr.net/npm/mathjax@3/es5/tex-mml-chtml.js"></script>
///
/// # Formula
/// The wet-bulb temperature for given dry-bulb temperature and humidity ratio shall satisfy the equation:
/// $$
/// \\begin{align}
/// f &= W(1093 + 0.444t - 1.00t^\*) - (1093 - 0.556t^\*) W_s^\* + 0.240(t - t^\*) = 0,\\quad t^\* \geq 32 \\\\
/// f &= W(1220 + 0.444t - 0.48t^\*) - (1220 - 0.040t^\*) W_s^\* + 0.240(t - t^\*) = 0,\quad t^\* < 32
/// \\end{align}
/// $$
/// The corresponding root of this equation is searched using Newton-Raphson method.
/// The derivative of the function is:
/// $$
/// \\begin{align}
/// f' = -1.00W - 1093 \frac{dW_s^\*}{dt^\*} + 0.556W_s^\* + 0.556 t^\* \frac{dW_s^\*}{dt^\*} - 0.240, \quad t \geq 0 \\\\
/// f' = -0.48W - 1220 \frac{dW_s^\*}{dt^\*} + 0.040W_s^\* + 0.040 t^\* \frac{dW_s^\*}{dt^\*} - 0.240, \quad t < 0
/// \\end{align}
/// $$
///
fn t_wet_bulb_from_humidity_ratio_ip(
    t_dry_bulb: f64,
    humidity_ratio: f64,
    pressure: f64,
) -> Result<f64, PsychroidError> {
    let f = |t_wet_bulb: f64| {
        let saturation_water_vapor = SaturatedWaterVapor::new_relaxed(t_wet_bulb, UnitSystem::IP);
        let saturation_pressure = saturation_water_vapor.saturation_pressure();
        let saturation_humidity_ratio =
            MASS_RATIO_WATER_DRY_AIR * saturation_pressure / (pressure - saturation_pressure);
        match t_wet_bulb >= FREEZING_POINT_WATER_IP {
            true => {
                humidity_ratio * (1093.0 + 0.444 * t_dry_bulb - t_wet_bulb)
                    - (1093.0 - 0.556 * t_wet_bulb) * saturation_humidity_ratio
                    + 0.240 * (t_dry_bulb - t_wet_bulb)
            }
            false => {
                humidity_ratio * (1220.0 + 0.444 * t_dry_bulb - 0.480 * t_wet_bulb)
                    - (1220.0 - 0.040 * t_wet_bulb) * saturation_humidity_ratio
                    + 0.240 * (t_dry_bulb - t_wet_bulb)
            }
        }
    };

    let d = |t_wet_bulb: f64| {
        let saturation_water_vapor = SaturatedWaterVapor::new_relaxed(t_wet_bulb, UnitSystem::IP);
        let saturation_pressure = saturation_water_vapor.saturation_pressure();
        let saturation_humidity_ratio =
            MASS_RATIO_WATER_DRY_AIR * saturation_pressure / (pressure - saturation_pressure);
        let deriv_saturation_humidity_ratio = MASS_RATIO_WATER_DRY_AIR
            * pressure
            * saturation_water_vapor.deriv_saturation_pressure()
            / (pressure - saturation_pressure).powi(2);

        match t_wet_bulb >= FREEZING_POINT_WATER_IP {
            true => {
                -humidity_ratio - 1093.0 * deriv_saturation_humidity_ratio
                    + 0.556 * saturation_humidity_ratio
                    + 0.556 * t_wet_bulb * deriv_saturation_humidity_ratio
                    - 0.240
            }
            false => {
                -0.480 * humidity_ratio - 1220.0 * deriv_saturation_humidity_ratio
                    + 0.040 * saturation_humidity_ratio
                    + 0.040 * t_wet_bulb * deriv_saturation_humidity_ratio
                    - 0.240
            }
        }
    };

    let mut convergency = SimpleConvergency {
        eps: 1e-6f64,
        max_iter: 50,
    };

    let root = find_root_newton_raphson(t_dry_bulb, &f, &d, &mut convergency)?;
    Ok(root)
}

/// Calculates the humidity ratio from dry-bulb temperature and relative humidity
fn humidity_ratio_from_relative_humidity(
    t_dry_bulb: f64,
    relative_humidity: f64,
    pressure: f64,
    unit: UnitSystem,
) -> Result<f64, PsychroidError> {
    // calculate vapor pressure from relative humidity
    let vapor = SaturatedWaterVapor::new(t_dry_bulb, unit)?;
    let pws = vapor.saturation_pressure();
    let pw = relative_humidity * pws;
    Ok(MASS_RATIO_WATER_DRY_AIR * pw / (pressure - pw))
}

/// Calculates the relative humidity from dry-bulb temperature and humidity ratio
fn relative_humidity_from_humidity_ratio(
    t_dry_bulb: f64,
    humidity_ratio: f64,
    pressure: f64,
    unit: UnitSystem,
) -> Result<f64, PsychroidError> {
    let water_pressure = pressure * humidity_ratio / (MASS_RATIO_WATER_DRY_AIR + humidity_ratio);
    let saturated_water_vapor = SaturatedWaterVapor::new(t_dry_bulb, unit)?;
    Ok(water_pressure / saturated_water_vapor.saturation_pressure())
}

/// Calculate the dew point temperature from dry-bulb temperature and relative humidity
/// If the relative humidity is 0 or very close to 0, NaN is returned as the dew point temperature
fn t_dew_point_from_humidity_ratio(
    humidity_ratio: f64,
    pressure: f64,
    unit: UnitSystem,
) -> Result<f64, PsychroidError> {
    if humidity_ratio <= f64::EPSILON {
        return Ok(f64::NAN);
    }

    let saturation_pressure =
        pressure * humidity_ratio / (MASS_RATIO_WATER_DRY_AIR + humidity_ratio);
    let f = |t: f64| {
        let saturated_water_vapor = SaturatedWaterVapor::new_relaxed(t, unit);
        saturated_water_vapor.saturation_pressure() - saturation_pressure
    };
    let d = |t: f64| {
        let saturated_water_vapor = SaturatedWaterVapor::new_relaxed(t, unit);
        saturated_water_vapor.deriv_saturation_pressure()
    };
    let mut convergency = SimpleConvergency {
        eps: 1e-6f64,
        max_iter: 50,
    };

    let partial_water_vapor_pressure = match unit {
        UnitSystem::SI => {
            // pressure in kPa
            0.001 * humidity_ratio * pressure / (MASS_RATIO_WATER_DRY_AIR + humidity_ratio)
        }
        UnitSystem::IP => humidity_ratio * pressure / (MASS_RATIO_WATER_DRY_AIR + humidity_ratio),
    };

    let alpha = partial_water_vapor_pressure.ln();
    let t_above = match unit {
        UnitSystem::IP => {
            C14_IP
                + C15_IP * alpha
                + C16_IP * alpha.powi(2)
                + C17_IP * alpha.powi(3)
                + C18_IP * partial_water_vapor_pressure.powf(0.1984)
        }
        UnitSystem::SI => {
            C14_SI
                + C15_SI * alpha
                + C16_SI * alpha.powi(2)
                + C17_SI * alpha.powi(3)
                + C18_SI * partial_water_vapor_pressure.powf(0.1984)
        }
    };
    let t_below = match unit {
        UnitSystem::IP => 90.12 + 26.142 * alpha + 0.8927 * alpha.powi(2),
        UnitSystem::SI => 6.09 + 12.608 * alpha + 0.4959 * alpha.powi(2),
    };
    let t_init = match (t_above >= 0.0, t_below >= 0.0) {
        (true, true) => t_above,
        (false, false) => t_below,
        _ => (t_above + t_below) / 2.0,
    };
    let root = find_root_newton_raphson(t_init, &f, &d, &mut convergency)?;
    Ok(root)
}

fn humidity_ratio_from_t_dew_point(
    t_dew_point: f64,
    pressure: f64,
    unit: UnitSystem,
) -> Result<f64, PsychroidError> {
    let saturated_water_vapor = SaturatedWaterVapor::new(t_dew_point, unit)?;
    let saturation_pressure = saturated_water_vapor.saturation_pressure();
    Ok(MASS_RATIO_WATER_DRY_AIR * saturation_pressure / (pressure - saturation_pressure))
}

/// Calculate the specific enthalpy from dry-bulb temperature and humidity ratio
///
/// ASHRAE Handbook - Fundamentals (2017) SI Ch. 1 Eq. (30)
/// ASHRAE Handbook - Fundamentals (2017) IP Ch. 1 Eq. (30)
fn specific_enthalpy_from_humidity_ratio(
    t_dry_bulb: f64,
    humidity_ratio: f64,
    unit: UnitSystem,
) -> f64 {
    match unit {
        UnitSystem::SI => 1.006 * t_dry_bulb + humidity_ratio * (2501.0 + 1.860 * t_dry_bulb),
        UnitSystem::IP => 0.240 * t_dry_bulb + humidity_ratio * (1061.0 + 0.444 * t_dry_bulb),
    }
}

/// Calculate the humidity ratio from specific enthalpy and dry-bulb temperature
///
/// ASHRAE Handbook - Fundamentals (2017) SI Ch. 1 Eq. (30)
/// ASHRAE Handbook - Fundamentals (2017) IP Ch. 1 Eq. (30)
fn humidity_ratio_from_specific_enthalpy(
    t_dry_bulb: f64,
    specific_enthalpy: f64,
    unit: UnitSystem,
) -> f64 {
    match unit {
        UnitSystem::SI => (specific_enthalpy - 1.006 * t_dry_bulb) / (2501.0 + 1.860 * t_dry_bulb),
        UnitSystem::IP => (specific_enthalpy - 0.240 * t_dry_bulb) / (1061.0 + 0.444 * t_dry_bulb),
    }
}

/// Calculate the dry-bulb temperature from specific enthalpy and humidity ratio
///
/// ASHRAE Handbook - Fundamentals (2017) SI Ch. 1 Eq. (30)
/// ASHRAE Handbook - Fundamentals (2017) IP Ch. 1 Eq. (30)
fn t_dry_bulb_from_specific_enthalpy_humidity_ratio(
    specific_enthalpy: f64,
    humidity_ratio: f64,
    unit: UnitSystem,
) -> f64 {
    match unit {
        UnitSystem::SI => {
            (specific_enthalpy - humidity_ratio * 2501.0) / (1.006 + humidity_ratio * 1.860)
        }
        UnitSystem::IP => {
            (specific_enthalpy - humidity_ratio * 1061.0) / (0.240 + humidity_ratio * 0.444)
        }
    }
}

/// Calculate the dry-bulb temperature from specific enthalpy and relative humidity.
/// <script src="https://cdn.jsdelivr.net/npm/mathjax@3/es5/tex-mml-chtml.js"></script>
///
/// # Formula
/// For SI unit system, the dry-bulb temeprature can be determined by solving the equation:
/// $$
/// \begin{equation}
/// h = 1.006 t_{db} + W (2501.0 + 1.860 t_{db}), \quad W = 0.621945 \frac{p_{w}}{p - p_{w}}
/// \end{equation}
/// $$
/// By multiplying the equation by \\(p - p_{w}\\), the equation to solve becomes:
/// $$
/// \begin{gather}
/// f(t_{db}) = (2501.0 \times 0.621945 + h) p_{w} + (1.860 \times 0.621945 - 1.006) t_{db}~p_{w} + 1.006 t_{db}~p - h p = 0
/// \end{gather}
/// $$
/// The derivative of the function is:
/// $$
/// \begin{gather}
/// \frac{df}{dt_{db}} = (2501.0 \times 0.621945 + h) \frac{dp_{w}}{dt_{db}} + (1.860 \times 0.621945 - 1.006) p_{w} + (1.860 \times 0.621945 - 1.006) t_{db} \frac{dp_{w}}{dt_{db}} + 1.006 p
/// \end{gather}
/// $$
///
/// For IP unit system, the dry-bulb temeprature can be determined by solving the equation:
/// $$
/// \begin{equation}
/// h = 0.240 t_{db} + W (1061.0 + 0.444 t_{db}), \quad W = 0.621945 \frac{p_{w}}{p - p_{w}}
/// \end{equation}
/// $$
/// By multiplying the equation by \\(p - p_{w}\\), the equation to solve becomes:
/// $$
/// \begin{gather}
/// f(t_{db}) = (1061.0 \times 0.621945 + h) p_{w} + (0.444 \times 0.621945 - 0.240) t_{db}~p_{w} + 0.240 t_{db}~p - h p = 0
/// \end{gather}
/// $$
/// The derivative of the function is:
/// $$
/// \begin{gather}
/// \frac{df}{dt_{db}} = (1061.0 \times 0.621945 + h) \frac{dp_{w}}{dt_{db}} + (0.444 \times 0.621945 - 0.240) p_{w} + (0.444 \times 0.621945 - 0.240) t_{db} \frac{dp_{w}}{dt_{db}} + 0.240 p
/// \end{gather}
/// $$
///
fn t_dry_bulb_from_specific_enthalpy_relative_humidity(
    specific_enthalpy: f64,
    relative_humidity: f64,
    pressure: f64,
    unit: UnitSystem,
) -> Result<f64, PsychroidError> {
    let f = |t_dry_bulb: f64| {
        let saturation_water_vapor = SaturatedWaterVapor::new_relaxed(t_dry_bulb, unit);
        let partial_water_vapor_pressure =
            relative_humidity * saturation_water_vapor.saturation_pressure();
        match unit {
            UnitSystem::SI => {
                (2501.0 * MASS_RATIO_WATER_DRY_AIR + specific_enthalpy)
                    * partial_water_vapor_pressure
                    + (1.860 * MASS_RATIO_WATER_DRY_AIR - 1.006)
                        * t_dry_bulb
                        * partial_water_vapor_pressure
                    + 1.006 * pressure * t_dry_bulb
                    - specific_enthalpy * pressure
            }
            UnitSystem::IP => {
                (1061.0 * MASS_RATIO_WATER_DRY_AIR + specific_enthalpy)
                    * partial_water_vapor_pressure
                    + (0.444 * MASS_RATIO_WATER_DRY_AIR - 0.240)
                        * t_dry_bulb
                        * partial_water_vapor_pressure
                    + 0.240 * pressure * t_dry_bulb
                    - specific_enthalpy * pressure
            }
        }
    };
    let d = |t_dry_bulb: f64| {
        let saturation_water_vapor = SaturatedWaterVapor::new_relaxed(t_dry_bulb, unit);
        let partial_water_vapor_pressure =
            relative_humidity * saturation_water_vapor.saturation_pressure();
        let deriv_partial_water_vapor_pressure =
            relative_humidity * saturation_water_vapor.deriv_saturation_pressure();
        match unit {
            UnitSystem::SI => {
                (2501.0 * MASS_RATIO_WATER_DRY_AIR + specific_enthalpy)
                    * deriv_partial_water_vapor_pressure
                    + (1.860 * MASS_RATIO_WATER_DRY_AIR - 1.006)
                        * (partial_water_vapor_pressure
                            + t_dry_bulb * deriv_partial_water_vapor_pressure)
                    + 1.006 * pressure
            }
            UnitSystem::IP => {
                (1061.0 * MASS_RATIO_WATER_DRY_AIR + specific_enthalpy)
                    * deriv_partial_water_vapor_pressure
                    + (0.444 * MASS_RATIO_WATER_DRY_AIR - 0.240)
                        * (partial_water_vapor_pressure
                            + t_dry_bulb * deriv_partial_water_vapor_pressure)
                    + 0.240 * pressure
            }
        }
    };
    let mut convergency = SimpleConvergency {
        eps: 1e-6f64,
        max_iter: 50,
    };
    let t_init = specific_enthalpy / 1.006; // humidity_ratio = 0.0
    let root = find_root_newton_raphson(t_init, &f, &d, &mut convergency)?;

    Ok(root)
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;
    use approx::assert_relative_eq;

    #[test]
    fn test_saturated_moist_air_si() {
        let moist_air =
            MoistAir::from_t_dry_bulb_relative_humidity(-50.0, 1.0, 101325.0, UnitSystem::SI)
                .unwrap();
        assert_relative_eq!(moist_air.humidity_ratio(), 0.0000243, max_relative = 0.01);
        assert_relative_eq!(moist_air.specific_enthalpy(), -50.2220, max_relative = 0.01);

        let moist_air =
            MoistAir::from_t_dry_bulb_relative_humidity(-20.0, 1.0, 101325.0, UnitSystem::SI)
                .unwrap();
        assert_relative_eq!(moist_air.humidity_ratio(), 0.0006373, max_relative = 0.01);
        assert_relative_eq!(moist_air.specific_enthalpy(), -18.5420, max_relative = 0.01);

        let moist_air =
            MoistAir::from_t_dry_bulb_relative_humidity(-5.0, 1.0, 101325.0, UnitSystem::SI)
                .unwrap();
        assert_relative_eq!(moist_air.humidity_ratio(), 0.0024863, max_relative = 0.005);
        assert_relative_eq!(moist_air.specific_enthalpy(), 1.164, max_relative = 0.03);

        let moist_air =
            MoistAir::from_t_dry_bulb_relative_humidity(5.0, 1.0, 101325.0, UnitSystem::SI)
                .unwrap();
        assert_relative_eq!(moist_air.humidity_ratio(), 0.005425, max_relative = 0.005);
        assert_relative_eq!(moist_air.specific_enthalpy(), 18.639, max_relative = 0.01);

        let moist_air =
            MoistAir::from_t_dry_bulb_relative_humidity(25.0, 1.0, 101325.0, UnitSystem::SI)
                .unwrap();
        assert_relative_eq!(moist_air.humidity_ratio(), 0.020173, max_relative = 0.005);
        assert_relative_eq!(moist_air.specific_enthalpy(), 76.504, max_relative = 0.01);

        let moist_air =
            MoistAir::from_t_dry_bulb_relative_humidity(50.0, 1.0, 101325.0, UnitSystem::SI)
                .unwrap();
        assert_relative_eq!(moist_air.humidity_ratio(), 0.086863, max_relative = 0.01);
        assert_relative_eq!(moist_air.specific_enthalpy(), 275.353, max_relative = 0.01);

        let moist_air =
            MoistAir::from_t_dry_bulb_relative_humidity(85.0, 1.0, 101325.0, UnitSystem::SI)
                .unwrap();
        assert_relative_eq!(moist_air.humidity_ratio(), 0.838105, max_relative = 0.02);
        assert_relative_eq!(moist_air.specific_enthalpy(), 2307.539, max_relative = 0.01);
    }

    #[test]
    fn test_saturated_moist_air_ip() {
        let moist_air =
            MoistAir::from_t_dry_bulb_relative_humidity(-58.0, 1.0, 14.696, UnitSystem::IP)
                .unwrap();
        assert_relative_eq!(moist_air.humidity_ratio(), 0.0000243, max_relative = 0.01);
        assert_relative_eq!(moist_air.specific_enthalpy(), -13.906, max_relative = 0.01);

        let moist_air =
            MoistAir::from_t_dry_bulb_relative_humidity(-4.0, 1.0, 14.696, UnitSystem::IP).unwrap();
        assert_relative_eq!(moist_air.humidity_ratio(), 0.0006373, max_relative = 0.01);
        assert_relative_eq!(moist_air.specific_enthalpy(), -0.286, max_relative = 0.01);

        let moist_air =
            MoistAir::from_t_dry_bulb_relative_humidity(23.0, 1.0, 14.696, UnitSystem::IP).unwrap();
        assert_relative_eq!(moist_air.humidity_ratio(), 0.0024863, max_relative = 0.005);
        assert_relative_eq!(moist_air.specific_enthalpy(), 8.186, max_relative = 0.01);

        let moist_air =
            MoistAir::from_t_dry_bulb_relative_humidity(41.0, 1.0, 14.696, UnitSystem::IP).unwrap();
        assert_relative_eq!(moist_air.humidity_ratio(), 0.005425, max_relative = 0.005);
        assert_relative_eq!(moist_air.specific_enthalpy(), 15.699, max_relative = 0.01);

        let moist_air =
            MoistAir::from_t_dry_bulb_relative_humidity(77.0, 1.0, 14.696, UnitSystem::IP).unwrap();
        assert_relative_eq!(moist_air.humidity_ratio(), 0.020173, max_relative = 0.005);
        assert_relative_eq!(moist_air.specific_enthalpy(), 40.576, max_relative = 0.01);

        let moist_air =
            MoistAir::from_t_dry_bulb_relative_humidity(122.0, 1.0, 14.696, UnitSystem::IP)
                .unwrap();
        assert_relative_eq!(moist_air.humidity_ratio(), 0.086863, max_relative = 0.01);
        assert_relative_eq!(moist_air.specific_enthalpy(), 126.066, max_relative = 0.01);

        let moist_air =
            MoistAir::from_t_dry_bulb_relative_humidity(185.0, 1.0, 14.696, UnitSystem::IP)
                .unwrap();
        assert_relative_eq!(moist_air.humidity_ratio(), 0.838105, max_relative = 0.015);
        assert_relative_eq!(moist_air.specific_enthalpy(), 999.749, max_relative = 0.01);
    }

    #[test]
    fn test_relative_humidity_humidity_ratio() {
        let relative_humidity_array = [0.0, 0.2, 0.4, 0.6, 0.8, 1.0];
        // SI units
        relative_humidity_array.iter().for_each(|&rh| {
            let moist_air =
                MoistAir::from_t_dry_bulb_relative_humidity(25.0, rh, 101325.0, UnitSystem::SI)
                    .unwrap();
            let humidity_ratio = moist_air.humidity_ratio();
            let moist_air = MoistAir::from_t_dry_bulb_humidity_ratio(
                25.0,
                humidity_ratio,
                101325.0,
                UnitSystem::SI,
            )
            .unwrap();
            assert_abs_diff_eq!(moist_air.relative_humidity().unwrap(), rh, epsilon = 1.0E-8);
        });
        // IP units
        relative_humidity_array.iter().for_each(|&rh| {
            let moist_air =
                MoistAir::from_t_dry_bulb_relative_humidity(77.0, rh, 14.696, UnitSystem::IP)
                    .unwrap();
            let humidity_ratio = moist_air.humidity_ratio();
            let moist_air = MoistAir::from_t_dry_bulb_humidity_ratio(
                77.0,
                humidity_ratio,
                14.696,
                UnitSystem::IP,
            )
            .unwrap();
            assert_abs_diff_eq!(moist_air.relative_humidity().unwrap(), rh, epsilon = 1.0E-8);
        });
    }

    #[test]
    fn test_example1_si() {
        // Test case from ASHRAE Handbook - Fundamentals (2017) SI Ch. 1, Example 1
        let moist_air =
            MoistAir::from_t_dry_bulb_t_wet_bulb(40.0, 20.0, 101325.0, UnitSystem::SI).unwrap();
        assert_relative_eq!(moist_air.humidity_ratio(), 0.0065, max_relative = 0.02);
        assert_relative_eq!(moist_air.specific_enthalpy(), 56.7, max_relative = 0.01);
        assert_relative_eq!(moist_air.t_dew_point().unwrap(), 7.0, max_relative = 0.06);
        assert_relative_eq!(
            moist_air.relative_humidity().unwrap(),
            0.14,
            max_relative = 0.01
        );
    }

    #[test]
    fn test_example1_ip() {
        // Test case from ASHRAE Handbook - Fundamentals (2017) IP Ch. 1, Example 1
        let moist_air =
            MoistAir::from_t_dry_bulb_t_wet_bulb(100.0, 65.0, 14.696, UnitSystem::IP).unwrap();
        assert_relative_eq!(moist_air.humidity_ratio(), 0.00523, max_relative = 0.01);
        assert_relative_eq!(moist_air.specific_enthalpy(), 29.80, max_relative = 0.01);
        assert_relative_eq!(moist_air.t_dew_point().unwrap(), 40.0, max_relative = 0.01);
        assert_relative_eq!(
            moist_air.relative_humidity().unwrap(),
            0.13,
            max_relative = 0.02
        );
    }

    #[test]
    fn test_example2_si() {
        // Test case from ASHRAE Handbook - Fundamentals (2017) SI Ch. 1, Example 2
        let mut moist_air =
            MoistAir::from_t_dry_bulb_relative_humidity(2.0, 1.0, 101325.0, UnitSystem::SI)
                .unwrap();
        let volumetric_flow_rate = 10.0; // m3/s
        let mass_flow_rate =
            volumetric_flow_rate * moist_air.density() / (1.0 + moist_air.humidity_ratio());
        assert_relative_eq!(mass_flow_rate, 12.74, max_relative = 0.00001);
        let q = moist_air.heating_t1(mass_flow_rate, 40.0);
        assert_relative_eq!(q, 490.0, max_relative = 0.002);
    }

    #[test]
    fn test_example2_ip() {
        // Test case from ASHRAE Handbook - Fundamentals (2017) IP Ch. 1, Example 2
        let mut moist_air =
            MoistAir::from_t_dry_bulb_relative_humidity(35.0, 1.0, 14.696, UnitSystem::IP).unwrap();
        let volumetric_flow_rate = 20000.0; // cfm
        let mass_flow_rate =
            volumetric_flow_rate * 60.0 * moist_air.density() / (1.0 + moist_air.humidity_ratio());
        assert_relative_eq!(mass_flow_rate, 95620.0, max_relative = 0.0006);
        let q = moist_air.heating_t1(mass_flow_rate, 100.0);
        assert_relative_eq!(q, 1507000.0, max_relative = 0.003);
    }

    #[test]
    fn test_relative_humidity_100_si() {
        let t_dry_bulb: Vec<f64> = (-100..=-5).step_by(5).map(|x| x as f64).collect();
        let unit = UnitSystem::SI;

        t_dry_bulb.iter().for_each(|&t| {
            let moist_air =
                MoistAir::from_t_dry_bulb_relative_humidity(t, 1.0, 101325.0, unit).unwrap();
            assert_relative_eq!(moist_air.t_dew_point().unwrap(), t, max_relative = 5.0E-5);
            assert_relative_eq!(moist_air.t_wet_bulb().unwrap(), t, max_relative = 5.0E-5);
        });

        let t_dry_bulb: Vec<f64> = (5..=195).step_by(5).map(|x| x as f64).collect();
        let unit = UnitSystem::SI;

        t_dry_bulb.iter().for_each(|&t| {
            let moist_air =
                MoistAir::from_t_dry_bulb_relative_humidity(t, 1.0, 101325.0, unit).unwrap();
            assert_relative_eq!(moist_air.t_dew_point().unwrap(), t, max_relative = 5.0E-5);
            assert_relative_eq!(moist_air.t_wet_bulb().unwrap(), t, max_relative = 5.0E-5);
        });

        let moist_air =
            MoistAir::from_t_dry_bulb_relative_humidity(0.0, 1.0, 101325.0, unit).unwrap();
        assert_abs_diff_eq!(moist_air.t_dew_point().unwrap(), 0.0, epsilon = 1.0E-8);
        assert_abs_diff_eq!(moist_air.t_wet_bulb().unwrap(), 0.0, epsilon = 1.0E-8);
    }
}
