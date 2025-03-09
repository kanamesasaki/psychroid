"use client";

import React, { useState } from "react";
import ProcessCard from "./ProcessCard";
import { Button } from "./ui/button";
import { SquarePlus } from "lucide-react";
import { Process } from "@/app/page";

const ProcessArray = ({ onApplyProcesses }: { onApplyProcesses: (processes: Process[]) => void }) => {
    const [processes, setProcesses] = useState<Process[]>([
        { id: 0, processType: "Heating", inputType: "Power", value: 0.0 } as Process,
    ]);

    const addProcessCard = () => {
        const newProcess: Process = { id: processes.length, processType: "Heating", inputType: "Power", value: 0.0 };
        setProcesses((prev) => [...prev, newProcess]);
    };

    const updateProcess = (updatedProcess: Process) => {
        setProcesses((prev) =>
            prev.map((proc) => (proc.id === updatedProcess.id ? updatedProcess : proc))
        );
    };

    const handleApplyProcess = () => {
        // update processes in page.tsx
        onApplyProcesses(processes);
    };

    return (
        <div className="flex flex-col gap-4">
            {processes.map((proc) => (
                <ProcessCard
                    key={proc.id}
                    processData={proc}
                    onChange={updateProcess}
                />
            ))}
            <div className="grid grid-cols-2 gap-4">
                <Button onClick={addProcessCard} className="text-sm px-3 py-1">
                    <SquarePlus className="mr-1" />
                    Add Process
                </Button>
                <Button onClick={handleApplyProcess} className="text-sm px-3 py-1">
                    Apply Process
                </Button>
            </div>
        </div>
    );
};

export default ProcessArray;