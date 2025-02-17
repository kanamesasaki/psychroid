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
    const [flowRateInput, setFlowRateInput] = useState<string>("1000.0");
    const [inputValue1, setInputValue1] = useState<string>("30.0");
    const [inputType2, setInputType2] = useState<string>("humidity_ratio");
    const [inputValue2, setInputValue2] = useState<string>("0.01");

    const handleSubmit = (e: React.FormEvent) => {
        e.preventDefault();
        const initialStateInput: InitialState = {
            pressure: Number(pressureInput),
            massFlow: Number(flowRateInput),
            parameterType1: "t_dry_bulb",
            value1: Number(inputValue1),
            parameterType2: inputType2,
            value2: Number(inputValue2),
        };
        onInitialize(initialStateInput);
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
                                value={inputValue1}
                                onChange={(e) => setInputValue1(e.target.value)}
                                placeholder="30.0"
                            />
                        </div>
                        <div className="space-y-1">
                            <Select onValueChange={setInputType2} defaultValue="humidity_ratio">
                                <SelectTrigger className="w-full">
                                    <SelectValue placeholder="Humidity Ratio" />
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