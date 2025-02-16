// components/NumberInputForm.tsx
"use client";

import React, { useState } from "react";
import { Input } from "./ui/input";
import { Button } from "./ui/button";
import {
    Select,
    SelectContent,
    SelectGroup,
    SelectItem,
    SelectLabel,
    SelectTrigger,
    SelectValue,
} from "@/components/ui/select"
import {
    Card,
    CardContent,
    CardDescription,
    CardHeader,
    CardTitle,
} from "./ui/card";
import { Label } from "./ui/label";

export default function Initialization({ onInitialize }: { onInitialize: (pressure: number, massflow: number, temperature: number, humidity: number) => void }) {
    const [pressureInput, setPressureInput] = useState<string>("101325.0");
    const [flowRateInput, setFlowRateInput] = useState<string>("1000.0");
    const [temperatureInput, setTemperatureInput] = useState<string>("30.0");
    const [humidityInput, setHumidityInput] = useState<string>("0.01");

    const handleSubmit = (e: React.FormEvent) => {
        e.preventDefault();
        const pressure = Number(pressureInput);
        const massflow = Number(flowRateInput);
        const temperature = Number(temperatureInput);
        const humidity = Number(humidityInput);
        onInitialize(pressure, massflow, temperature, humidity);
    };

    return (
        <Card className="w-full mb-4">
            <CardHeader>
                <CardTitle>Initialization</CardTitle>
                {/* <CardDescription>Set initial state</CardDescription> */}
            </CardHeader>
            <CardContent>
                <form onSubmit={handleSubmit} className="flex flex-col gap-4 w-full">
                    <div className="grid grid-cols-2 gap-4">
                        <div>
                            <Label>Pressure [Pa]</Label>
                            <Input
                                type="number"
                                value={pressureInput}
                                onChange={(e) => setPressureInput(e.target.value)}
                                placeholder="101325"
                            />
                        </div>
                        <div>
                            <Label>Air mass flow rate [kg/s]</Label>
                            <Input
                                type="number"
                                value={flowRateInput}
                                onChange={(e) => setFlowRateInput(e.target.value)}
                                placeholder="10000"
                            />
                        </div>
                        <div>
                            <Label>Dry-bulb Temperature [°C]</Label>
                            <Input
                                type="number"
                                value={temperatureInput}
                                onChange={(e) => setTemperatureInput(e.target.value)}
                                placeholder="30.0"
                            />
                        </div>
                        <div className="space-y-1">
                            <Select>
                                <SelectTrigger className="w-full">
                                    <SelectValue placeholder="Humidity Ratio" />
                                </SelectTrigger>
                                <SelectContent>
                                    <SelectGroup>
                                        <SelectLabel>Select Input Type</SelectLabel>
                                        <SelectItem value="humidity_ratio">Humidity Ratio</SelectItem>
                                        <SelectItem value="relative_humidity">Relative Humidity</SelectItem>
                                        <SelectItem value="wet-bulb_temperature">Wet-bulb Temperature</SelectItem>
                                    </SelectGroup>
                                </SelectContent>
                            </Select>
                            <Input
                                type="number"
                                value={humidityInput}
                                onChange={(e) => setHumidityInput(e.target.value)}
                                placeholder="0.01"
                            />
                        </div>
                    </div>

                    <Button type="submit">Set Initial State</Button>
                </form>
            </CardContent>
        </Card>
    );
}