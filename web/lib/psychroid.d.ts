/* tslint:disable */
/* eslint-disable */
/**
 * Generates data points for constant relative humidity line on psychrometric chart
 * <script src="https://cdn.jsdelivr.net/npm/mathjax@3/es5/tex-mml-chtml.js"></script>
 *
 * # Arguments
 * * `phi` - Relative humidity [0.0-1.0]
 * * `pressure` - Atmospheric pressure [Pa] (SI) or [Psi] (IP)
 * * `unit` - Unit system (SI or IP)
 *
 * # Returns
 * Tuple of vectors (temperatures, humidity ratios):
 * - temperatures: temperature array in \\(^\\circ \\mathrm{C}\\) (SI) or \\(^\\circ \\mathrm{F}\\) (IP)
 * - humidity ratios: corresponding humidity ratio array in \\( \\mathrm{kg_w / kg_{da}} \\) (SI) or \\( \\mathrm{lb_w / lb_{da}} \\) (IP)
 */
export function relativeHumidityLine(phi: number, pressure: number, t_min: number, t_max: number, is_si: boolean): WasmPoint[];
/**
 * Generates data points for constant specific enthalpy line on psychrometric chart
 * <script src="https://cdn.jsdelivr.net/npm/mathjax@3/es5/tex-mml-chtml.js"></script>
 *
 * # Arguments
 * * `h` - Specific enthalpy \\( \\mathrm{kJ / kg_{da}} \\) (SI) or \\( \\mathrm{Btu / lb_{da}} \\) (IP)
 * * `unit` - Unit system (SI or IP)
 *
 * # Returns
 * Tuple of vectors (temperatures, humidity ratios):
 * - temperatures: temperature array in \\(^\\circ \\mathrm{C}\\) (SI) or \\(^\\circ \\mathrm{F}\\) (IP)
 * - humidity ratios: corresponding humidity ratio array in \\( \\mathrm{kg_w / kg_{da}} \\) (SI) or \\( \\mathrm{lb_w / lb_{da}} \\) (IP)
 *
 * # Formula
 * $$
 * \begin{align}
 * \\mathrm{SI:}\\quad W &= \\frac{h - 1.006~t}{2501 + 1.860~t} \\\\
 * \\mathrm{IP:}\\quad W &= \\frac{h - 0.240~t}{1061 + 0.444~t}
 * \end{align}
 * $$
 * where:
 * - \\(t\\) is dry-bulb temperature
 * - \\(h\\) is specific enthalpy
 */
export function specificEnthalpyLine(h: number, pressure: number, t_min: number, t_max: number, is_si: boolean): WasmPoint[];
/**
 * A WASM-friendly wrapper around the MoistAir struct.
 */
