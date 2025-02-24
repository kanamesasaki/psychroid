"use client";

import React, { useState, useEffect } from "react";
import Initialization from "../components/Initialization";
import Chart from "../components/Chart";
import Header from "../components/Header";
import StateTable from "../components/StateTable";
import init, { relativeHumidityLine, specificEnthalpyLine, WasmMoistAir } from '@/lib/psychroid';
import ProcessArray from "../components/ProcessArray";

export type Point = {
  x: number; // Dry-bulb temperature in °C 
  y: number; // Humidity ratio in kg/kg
};

export type Line = {
  data: Point[];
  label: string;
};

export type InitialState = {
  pressure: number;
  massFlow: number;
  parameterType1: string; // t_dry_bulb
  value1: number;
  parameterType2: string; // humidity_ratio, relative_humidity, t_wet_bulb, t_dew_point, enthalpy
  value2: number;
};

export type State = {
  id: number;
  tDryBulb: number;
  humidityRatio: number;
  tWetBulb: number;
  tDewPoint: number;
  relativeHumidity: number;
  enthalpy: number;
  density: number;
}

export type Process = {
  id: number;
  processType: string; // heating, cooling, humidification
  inputType: string;
  value: number;
};


const Page = () => {
  // WASM init state
  const [wasmInitialized, setWasmInitialized] = React.useState(false);
  // Chart lines
  const [rhLines, setRhLines] = React.useState<Line[]>([]);
  const [enthalpyLines, setEnthalpyLines] = React.useState<Line[]>([]);
  // initial state
  const [initialState, setInitialState] = React.useState<InitialState>(
    {
      pressure: 101325,
      massFlow: 3.3333,
      parameterType1: "t_dry_bulb",
      value1: 30.0,
      parameterType2: "humidity_ratio",
      value2: 0.01
    }
  );
  // Process array
  const [processes, setProcesses] = useState<Process[]>([]);
  // State array
  const [states, setStates] = useState<Array<State>>([]);

  // Load WASM module
  useEffect(() => {
    async function loadWasm() {
      try {
        console.log("Starting WASM initialization");
        await init();
        console.log("WASM initialized");
        setWasmInitialized(true);
      } catch (err) {
        console.error("Failed to load WASM:", err);
      }
    }
    loadWasm();
  }, []);

  // Get relative humidity lines using WASM module
  const getRhLines = (): Line[] => {
    const rhValues = Array.from({ length: 10 }, (_, i) => (i + 1) * 0.1);
    const rhLines: Line[] = [];

    rhValues.forEach(rh => {
      const wasmPoints = relativeHumidityLine(
        rh,                       // RH value (0.1 to 1.0)
        initialState.pressure,    // Standard pressure
        -15,                      // Min dry-bulb temperature
        40,                       // Max dry-bulb temperature
        true                      // Use SI units
      );

      rhLines.push({
        data: wasmPoints,
        label: `${Math.round(rh * 100)}%`
      });
    });

    return rhLines;
  };

  // Get enthalpy lines using WASM module
  const getEnthalpyLines = (): Line[] => {
    const enthalpyValues = Array.from({ length: 13 }, (_, i) => (i - 1) * 10);
    const enthalpyLines: Line[] = [];

    enthalpyValues.forEach(enthalpy => {
      const wasmPoints = specificEnthalpyLine(
        enthalpy,                 // Enthalpy value
        initialState.pressure,    // Standard pressure
        -15,                      // Min dry-bulb temperature
        40,                       // Max dry-bulb temperature
        true                      // Use SI units
      );

      enthalpyLines.push({
        data: wasmPoints,
        label: `${enthalpy} kJ/kg`
      });
    });

    return enthalpyLines;
  };

  // Update rhLines whenever wasmInitialized or initialState changes
  useEffect(() => {
    if (wasmInitialized) {
      setRhLines(getRhLines());
      setEnthalpyLines(getEnthalpyLines());
      console.log("plot RH lines and enthalpy lines");
    }
  }, [wasmInitialized, initialState]);

  const handleInitialize = (initialStateInput: InitialState) => {
    setInitialState(initialStateInput);
    console.log("Initialized:", initialState);
  };

  const handleApplyProcesses = (processes: Process[]) => {
    setProcesses(processes);
    console.log("Processes:", processes);
  };

  const calculateNextState = (prev: State, proc: Process) => {
    let moistAir: WasmMoistAir = WasmMoistAir.fromHumidityRatio(prev.tDryBulb, prev.humidityRatio, initialState.pressure, true);
    if (proc.processType === "Heating" && proc.inputType === "Power") {
      moistAir.heatingPower(initialState.massFlow, proc.value);
    } else if (proc.processType === "Heating" && proc.inputType === "ΔT") {
      let q = moistAir.heatingDeltaTemperature(initialState.massFlow, proc.value);
      console.log("Heating with ΔT:", proc.value, moistAir.tDryBulb(), q);
    } else if (proc.processType === "Cooling" && proc.inputType === "Power") {
      moistAir.coolingPower(initialState.massFlow, proc.value);
    } else if (proc.processType === "Cooling" && proc.inputType === "ΔT") {
      moistAir.coolingDeltaTemperature(initialState.massFlow, proc.value);
    }
    let next = {
      id: prev.id + 1,
      tDryBulb: moistAir.tDryBulb(),
      humidityRatio: moistAir.humidityRatio(),
      tWetBulb: moistAir.tWetBulb(),
      tDewPoint: moistAir.tDewPoint(),
      relativeHumidity: moistAir.relativeHumidity(),
      enthalpy: moistAir.specificEnthalpy(),
      density: moistAir.density()
    } as State;
    return next;
  };

  // Update states whenever initialState changes
  useEffect(() => {
    if (wasmInitialized) {
      const stateArray: State[] = [];

      // Create moist air object based on the second input parameter
      let moistAir: WasmMoistAir;
      if (initialState.parameterType2 === "humidity_ratio") {
        moistAir = WasmMoistAir.fromHumidityRatio(initialState.value1, initialState.value2, initialState.pressure, true);
      } else if (initialState.parameterType2 === "relative_humidity") {
        moistAir = WasmMoistAir.fromRelativeHumidity(initialState.value1, initialState.value2, initialState.pressure, true);
      } else if (initialState.parameterType2 === "t_wet_bulb") {
        moistAir = WasmMoistAir.fromTWetBulb(initialState.value1, initialState.value2, initialState.pressure, true);
      } else if (initialState.parameterType2 === "t_dew_point") {
        moistAir = WasmMoistAir.fromTDewPoint(initialState.value1, initialState.value2, initialState.pressure, true);
      } else if (initialState.parameterType2 === "specific_enthalpy") {
        moistAir = WasmMoistAir.fromSpecificEnthalpy(initialState.value1, initialState.value2, initialState.pressure, true);
      } else {
        console.error("Invalid parameter type");
        return
      }

      const state0: State = {
        id: 0,
        tDryBulb: moistAir.tDryBulb(),
        humidityRatio: moistAir.humidityRatio(),
        tWetBulb: moistAir.tWetBulb(),
        tDewPoint: moistAir.tDewPoint(),
        relativeHumidity: moistAir.relativeHumidity(),
        enthalpy: moistAir.specificEnthalpy(),
        density: moistAir.density()
      }
      stateArray.push(state0);

      processes.map((proc, index) => {
        const prevState = stateArray[index];
        const nextState = calculateNextState(prevState, proc);
        // const nextState = prevState;
        stateArray.push(nextState);
      })

      setStates(stateArray);
      console.log("States:", stateArray);
    }
  }, [initialState, wasmInitialized, processes]);

  return (
    // padding: 1.5rem; /* 24px */
    <main className="pt-2 px-6 pb-6">
      <Header />
      <div className="grid grid-cols-1 md:grid-cols-12 gap-6">
        <div className="col-span-1 md:col-span-7">
          <Chart rhLines={rhLines} enthalpyLines={enthalpyLines} states={states} />
          <StateTable states={states} />
        </div>
        <div className="col-span-1 md:col-span-5">
          <Initialization onInitialize={handleInitialize} />
          <ProcessArray onApplyProcesses={handleApplyProcesses} />
        </div>
      </div>
    </main>
  );
}
export default Page;