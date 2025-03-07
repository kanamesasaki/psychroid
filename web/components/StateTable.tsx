"use client";

import React from "react";
import { State } from '../app/page';

interface StateTableProps {
    states: Array<State>;
}

const ProcessTable: React.FC<StateTableProps> = ({ states }) => {
    return (
        <div className="overflow-x-auto">
            <table className="min-w-full bg-white border border-gray-200">
                <thead>
                    <tr>
                        <th className="px-4 py-2 border-b text-left">ID</th>
                        <th className="px-4 py-2 border-b text-left">T<sub>db</sub> (°C)</th>
                        <th className="px-4 py-2 border-b text-left">W (kg/kg)</th>
                        <th className="px-4 py-2 border-b text-left">RH (%)</th>
                        <th className="px-4 py-2 border-b text-left">h (kJ/kg)</th>
                        <th className="px-4 py-2 border-b text-left">T<sub>wb</sub> (°C)</th>
                        <th className="px-4 py-2 border-b text-left">T<sub>dew</sub> (°C)</th>
                        <th className="px-4 py-2 border-b text-left">ρ (kg/m³)</th>
                        <th className="px-4 py-2 border-b text-left">V (m³/s)</th>
                    </tr>
                </thead>
                <tbody>
                    {states.map((state, index) => (
                        <tr key={index}>
                            <td className="px-4 py-2 border-b">{index}</td>
                            <td className="px-4 py-2 border-b">{state.tDryBulb.toFixed(2)}</td>
                            <td className="px-4 py-2 border-b">{state.humidityRatio.toFixed(4)}</td>
                            <td className="px-4 py-2 border-b">{(state.relativeHumidity * 100).toFixed(1)}</td>
                            <td className="px-4 py-2 border-b">{state.enthalpy.toFixed(2)}</td>
                            <td className="px-4 py-2 border-b">{state.tWetBulb.toFixed(2)}</td>
                            <td className="px-4 py-2 border-b">{state.tDewPoint.toFixed(2)}</td>
                            <td className="px-4 py-2 border-b">{state.density.toFixed(3)}</td>
                            <td className="px-4 py-2 border-b">{(state.dryAirMassFlowRate * (1.0 + state.humidityRatio) / state.density).toFixed(3)}</td>
                        </tr>
                    ))}
                </tbody>
            </table>
        </div>
    );
};

export default ProcessTable;