export class WasmMoistAir {
  private constructor();
  free(): void;
  /**
   * Creates a new WasmMoistAir instance from dry-bulb temperature and relative humidity.
   *
   * Arguments:
   * * `t_dry_bulb` - Dry-bulb temperature (°C or °F)
   * * `relative_humidity` - Relative humidity [0.0 - 1.0]
   * * `pressure` - Atmospheric pressure (Pa for SI, Psi for IP)
   * * `is_si` - true for SI units, false for IP
   */
  static fromRelativeHumidity(t_dry_bulb: number, relative_humidity: number, pressure: number, is_si: boolean): WasmMoistAir;
  /**
   * Creates a new WasmMoistAir instance from dry-bulb temperature and humidity ratio.
   *
   * Arguments:
   * * `t_dry_bulb` - Dry-bulb temperature (°C or °F)
   * * `humidity_ratio` - Humidity Ratio (kg/kg for SI, lb/lb for IP)
   * * `pressure` - Atmospheric pressure (Pa for SI, Psi for IP)
   * * `is_si` - true for SI units, false for IP
   */
  static fromHumidityRatio(t_dry_bulb: number, humidity_ratio: number, pressure: number, is_si: boolean): WasmMoistAir;
  /**
   * Creates a new WasmMoistAir instance from dry-bulb temperature and specific enthalpy.
   */
  static fromSpecificEnthalpy(t_dry_bulb: number, specific_enthalpy: number, pressure: number, is_si: boolean): WasmMoistAir;
  /**
   * Returns the current wet-bulb temperature.
   */
  static fromTWetBulb(t_dry_bulb: number, t_wet_bulb: number, pressure: number, is_si: boolean): WasmMoistAir;
  /**
   * Returns the current dew-point temperature.
   */
  static fromTDewPoint(t_dry_bulb: number, t_dew_point: number, pressure: number, is_si: boolean): WasmMoistAir;
  /**
   * Returns the current dry-bulb temperature.
   */
  tDryBulb(): number;
  /**
   * Returns the current humidity ratio.
   */
  humidityRatio(): number;
  /**
   * Returns the specific enthalpy.
   */
  specificEnthalpy(): number;
  /**
   * Returns the relative humidity.
   */
  relativeHumidity(): number;
  /**
   * Returns the wet-bulb temperature.
   */
  tWetBulb(): number;
  /**
   * Returns the dew-point temperature.
   */
  tDewPoint(): number;
  /**
   * Returns the moist air density.
   */
  density(): number;
  /**
   * Heating process
   */
  heatingPower(mda: number, power: number): void;
  /**
   * Heating process
   */
  heatingDeltaTemperature(mda: number, dt: number): number;
  /**
   * Cooling process
   */
  coolingPower(mda: number, power: number): void;
  /**
   * Cooling process
   */
  coolingDeltaTemperature(mda: number, dt: number): number;
  /**
   * Humidification process
   */
  humidifyAdiabatic(mda: number, w: number): void;
  /**
   * Humidification process
   */
  humidifyIsothermal(mda: number, w: number): void;
}
export class WasmPoint {
  private constructor();
  free(): void;
  x: number;
  y: number;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_wasmpoint_free: (a: number, b: number) => void;
  readonly __wbg_get_wasmpoint_x: (a: number) => number;
  readonly __wbg_set_wasmpoint_x: (a: number, b: number) => void;
  readonly __wbg_get_wasmpoint_y: (a: number) => number;
  readonly __wbg_set_wasmpoint_y: (a: number, b: number) => void;
  readonly relativeHumidityLine: (a: number, b: number, c: number, d: number, e: number) => [number, number];
  readonly specificEnthalpyLine: (a: number, b: number, c: number, d: number, e: number) => [number, number, number, number];
  readonly __wbg_wasmmoistair_free: (a: number, b: number) => void;
  readonly wasmmoistair_fromRelativeHumidity: (a: number, b: number, c: number, d: number) => [number, number, number];
  readonly wasmmoistair_fromHumidityRatio: (a: number, b: number, c: number, d: number) => [number, number, number];
  readonly wasmmoistair_fromSpecificEnthalpy: (a: number, b: number, c: number, d: number) => number;
  readonly wasmmoistair_fromTWetBulb: (a: number, b: number, c: number, d: number) => [number, number, number];
  readonly wasmmoistair_fromTDewPoint: (a: number, b: number, c: number, d: number) => [number, number, number];
  readonly wasmmoistair_tDryBulb: (a: number) => number;
  readonly wasmmoistair_humidityRatio: (a: number) => number;
  readonly wasmmoistair_specificEnthalpy: (a: number) => number;
  readonly wasmmoistair_relativeHumidity: (a: number) => [number, number, number];
  readonly wasmmoistair_tWetBulb: (a: number) => [number, number, number];
  readonly wasmmoistair_tDewPoint: (a: number) => [number, number, number];
  readonly wasmmoistair_density: (a: number) => number;
  readonly wasmmoistair_heatingPower: (a: number, b: number, c: number) => void;
  readonly wasmmoistair_heatingDeltaTemperature: (a: number, b: number, c: number) => number;
  readonly wasmmoistair_coolingPower: (a: number, b: number, c: number) => [number, number];
  readonly wasmmoistair_coolingDeltaTemperature: (a: number, b: number, c: number) => [number, number, number];
  readonly wasmmoistair_humidifyAdiabatic: (a: number, b: number, c: number) => [number, number];
  readonly wasmmoistair_humidifyIsothermal: (a: number, b: number, c: number) => [number, number];
  readonly __wbindgen_export_0: WebAssembly.Table;
  readonly __externref_drop_slice: (a: number, b: number) => void;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __externref_table_dealloc: (a: number) => void;
  readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
*
* @returns {InitOutput}
*/
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
