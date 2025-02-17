"use client";

import React, { useState } from "react";
import { Card, CardContent, CardHeader, CardTitle } from "./ui/card";
import { Input } from "./ui/input";
import { Button } from "./ui/button";
import {
    Select,
    SelectTrigger,
    SelectValue,
    SelectContent,
    SelectItem,
} from "@/components/ui/select";
import { Label } from "./ui/label";

export interface ProcessData {
    id: number;
    processType: string;
    // 他に必要なデータがあれば追加
}

interface ProcessCardProps {
    processData: ProcessData;
    onChange: (data: ProcessData) => void;
}

const ProcessCard = ({ processData, onChange }: ProcessCardProps) => {
    const [processType, setProcessType] = useState<string>(processData.processType);
    const [inputType, setInputType] = useState<string>("Power");
    const [inputValue, setInputValue] = useState<string>("0.0");


    const handleProcessTypeChange = (value: string) => {
        setProcessType(value);
        // Process の種類を変更したら入力値をリセットする例
        setInputValue("0.0");
        onChange({ ...processData, processType: value });
    };

    // const handleInputChange = (
    //     e: React.ChangeEvent<HTMLInputElement>,
    //     field: string
    // ) => {
    //     const newValues = { ...inputValues, [field]: e.target.value };
    //     setInputValues(newValues);
    //     // 必要に応じて onChange で上位コンポーネントへ通知
    //     onChange({ ...processData, processType, /* その他の値 */ });
    // };

    const renderInputs = () => {
        switch (processType) {
            case "Heating":
                return (
                    <div className="grid grid-cols-2 gap-4">
                        {/* Top Row */}
                        <div>
                            <Label>Process Type</Label>
                            <Select value={processType} onValueChange={handleProcessTypeChange}>
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
                            <Select value={inputType} onValueChange={(value: string) => setInputType(value)}>
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
                                {inputType === "Power" ? "Power [kW]" : "ΔT [°C]"}
                            </Label>
                            <Input
                                type="number"
                                value={inputValue}
                                onChange={(e) => setInputValue(e.target.value)}
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
                            <Select value={processType} onValueChange={handleProcessTypeChange}>
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
                            <Select value={inputType} onValueChange={(value: string) => setInputType(value)}>
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
                                {inputType === "Power" ? "Power [kW]" : "ΔT [°C]"}
                            </Label>
                            <Input
                                type="number"
                                value={inputValue}
                                onChange={(e) => setInputValue(e.target.value)}
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
                            <Select value={processType} onValueChange={handleProcessTypeChange}>
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
                            <Select value={inputType} onValueChange={(value: string) => setInputType(value)}>
                                <SelectTrigger>
                                    <SelectValue placeholder="Select option" />
                                </SelectTrigger>
                                <SelectContent>
                                    <SelectItem value="dw_adeabatic">ΔW Adeabatic</SelectItem>
                                    <SelectItem value="dw_isotherm">ΔW Isotherm</SelectItem>
                                </SelectContent>
                            </Select>
                        </div>

                        {/* Bottom Row */}
                        <div>{/* Left bottom is blank */}</div>
                        <div>
                            <Label>
                                {inputType === "dw_adeabatic" ? "ΔW Adeabatic [kg/kg]" : "ΔW Isotherm [kg/kg]"}
                            </Label>
                            <Input
                                type="number"
                                value={inputValue}
                                onChange={(e) => setInputValue(e.target.value)}
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
            <CardHeader>
                <CardTitle>Process Settings</CardTitle>
            </CardHeader>
            <CardContent>
                <div className="flex flex-col gap-4">
                    {renderInputs()}
                    <Button type="button" onClick={() => console.log("Process Applied")}>
                        Apply Process
                    </Button>
                </div>
            </CardContent>
        </Card>
    );
};

export default ProcessCard;