"use client";

import React from "react";
import { Card, CardContent, CardHeader, CardTitle } from "./ui/card";
import { Input } from "./ui/input";
import {
    Select,
    SelectTrigger,
    SelectValue,
    SelectContent,
    SelectItem,
} from "@/components/ui/select";
import { Label } from "./ui/label";
import { Process } from "@/app/page";

interface ProcessCardProps {
    processData: Process;
    onChange: (data: Process) => void;
}

const ProcessCard = ({ processData, onChange }: ProcessCardProps) => {
    const localProcessData: Process = processData;

    const handleProcessTypeChange = (value: string) => {
        let inputType: string = "";
        if (value === "Heating" || value === "Cooling") {
            inputType = "Power";
        } else if (value === "Humidify") {
            inputType = "ΔW Adiabatic";
        }

        localProcessData.processType = value;
        localProcessData.inputType = inputType;
        localProcessData.value = 0.0;

        onChange(localProcessData);
    };

    const handleInputTypeChange = (value: string) => {
        localProcessData.inputType = value;
        onChange(localProcessData);
    };

    const handleValueChange = (value: string) => {
        localProcessData.value = Number(value);
        onChange(localProcessData);
    };

    const renderInputs = () => {
        switch (localProcessData.processType) {
            case "Heating":
                return (
                    <div className="grid grid-cols-2 gap-4">
                        {/* Top Row */}
                        <div>
                            <Label>Process Type</Label>
                            <Select value={localProcessData.processType} onValueChange={handleProcessTypeChange}>
                                <SelectTrigger>
                                    <SelectValue placeholder="Select process type" />
                                </SelectTrigger>
                                <SelectContent>
                                    <SelectItem value="Heating">Heating</SelectItem>
                                    <SelectItem value="Cooling">Cooling</SelectItem>
                                    <SelectItem value="Humidify">Humidify</SelectItem>
                                </SelectContent>
                            </Select>
                        </div>
                        <div>
                            <Label>Input Type</Label>
                            <Select value={localProcessData.inputType} onValueChange={handleInputTypeChange}>
                                <SelectTrigger>
                                    <SelectValue placeholder="Select option" />
                                </SelectTrigger>
                                <SelectContent>
                                    <SelectItem value="Power">Power</SelectItem>
                                    <SelectItem value="ΔT">ΔT</SelectItem>
                                </SelectContent>
                            </Select>
                        </div>

                        {/* Bottom Row */}
                        <div>{/* Left bottom is blank */}</div>
                        <div>
                            <Label>
                                {localProcessData.inputType === "Power" ? "Power [kW]" : "ΔT [°C]"}
                            </Label>
                            <Input
                                type="number"
                                value={localProcessData.value}
                                onChange={(e) => handleValueChange(e.target.value)}
                                placeholder={"0.0"}
                            />
                        </div>
                    </div>
                );
            case "Cooling":
                return (
                    <div className="grid grid-cols-2 gap-4">
                        {/* Top Row */}
                        <div>
                            <Label>Process Type</Label>
                            <Select value={localProcessData.processType} onValueChange={handleProcessTypeChange}>
                                <SelectTrigger>
                                    <SelectValue placeholder="Select process type" />
                                </SelectTrigger>
                                <SelectContent>
                                    <SelectItem value="Heating">Heating</SelectItem>
                                    <SelectItem value="Cooling">Cooling</SelectItem>
                                    <SelectItem value="Humidify">Humidify</SelectItem>
                                </SelectContent>
                            </Select>
                        </div>
                        <div>
                            <Label>Input Type</Label>
                            <Select value={localProcessData.inputType} onValueChange={handleInputTypeChange}>
                                <SelectTrigger>
                                    <SelectValue placeholder="Select option" />
                                </SelectTrigger>
                                <SelectContent>
                                    <SelectItem value="Power">Power</SelectItem>
                                    <SelectItem value="ΔT">ΔT</SelectItem>
                                </SelectContent>
                            </Select>
                        </div>

                        {/* Bottom Row */}
                        <div>{/* Left bottom is blank */}</div>
                        <div>
                            <Label>
                                {localProcessData.inputType === "Power" ? "Power [kW]" : "ΔT [°C]"}
                            </Label>
                            <Input
                                type="number"
                                value={localProcessData.value}
                                onChange={(e) => handleValueChange(e.target.value)}
                                placeholder={"0.0"}
                            />
                        </div>
                    </div>
                );
            case "Humidify":
                return (
                    <div className="grid grid-cols-2 gap-4">
                        {/* Top Row */}
                        <div>
                            <Label>Process Type</Label>
                            <Select value={localProcessData.processType} onValueChange={handleProcessTypeChange}>
                                <SelectTrigger>
                                    <SelectValue placeholder="Select process type" />
                                </SelectTrigger>
                                <SelectContent>
                                    <SelectItem value="Heating">Heating</SelectItem>
                                    <SelectItem value="Cooling">Cooling</SelectItem>
                                    <SelectItem value="Humidify">Humidify</SelectItem>
                                </SelectContent>
                            </Select>
                        </div>
                        <div>
                            <Label>Input Type</Label>
                            <Select value={localProcessData.inputType} onValueChange={handleInputTypeChange}>
                                <SelectTrigger>
                                    <SelectValue placeholder="Select option" />
                                </SelectTrigger>
                                <SelectContent>
                                    <SelectItem value="ΔW Adiabatic">ΔW Adiabatic</SelectItem>
                                    <SelectItem value="ΔW Isothermal">ΔW Isothermal</SelectItem>
                                </SelectContent>
                            </Select>
                        </div>

                        {/* Bottom Row */}
                        <div>{/* Left bottom is blank */}</div>
                        <div>
                            <Label>
                                {localProcessData.inputType === "ΔW Adiabatic" ? "ΔW Adiabatic [kg/s]" : "ΔW Isotherm [kg/s]"}
                            </Label>
                            <Input
                                type="number"
                                value={localProcessData.value}
                                onChange={(e) => handleValueChange(e.target.value)}
                                placeholder={"0.0"}
                            />
                        </div>
                    </div>
                );
            default:
                return null;
        }
    };

    return (
        <Card className="w-full">
            <CardHeader className="pb-2 pt-4">
                <CardTitle className="text-lg">
                    Process Settings: {localProcessData.id} &#8211; {localProcessData.id + 1}
                </CardTitle>
            </CardHeader>
            <CardContent className="pb-4">
                <div className="flex flex-col gap-4">
                    {renderInputs()}
                    {/* <Button type="button" onClick={() => console.log("Process Applied")}>
                        Apply Process
                    </Button> */}
                </div>
            </CardContent>
        </Card>
    );
};

export default ProcessCard;