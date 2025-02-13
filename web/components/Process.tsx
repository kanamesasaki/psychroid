"use client";

import React, { useState } from "react";
import ProcessCard, { ProcessData } from "./ProcessCard";
import { Button } from "./ui/button";

const Process = () => {
    const [processes, setProcesses] = useState<ProcessData[]>([
        { id: 1, processType: "Heating" },
    ]);

    const addProcessCard = () => {
        const newProcess: ProcessData = { id: Date.now(), processType: "Heating" };
        setProcesses((prev) => [...prev, newProcess]);
    };

    const updateProcess = (updatedProcess: ProcessData) => {
        setProcesses((prev) =>
            prev.map((proc) => (proc.id === updatedProcess.id ? updatedProcess : proc))
        );
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
            <div className="flex justify-end">
                <Button onClick={addProcessCard} className="text-sm px-3 py-1">
                    Add Process
                </Button>
            </div>
        </div>
    );
};

export default Process;