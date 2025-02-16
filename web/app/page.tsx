"use client";

import React, { useState, useEffect } from "react";
import Initialization from "../components/Initialization";
import Chart from "../components/Chart";
import Process from "../components/Process";
import Header from "../components/Header";
import ProcessTable from "../components/StateTable";
import init, { get_relative_humidity_line } from '@/lib/psychroid';

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
  tDryBulb: number;
  humidityRatio: number;
  tWetBulb: number;
  tDewPoint: number;
  relativeHumidity: number;
  enthalpy: number;
}

export type Process = {
  processType: string;
  parameterType: string;
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
      massFlow: 10000.0,
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
  const getLines = (): Line[] => {
    const rhValues = Array.from({ length: 10 }, (_, i) => (i + 1) * 0.1);
    const lines: Line[] = [];

    rhValues.forEach(rh => {
      const data = get_relative_humidity_line(
        rh,       // RH value (0.1 to 1.0)
        initialState.pressure, // Standard pressure
        true      // Use SI units
      );

      const points = Array.from(data.temperatures).map((t, i) => ({
        x: t,
        y: data.humidity_ratios[i]
      }));

      lines.push({
        data: points,
        label: `${Math.round(rh * 100)}%`
      });
    });

    return lines;
  };

  // Update rhLines whenever wasmInitialized or initialState changes
  useEffect(() => {
    if (wasmInitialized) {
      setRhLines(getLines());
      console.log("replot rH lines");
    }
  }, [wasmInitialized, initialState]);

  const handleInitialize = (initialStateInput: InitialState) => {
    setInitialState(initialStateInput);
    console.log("Initialized:", initialState);
  };

  // Update states whenever initialState changes
  useEffect(() => {
    if (wasmInitialized) {
      const stateArray: State[] = [];
      const state0: State = {
        tDryBulb: initialState.value1,
        humidityRatio: initialState.value2,
        tWetBulb: 0,
        tDewPoint: 0,
        relativeHumidity: 0,
        enthalpy: 0
      }
      stateArray.push(state0);
      setStates(stateArray);
      console.log("States:", states);
    }
  }, [initialState, wasmInitialized]);

  return (
    // padding: 1.5rem; /* 24px */
    <main className="pt-2 px-6 pb-6">
      <Header />
      <div className="grid grid-cols-1 md:grid-cols-12 gap-6">
        <div className="col-span-1 md:col-span-7">
          <Chart lines={rhLines} states={states} />
          <ProcessTable states={states} />
        </div>
        <div className="col-span-1 md:col-span-5">
          <Initialization onInitialize={handleInitialize} />
          <Process />
        </div>
      </div>
    </main>
  );
}
export default Page;