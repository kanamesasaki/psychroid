"use client";

import React, { use, useEffect } from "react";
import Initialization from "../components/Initialization";
import Chart from "../components/Chart";
import Process from "../components/Process";
import init, { get_relative_humidity_line } from '@/lib/psychroid';

export type Point = {
  x: number; // Dry-bulb temperature in °C 
  y: number; // Humidity ratio in kg/kg
};

export type Line = {
  data: Point[];
  label: string;
};

export type State = {
  pressure: number;
  massFlow: number;
  temperature: number;
  humidityRatio: number;
};

const Page = () => {
  // WASM init state
  const [wasmInitialized, setWasmInitialized] = React.useState(false);
  // Total pressure in Pa
  const [pressure, setPressure] = React.useState(101325.0);
  // Chart lines
  const [rhLines, setRhLines] = React.useState<Line[]>([]);
  const [enthalpyLines, setEnthalpyLines] = React.useState<Line[]>([]);
  // initial state
  const [initialState, setInitialState] = React.useState<State | null>(null);

  // Load WASM module
  useEffect(() => {
    async function loadWasm() {
      try {
        console.log("Starting WASM initialization");
        await init(); // default export の初期化関数を呼び出す
        console.log("WASM initialized");
        setWasmInitialized(true);
      } catch (err) {
        console.error("Failed to load WASM:", err);
      }
    }
    loadWasm();
  }, []);

  const getLines = (): Line[] => {
    const rhValues = Array.from({ length: 10 }, (_, i) => (i + 1) * 0.1);
    const lines: Line[] = [];

    rhValues.forEach(rh => {
      const data = get_relative_humidity_line(
        rh,       // RH value (0.1 to 1.0)
        pressure, // Standard pressure
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

  // Update rhLines whenever pressure changes
  useEffect(() => {
    if (wasmInitialized) {
      setRhLines(getLines());
    }
  }, [pressure, wasmInitialized]);

  const handleInitialize = (pressure: number, massFlow: number, temperature: number, humidity: number) => {
    setInitialState({
      pressure,
      massFlow,
      temperature,
      humidityRatio: humidity
    });
    console.log("Initialized:", initialState);
  };

  return (
    // padding: 1.5rem; /* 24px */
    <main className="p-6">
      <h1 className="text-2xl font-bold mb-4">Psychrometric Chart</h1>
      <div className="grid grid-cols-1 md:grid-cols-12 gap-6">
        <div className="col-span-1 md:col-span-7">
          <Chart lines={rhLines} initialState={initialState} />
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