use crate::common::UnitSystem;
use crate::moist_air::MoistAir;
use wasm_bindgen::prelude::*;

// cargo install wasm-pack
// wasm-pack build --target web

#[wasm_bindgen]
pub struct PsychroidWasm {
    moist_air: MoistAir,
}

#[wasm_bindgen]
impl PsychroidWasm {
    #[wasm_bindgen(constructor)]
    pub fn new(t_dry_bulb: f64, humidity_ratio: f64, pressure: f64, is_si: bool) -> Self {
        let unit = if is_si {
            UnitSystem::SI
        } else {
            UnitSystem::IP
        };
        let moist_air = MoistAir::new(t_dry_bulb, humidity_ratio, pressure, unit);
        PsychroidWasm { moist_air }
    }

    #[wasm_bindgen]
    pub fn specific_enthalpy(&self) -> f64 {
        self.moist_air.specific_enthalpy()
    }
}
