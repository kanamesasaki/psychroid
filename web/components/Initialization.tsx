// components/NumberInputForm.tsx
"use client";

import { InitialState } from '../app/page';
import React, { useState, useImperativeHandle, forwardRef } from "react";
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
    // CardDescription,
    CardHeader,
    CardTitle,
} from "./ui/card";
import { Label } from "./ui/label";

export type InitializationRef = {
    resetForm: () => void;
};

const Initialization = forwardRef<InitializationRef, { onInitialize: (initialStateInput: InitialState) => void }>(
    ({ onInitialize }: { onInitialize: (initialStateInput: InitialState) => void }, ref: React.ForwardedRef<InitializationRef>) => {
        const defaultValues = {
            pressure: "101325.0",
            flowRateType: "volumetric_flow_rate",
            flowRateValue: "1500.0",
            value1: "30.0",
            inputType2: "relative_humidity",
            value2: "50.0"
        };

        const [pressureInput, setPressureInput] = useState<string>(defaultValues.pressure);
        const [flowRateType, setFlowRateType] = useState<string>(defaultValues.flowRateType);
        const [flowRateInput, setFlowRateInput] = useState<string>(defaultValues.flowRateValue);
        const [inputValue1, setInputValue1] = useState<string>(defaultValues.value1);
        const [inputType2, setInputType2] = useState<string>(defaultValues.inputType2);
        const [inputValue2, setInputValue2] = useState<string>(defaultValues.value2);

        const resetForm = () => {
            setPressureInput(defaultValues.pressure);
            setFlowRateType(defaultValues.flowRateType);
            setFlowRateInput(defaultValues.flowRateValue);
            setInputValue1(defaultValues.value1);
            setInputType2(defaultValues.inputType2);
            setInputValue2(defaultValues.value2);
        };

        useImperativeHandle(ref, () => ({
            resetForm,
        }));

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
                    return { min: 0.0, max: undefined };
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
                <CardHeader className="pb-2 pt-4">
                    <CardTitle className="text-lg">
                        Initialization
                    </CardTitle>
                    {/* <CardDescription>Set initial state</CardDescription> */}
                </CardHeader>
                <CardContent className="pb-4">
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
                                    step="any"
                                />
                            </div>
                            <div className="space-y-1">
                                <Select onValueChange={setFlowRateType} defaultValue="volumetric_flow_rate">
                                    <SelectTrigger className="w-full py-1 h-9">
                                        <SelectValue placeholder="volumetric_flow_rate" />
                                    </SelectTrigger>
                                    <SelectContent>
                                        <SelectGroup>
                                            <SelectLabel>Select Input Type</SelectLabel>
                                            <SelectItem value="total_air_mass_flow_rate">Total air mass flow rate [kg/s]</SelectItem>
                                            <SelectItem value="dry_air_mass_flow_rate">Dry air mass flow rate [kg/s]</SelectItem>
                                            <SelectItem value="volumetric_flow_rate">Volumetric flow rate [m³/h]</SelectItem>
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
                                <Select onValueChange={setInputType2} defaultValue="relative_humidity">
                                    <SelectTrigger className="w-full py-1 h-9">
                                        <SelectValue placeholder="relative_humidity" />
                                    </SelectTrigger>
                                    <SelectContent>
                                        <SelectGroup>
                                            <SelectLabel>Select Input Type</SelectLabel>
                                            <SelectItem value="humidity_ratio">Humidity Ratio [kg/kg]</SelectItem>
                                            <SelectItem value="relative_humidity">Relative Humidity [%]</SelectItem>
                                            <SelectItem value="t_wet_bulb">Wet-bulb Temperature [°C]</SelectItem>
                                            <SelectItem value="t_dew_point">Dew-point Temperature [°C]</SelectItem>
                                            <SelectItem value="specific_enthalpy">Specific Enthalpy [kJ/kg]</SelectItem>
                                        </SelectGroup>
                                    </SelectContent>
                                </Select>
                                <Input
                                    type="number"
                                    value={inputValue2}
                                    onChange={(e) => setInputValue2(e.target.value)}
                                    min={min}
                                    max={max}
                                    step="any"
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
);

Initialization.displayName = 'Initialization';
export default Initialization;