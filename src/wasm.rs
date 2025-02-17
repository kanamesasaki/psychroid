use crate::common::UnitSystem;
use crate::moist_air::MoistAir;
use wasm_bindgen::prelude::*;

// wasm-pack build --target web --out-dir web/lib

#[wasm_bindgen]
pub struct WasmPoint {
    pub x: f64,
    pub y: f64,
}

/// Generates data points for constant relative humidity line on psychrometric chart
/// <script src="https://cdn.jsdelivr.net/npm/mathjax@3/es5/tex-mml-chtml.js"></script>
///
/// # Arguments
/// * `phi` - Relative humidity [0.0-1.0]
/// * `pressure` - Atmospheric pressure [Pa] (SI) or [Psi] (IP)
/// * `unit` - Unit system (SI or IP)
///
/// # Returns
/// Tuple of vectors (temperatures, humidity ratios):
/// - temperatures: temperature array in \\(^\\circ \\mathrm{C}\\) (SI) or \\(^\\circ \\mathrm{F}\\) (IP)
/// - humidity ratios: corresponding humidity ratio array in \\( \\mathrm{kg_w / kg_{da}} \\) (SI) or \\( \\mathrm{lb_w / lb_{da}} \\) (IP)
///
/// # Example
/// ```
/// use psychroid::{chart, UnitSystem};
///
/// let (temps, hum_ratios) = chart::line_relative_humidity(
///     0.5,      // 50% RH
///     101325.0, // Standard pressure
///     UnitSystem::SI
/// );
/// ```
fn line_relative_humidity(phi: f64, pressure: f64, unit: UnitSystem) -> Vec<WasmPoint> {
    let t_array: Vec<f64> = match unit {
        UnitSystem::SI => (-15..=40).step_by(1).map(|x| x as f64).collect(),
        UnitSystem::IP => (5..=104).step_by(5).map(|x| x as f64).collect(),
    };
    let point_array: Vec<WasmPoint> = t_array
        .iter()
        .map(|&t_dry_bulb| {
            let moist_air =
                MoistAir::from_t_dry_bulb_relative_humidity(t_dry_bulb, phi, pressure, unit);
            WasmPoint {
                x: t_dry_bulb,
                y: moist_air.humidity_ratio(),
            }
        })
        .collect();
    point_array
}

/// Generates data points for constant specific enthalpy line on psychrometric chart
/// <script src="https://cdn.jsdelivr.net/npm/mathjax@3/es5/tex-mml-chtml.js"></script>
///
/// # Arguments
/// * `h` - Specific enthalpy \\( \\mathrm{kJ / kg_{da}} \\) (SI) or \\( \\mathrm{Btu / lb_{da}} \\) (IP)
/// * `unit` - Unit system (SI or IP)
///
/// # Returns
/// Tuple of vectors (temperatures, humidity ratios):
/// - temperatures: temperature array in \\(^\\circ \\mathrm{C}\\) (SI) or \\(^\\circ \\mathrm{F}\\) (IP)
/// - humidity ratios: corresponding humidity ratio array in \\( \\mathrm{kg_w / kg_{da}} \\) (SI) or \\( \\mathrm{lb_w / lb_{da}} \\) (IP)
///
/// # Formula
/// $$
/// \begin{align}
/// \\mathrm{SI:}\\quad W &= \\frac{h - 1.006~t}{2501 + 1.860~t} \\\\
/// \\mathrm{IP:}\\quad W &= \\frac{h - 0.240~t}{1061 + 0.444~t}
/// \end{align}
/// $$
/// where:
/// - \\(t\\) is dry-bulb temperature
/// - \\(h\\) is specific enthalpy
///
/// # Example
/// ```
/// use psychroid::{chart, UnitSystem};
///
/// let (temps, hum_ratios) = chart::line_specific_enthalpy(
///     50.0,           // 50 kJ/kg_da
///     UnitSystem::SI
/// );
/// ```
fn line_specific_enthalpy(h: f64, pressure: f64, unit: UnitSystem) -> Vec<WasmPoint> {
    let t_array: Vec<f64> = match unit {
        UnitSystem::SI => (-15..=40).step_by(1).map(|x| x as f64).collect(),
        UnitSystem::IP => (5..=104).step_by(5).map(|x| x as f64).collect(),
    };
    let point_array: Vec<WasmPoint> = t_array
        .iter()
        .map(|&t_dry_bulb| {
            let moist_air = MoistAir::from_t_dry_bulb_enthalpy(t_dry_bulb, h, pressure, unit);
            WasmPoint {
                x: t_dry_bulb,
                y: moist_air.humidity_ratio(),
            }
        })
        .collect();
    point_array
}

#[wasm_bindgen]
pub fn relative_humidity_lines(phi: f64, pressure: f64, is_si: bool) -> Vec<WasmPoint> {
    let unit = if is_si {
        UnitSystem::SI
    } else {
        UnitSystem::IP
    };
    line_relative_humidity(phi, pressure, unit)
}

