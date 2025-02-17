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
        UnitSystem::SI => (-15..=40).step_by(5).map(|x| x as f64).collect(),
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
        UnitSystem::SI => (-15..=40).step_by(5).map(|x| x as f64).collect(),
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
pub fn get_relative_humidity_line(phi: f64, pressure: f64, is_si: bool) -> Vec<WasmPoint> {
    let unit = if is_si {
        UnitSystem::SI
    } else {
        UnitSystem::IP
    };
    line_relative_humidity(phi, pressure, unit)
}

#[wasm_bindgen]
pub fn get_specific_enthalpy_line(h: f64, pressure: f64, is_si: bool) -> Vec<WasmPoint> {
    let unit = if is_si {
        UnitSystem::SI
    } else {
        UnitSystem::IP
    };
    line_specific_enthalpy(h, pressure, unit)
}
