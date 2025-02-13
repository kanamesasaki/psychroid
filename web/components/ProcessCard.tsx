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
    const [inputValues, setInputValues] = useState<{ [key: string]: string }>({});

    const handleProcessTypeChange = (value: string) => {
        setProcessType(value);
        // Process の種類を変更したら入力値をリセットする例
        setInputValues({});
        onChange({ ...processData, processType: value });
    };

    const handleInputChange = (
        e: React.ChangeEvent<HTMLInputElement>,
        field: string
    ) => {
        const newValues = { ...inputValues, [field]: e.target.value };
        setInputValues(newValues);
        // 必要に応じて onChange で上位コンポーネントへ通知
        onChange({ ...processData, processType, /* その他の値 */ });
    };

    const renderInputs = () => {
        switch (processType) {
            case "Heating":
                return (
                    <>
                        <div>
                            <Label>Target Temperature [°C]</Label>
                            <Input
                                type="number"
                                value={inputValues.targetTemperature || ""}
                                onChange={(e) => handleInputChange(e, "targetTemperature")}
                                placeholder="Enter target temperature"
                            />
                        </div>
                        <div>
                            <Label>Heating Rate [°C/min]</Label>
                            <Input
                                type="number"
                                value={inputValues.heatingRate || ""}
                                onChange={(e) => handleInputChange(e, "heatingRate")}
                                placeholder="Enter heating rate"
                            />
                        </div>
                    </>
                );
            case "Cooling":
                return (
                    <>
                        <div>
                            <Label>Target Temperature [°C]</Label>
                            <Input
                                type="number"
                                value={inputValues.targetTemperature || ""}
                                onChange={(e) => handleInputChange(e, "targetTemperature")}
                                placeholder="Enter target temperature"
                            />
                        </div>
                        <div>
                            <Label>Cooling Rate [°C/min]</Label>
                            <Input
                                type="number"
                                value={inputValues.coolingRate || ""}
                                onChange={(e) => handleInputChange(e, "coolingRate")}
                                placeholder="Enter cooling rate"
                            />
                        </div>
                    </>
                );
            case "Humidify":
                return (
                    <div>
                        <Label>Target Humidity Ratio [kg/kg]</Label>
                        <Input
                            type="number"
                            value={inputValues.targetHumidity || ""}
                            onChange={(e) => handleInputChange(e, "targetHumidity")}
                            placeholder="Enter target humidity ratio"
                        />
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
                    <div className="flex flex-col gap-2">
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