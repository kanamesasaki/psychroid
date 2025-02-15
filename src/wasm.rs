use crate::chart;
use crate::common::UnitSystem;
use wasm_bindgen::prelude::*;

// wasm-pack build --target web --out-dir web/lib

#[wasm_bindgen]
pub struct ChartData {
    temperatures: Vec<f64>,
    humidity_ratios: Vec<f64>,
}

#[wasm_bindgen]
impl ChartData {
    #[wasm_bindgen(getter)]
    pub fn temperatures(&self) -> Vec<f64> {
        self.temperatures.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn humidity_ratios(&self) -> Vec<f64> {
        self.humidity_ratios.clone()
    }
}

#[wasm_bindgen]
pub fn get_relative_humidity_line(phi: f64, pressure: f64, is_si: bool) -> ChartData {
    let unit = if is_si {
        UnitSystem::SI
    } else {
        UnitSystem::IP
    };
    let (temps, ratios) = chart::line_relative_humidity(phi, pressure, unit);

    ChartData {
        temperatures: temps,
        humidity_ratios: ratios,
    }
}
