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
                        <th className="px-4 py-2 border-b">ID</th>
                        <th className="px-4 py-2 border-b">T<sub>db</sub> (°C)</th>
                        <th className="px-4 py-2 border-b">W (kg/kg)</th>
                        <th className="px-4 py-2 border-b">rH (%)</th>
                        <th className="px-4 py-2 border-b">h (J)</th>
                        <th className="px-4 py-2 border-b">T<sub>wb</sub> (°C)</th>
                        <th className="px-4 py-2 border-b">T<sub>dew</sub> (°C)</th>
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
                        </tr>
                    ))}
                </tbody>
            </table>
        </div>
    );
};

export default ProcessTable;