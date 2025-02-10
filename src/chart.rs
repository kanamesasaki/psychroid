use crate::common::UnitSystem;
use crate::moist_air::MoistAir;

pub fn line_relative_humidity(phi: f64, pressure: f64, unit: UnitSystem) -> (Vec<f64>, Vec<f64>) {
    let t_array: Vec<f64> = (0..=100).step_by(10).map(|x| x as f64).collect();
    let w_array: Vec<f64> = t_array
        .iter()
        .map(|&t_dry_bulb| {
            let moist_air = MoistAir::from_relative_humidity(t_dry_bulb, phi, pressure, unit);
            moist_air.get_humidity_ratio()
        })
        .collect();
    (t_array, w_array)
}

pub fn line_specific_enthalpy(h: f64, unit: UnitSystem) -> (Vec<f64>, Vec<f64>) {
    let t_array: Vec<f64> = (0..=100).step_by(10).map(|x| x as f64).collect();
    let w_array: Vec<f64> = t_array
        .iter()
        .map(|&t_dry_bulb| match unit {
            UnitSystem::SI => (h - 1.006 * t_dry_bulb) / (2501.0 + 1.860 * t_dry_bulb),
            UnitSystem::IP => (h - 1.006 * t_dry_bulb) / (1061.0 + 0.444 * t_dry_bulb),
        })
        .collect();
    (t_array, w_array)
}