#[wasm_bindgen]
pub fn specific_enthalpy_lines(h: f64, pressure: f64, is_si: bool) -> Vec<WasmPoint> {
    let unit = if is_si {
        UnitSystem::SI
    } else {
        UnitSystem::IP
    };
    line_specific_enthalpy(h, pressure, unit)
}

/// A WASM-friendly wrapper around the MoistAir struct.
#[wasm_bindgen]
pub struct WasmMoistAir {
    inner: MoistAir,
}

#[wasm_bindgen]
impl WasmMoistAir {
    /// Creates a new WasmMoistAir instance from dry-bulb temperature and relative humidity.
    ///
    /// Arguments:
    /// * `t_dry_bulb` - Dry-bulb temperature (°C or °F)
    /// * `relative_humidity` - Relative humidity [0.0 - 1.0]
    /// * `pressure` - Atmospheric pressure (Pa for SI, Psi for IP)
    /// * `is_si` - true for SI units, false for IP
    #[wasm_bindgen]
    pub fn fromRelativeHumidity(
        t_dry_bulb: f64,
        relative_humidity: f64,
        pressure: f64,
        is_si: bool,
    ) -> WasmMoistAir {
        let unit = if is_si {
            UnitSystem::SI
        } else {
            UnitSystem::IP
        };
        let inner = MoistAir::from_t_dry_bulb_relative_humidity(
            t_dry_bulb,
            relative_humidity,
            pressure,
            unit,
        );
        WasmMoistAir { inner }
    }

    /// Creates a new WasmMoistAir instance from dry-bulb temperature and humidity ratio.
    ///
    /// Arguments:
    /// * `t_dry_bulb` - Dry-bulb temperature (°C or °F)
    /// * `humidity_ratio` - Humidity Ratio (kg/kg for SI, lb/lb for IP)
    /// * `pressure` - Atmospheric pressure (Pa for SI, Psi for IP)
    /// * `is_si` - true for SI units, false for IP
    #[wasm_bindgen]
    pub fn fromHumidityRatio(
        t_dry_bulb: f64,
        humidity_ratio: f64,
        pressure: f64,
        is_si: bool,
    ) -> WasmMoistAir {
        let unit = if is_si {
            UnitSystem::SI
        } else {
            UnitSystem::IP
        };
        let inner =
            MoistAir::from_t_dry_bulb_humidity_ratio(t_dry_bulb, humidity_ratio, pressure, unit);
        WasmMoistAir { inner }
    }

    /// Creates a new WasmMoistAir instance from dry-bulb temperature and specific enthalpy.
    #[wasm_bindgen]
    pub fn fromSpecificEnthalpy(
        t_dry_bulb: f64,
        specific_enthalpy: f64,
        pressure: f64,
        is_si: bool,
    ) -> WasmMoistAir {
        let unit = if is_si {
            UnitSystem::SI
        } else {
            UnitSystem::IP
        };
        let inner =
            MoistAir::from_t_dry_bulb_enthalpy(t_dry_bulb, specific_enthalpy, pressure, unit);
        WasmMoistAir { inner }
    }

    /// Returns the current wet-bulb temperature.
    #[wasm_bindgen]
    pub fn fromTWetBulb(
        t_dry_bulb: f64,
        t_wet_bulb: f64,
        pressure: f64,
        is_si: bool,
    ) -> WasmMoistAir {
        let unit = if is_si {
            UnitSystem::SI
        } else {
            UnitSystem::IP
        };
        let inner = MoistAir::from_t_dry_bulb_t_wet_bulb(t_dry_bulb, t_wet_bulb, pressure, unit);
        WasmMoistAir { inner }
    }

    /// Returns the current dew-point temperature.
    #[wasm_bindgen]
    pub fn fromTDewPoint(
        t_dry_bulb: f64,
        t_dew_point: f64,
        pressure: f64,
        is_si: bool,
    ) -> WasmMoistAir {
        let unit = if is_si {
            UnitSystem::SI
        } else {
            UnitSystem::IP
        };
        let inner = MoistAir::from_t_dry_bulb_t_dew_point(t_dry_bulb, t_dew_point, pressure, unit);
        WasmMoistAir { inner }
    }

    /// Returns the current dry-bulb temperature.
    #[wasm_bindgen]
    pub fn tDryBulb(&self) -> f64 {
        self.inner.t_dry_bulb()
    }

    /// Returns the current humidity ratio.
    #[wasm_bindgen]
    pub fn humidityRatio(&self) -> f64 {
        self.inner.humidity_ratio()
    }

    /// Returns the specific enthalpy.
    #[wasm_bindgen]
    pub fn specificEnthalpy(&self) -> f64 {
        self.inner.specific_enthalpy()
    }

    /// Returns the relative humidity.
    #[wasm_bindgen]
    pub fn relativeHumidity(&self) -> f64 {
        self.inner.relative_humidity()
    }

    /// Returns the wet-bulb temperature.
    #[wasm_bindgen]
    pub fn tWetBulb(&self) -> f64 {
        self.inner.t_wet_bulb()
    }

    /// Returns the dew-point temperature.
    #[wasm_bindgen]
    pub fn tDewPoint(&self) -> f64 {
        self.inner.t_dew_point()
    }
}
