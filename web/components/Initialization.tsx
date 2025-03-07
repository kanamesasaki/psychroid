// components/NumberInputForm.tsx
"use client";

import { InitialState } from '../app/page';
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

// onInitialize: props として渡される関数
export default function Initialization({ onInitialize }: { onInitialize: (initialStateInput: InitialState) => void }) {
    const [pressureInput, setPressureInput] = useState<string>("101325.0");
    const [flowRateType, setFlowRateType] = useState<string>("dry_air_mass_flow_rate");
    const [flowRateInput, setFlowRateInput] = useState<string>("3.3333");
    const [inputValue1, setInputValue1] = useState<string>("30.0");
    const [inputType2, setInputType2] = useState<string>("humidity_ratio");
    const [inputValue2, setInputValue2] = useState<string>("0.01");

    const handleSubmit = (e: React.FormEvent) => {
        e.preventDefault();
        const initialStateInput: InitialState = {
            pressure: Number(pressureInput),
            flowRateType: flowRateType,
            flowRateValue: Number(flowRateInput),
            parameterType1: "t_dry_bulb",
            value1: Number(inputValue1),
            parameterType2: inputType2,
            value2: Number(inputValue2),
        };
        onInitialize(initialStateInput);
    };

    const getRangeForInputType = (inputType: string) => {
        switch (inputType) {
            case "humidity_ratio":
                return { min: 0.0, max: 0.030 };
            case "relative_humidity":
                return { min: 0.0, max: 100.0 };
            case "t_wet_bulb":
                return { min: -100.0, max: 200.0 };
            case "t_dew_point":
                return { min: -100.0, max: 200.0 };
            case "specific_enthalpy":
                return { min: 0.0, max: 500.0 };
            default:
                return { min: 0.0, max: 1.0 };
        }
    };

    const { min, max } = getRangeForInputType(inputType2);

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
                                placeholder="101325.0"
                            />
                        </div>
                        <div>
                            <Label>Dry-bulb Temperature [°C]</Label>
                            <Input
                                type="number"
                                value={inputValue1}
                                onChange={(e) => setInputValue1(e.target.value)}
                                placeholder="30.0"
                                min="-100"
                                max="200"
                            />
                        </div>
                        <div className="space-y-1">
                            <Select onValueChange={setFlowRateType} defaultValue="dry_air_mass_flow_rate">
                                <SelectTrigger className="w-full">
                                    <SelectValue placeholder="dry_air_mass_flow_rate" />
                                </SelectTrigger>
                                <SelectContent>
                                    <SelectGroup>
                                        <SelectLabel>Select Input Type</SelectLabel>
                                        <SelectItem value="total_air_mass_flow_rate">Total air mass flow rate [kg/s]</SelectItem>
                                        <SelectItem value="dry_air_mass_flow_rate">Dry air mass flow rate [kg/s]</SelectItem>
                                        <SelectItem value="volumetric_flow_rate">Volumetric flow rate [m³/s]</SelectItem>
                                    </SelectGroup>
                                </SelectContent>
                            </Select>
                            <Input
                                type="number"
                                value={flowRateInput}
                                onChange={(e) => setFlowRateInput(e.target.value)}
                            />
                        </div>
                        <div className="space-y-1">
                            <Select onValueChange={setInputType2} defaultValue="humidity_ratio">
                                <SelectTrigger className="w-full">
                                    <SelectValue placeholder="humidity_ratio" />
                                </SelectTrigger>
                                <SelectContent>
                                    <SelectGroup>
                                        <SelectLabel>Select Input Type</SelectLabel>
                                        <SelectItem value="humidity_ratio">Humidity Ratio</SelectItem>
                                        <SelectItem value="relative_humidity">Relative Humidity</SelectItem>
                                        <SelectItem value="t_wet_bulb">Wet-bulb Temperature</SelectItem>
                                        <SelectItem value="t_dew_point">Dew-point Temperature</SelectItem>
                                        <SelectItem value="specific_enthalpy">Specific Enthalpy</SelectItem>
                                    </SelectGroup>
                                </SelectContent>
                            </Select>
                            <Input
                                type="number"
                                value={inputValue2}
                                onChange={(e) => setInputValue2(e.target.value)}
                            // min={min}
                            // max={max}
                            />
                        </div>
                        <div className="flex items-center h-full">
                            {/* <RadioGroup defaultValue="si" className="flex flex-row space-x-4">
                                <Label>Unit System: </Label>
                                <div className="flex items-center space-x-2">
                                    <RadioGroupItem value="si" id="si" />
                                    <Label htmlFor="SI">SI</Label>
                                </div>
                                <div className="flex items-center space-x-2">
                                    <RadioGroupItem value="ip" id="ip" />
                                    <Label htmlFor="IP">IP</Label>
                                </div>
                            </RadioGroup> */}
                        </div>
                        <div>
                            <Button type="submit" className="w-full">Set Initial State</Button>
                        </div>
                    </div>


                </form>
            </CardContent>
        </Card>
    );
}