"use client";

import { useEffect, useRef, useState } from 'react';
import * as d3 from 'd3';
import { Point, Line, State } from '../app/page';
import { Button } from './ui/button';
import { Download } from "lucide-react"; // アイコン用

interface ChartProps {
    lines: Line[];
    initialState: State | null;
}

const width = 400;
const height = 300;
const margin = { top: 5, right: 55, bottom: 40, left: 10 };


const Chart = ({ lines, initialState }: ChartProps) => {
    const svgRef = useRef<SVGSVGElement>(null);
    const [xMin, setXMin] = useState(0);
    const [xMax, setXMax] = useState(40);
    const [yMin, setYMin] = useState(0);
    const [yMax, setYMax] = useState(0.03);

    const exportSVG = async () => {
        const svgEl = svgRef.current;
        if (!svgEl) return;
        const serializer = new XMLSerializer();
        let source = serializer.serializeToString(svgEl);

        // Add missing namespaces
        if (!source.match(/^<svg[^>]+xmlns="http:\/\/www\.w3\.org\/2000\/svg"/)) {
            source = source.replace(/^<svg/, '<svg xmlns="http://www.w3.org/2000/svg"');
        }
        if (!source.match(/^<svg[^>]+"http:\/\/www\.w3\.org\/1999\/xlink"/)) {
            source = source.replace(/^<svg/, '<svg xmlns:xlink="http://www.w3.org/1999/xlink"');
        }

        const blob = new Blob([source], { type: "image/svg+xml" });

        // if File System Access API is available 
        if ("showSaveFilePicker" in window) {
            try {
                const options = {
                    suggestedName: "chart.svg",
                    types: [
                        {
                            description: "SVG file",
                            accept: { "image/svg+xml": [".svg"] },
                        },
                    ],
                };
                // @ts-ignore
                const fileHandle = await window.showSaveFilePicker(options);
                const writable = await fileHandle.createWritable();
                await writable.write(blob);
                await writable.close();
            } catch (error: any) {
                if (error.name === "AbortError") {
                    console.log("File saving was cancelled");
                } else {
                    console.error("File saving was cancelled or failed", error);
                }
            }
        } else {
            // Fallback for browsers without File System Access API
            const url = URL.createObjectURL(blob);
            const a = document.createElement("a");
            a.href = url;
            a.download = "chart.svg";
            document.body.appendChild(a);
            a.click();
            document.body.removeChild(a);
            URL.revokeObjectURL(url);
        }
    };

    useEffect(() => {
        if (!svgRef.current) return;
        if (lines.length === 0) return;

        // Aggregate all points from all lines
        const allPoints: Point[] = lines.flatMap((line) => line.data);

        // Find min/max values for scales
        const xMinNew = d3.min(allPoints, (d: Point) => d.x) ?? 0;
        const xMaxNew = d3.max(allPoints, (d: Point) => d.x) ?? 40;
        const yMinNew = 0;
        const yMaxNew = d3.max(allPoints, (d: Point) => d.y) ?? 0.03;

        // Update state
        setXMin(xMinNew);
        setXMax(xMaxNew);
        setYMin(yMinNew);
        setYMax(yMaxNew);
        console.log('XY range:', xMinNew, xMaxNew, yMinNew, yMaxNew);

        // Set scales based on min/max values
        const xScale = d3.scaleLinear()
            .domain([xMinNew, xMaxNew])
            .range([margin.left, width - margin.right]);

        const yScale = d3.scaleLinear()
            .domain([yMinNew, yMaxNew])
            .range([height - margin.bottom, margin.top]);

        // SVGの作成
        const svg = d3.select(svgRef.current)
            .attr('viewBox', `0 0 ${width} ${height}`)
            .attr('preserveAspectRatio', 'xMidYMid meet');

        // Clear SVG
        svg.selectAll('*').remove();

        // Add X axis
        svg.append('g')
            .attr('class', 'x-axis')
            .attr('transform', `translate(0,${height - margin.bottom})`)
            .call(d3.axisBottom(xScale))
            .append('text')
            .attr('x', width / 2)
            .attr('y', 27)
            .attr('fill', 'black')
            .attr('font-size', '8px')
            .text('Dry-bulb temperature [°C]');

        svg.select('.x-axis')
            .selectAll('.tick text')
            .style('font-size', '8px');

        // Add Y axis
        svg.append('g')
            .attr('class', 'y-axis')
            .attr('transform', `translate(${width - margin.right},0)`)
            .call(d3.axisRight(yScale))
            .append('text')
            .attr('transform', 'rotate(-90)')
            .attr('y', 45)
            .attr('x', -height / 2 - 30)
            .attr('fill', 'black')
            .attr('font-size', '8px')
            .text('Humidity ratio [kg/kg]');

        svg.select('.y-axis')
            .selectAll('.tick text')
            .style('font-size', '8px');

        // Create grid container with clip path
        const gridContainer = svg.append("g")
            .attr("clip-path", "url(#grid-clip)");

        // Add vertical grid lines
        const xGrid = d3.axisBottom(xScale)
            .tickSize(height)
            .tickFormat(() => '')
            .ticks(12);

        gridContainer.append('g')
            .attr('class', 'grid vertical-grid')
            .call(xGrid);

        // Add minor vertical grid lines
        const majorXTicks = xScale.ticks(12);
        const allXTicks = xScale.ticks(60);
        const minorXTicks = allXTicks.filter(tick => !majorXTicks.includes(tick));

        gridContainer.append('g')
            .attr('class', 'grid vertical-minor-grid')
            .selectAll('line')
            .data(minorXTicks)
            .enter()
            .append('line')
            .attr('x1', d => xScale(d))
            .attr('x2', d => xScale(d))
            .attr('y1', margin.top)
            .attr('y2', height - margin.bottom)
            .attr('stroke', '#eeeeee')
            .attr('stroke-width', 0.5);

        // Add horizontal grid lines
        const yGrid = d3.axisRight(yScale)
            .tickSize(width)
            .tickFormat(() => '')
            .ticks(10);

        gridContainer.append('g')
            .attr('class', 'grid horizontal-grid')
            .call(yGrid);

        // Style for grid lines
        const style = document.createElement('style');
        style.textContent = `
                .vertical-grid line,
                .horizontal-grid line {
                    stroke: #aaaaaa;
                    stroke-width: 0.5;
                }
            `;
        document.head.appendChild(style);

        // Create line generator
        const lineFunc = d3.line<Point>()
            .x(d => xScale(d.x))
            .y(d => yScale(d.y))
            .curve(d3.curveCatmullRom);

        lines.forEach(lineData => {
            // Add spline path
            svg.append('path')
                .datum(lineData.data)
                .attr('fill', 'none')
                .attr('stroke', '#666666')
                .attr('stroke-width', 0.5)
                .attr('d', lineFunc);

            // Add rH labels at the end of each line
            const lastPoint = lineData.data[lineData.data.length - 1];
            svg.append('text')
                .attr('x', xScale(lastPoint.x) - 5)
                .attr('y', yScale(lastPoint.y))
                .attr('fill', '#666666')
                .attr('font-size', '6px')
                .attr('text-anchor', 'end')
                .text(`${lineData.label}`);
        });

        // Create a clipping path
        const rh100Line = lines[lines.length - 1];
        console.log(rh100Line);
        svg.append("defs")
            .append("clipPath")
            .attr("id", "grid-clip")
            .append("path")
            .datum([
                // Start from top-right corner
                { x: xMax, y: yMin },
                // Add all points from RH100% line in reverse order
                ...rh100Line.data.slice().reverse(),
                // Complete the polygon by going around the chart edges
                { x: xMin, y: yMin },
                { x: xMax, y: yMin }
            ])
            .attr("d", d3.line<{ x: number, y: number }>()
                .x(d => xScale(d.x))
                .y(d => yScale(d.y))
                .curve(d3.curveLinear) // Use linear interpolation for the closing edges
            );
    }, [lines]);

    useEffect(() => {
        if (!svgRef.current) return;
        if (!initialState) return;

        const svg = d3.select(svgRef.current);

        const xScale = d3.scaleLinear()
            .domain([xMin, xMax])
            .range([margin.left, width - margin.right]);

        const yScale = d3.scaleLinear()
            .domain([yMin, yMax])
            .range([height - margin.bottom, margin.top]);

        // Add tooltip div
        const tooltip = d3.select('body')
            .append('div')
            .attr('class', 'tooltip')
            .style('opacity', 0)
            .style('position', 'absolute')
            .style('background-color', 'white')
            .style('border', '1px solid #ddd')
            .style('padding', '5px')
            .style('border-radius', '3px')
            .style('font-size', '12px')
            .style('pointer-events', 'none');

        // Add circle for initial state
        svg.selectAll('.initial-state-point').remove();
        svg.append('circle')
            .attr('cx', xScale(initialState.temperature))
            .attr('cy', yScale(initialState.humidityRatio))
            .attr('r', 3)
            .attr('fill', 'white')
            .attr('stroke', 'black')
            .attr('stroke-width', 2)
            .attr('class', 'initial-state-point')
            .on('mouseover', (event) => {
                tooltip.transition()
                    .duration(200)
                    .style('opacity', .9);
                tooltip.html(`Temperature: ${initialState.temperature.toFixed(1)}°C<br/>` +
                    `Humidity Ratio: ${initialState.humidityRatio.toFixed(4)} kg/kg`)
                    .style('left', (event.pageX + 10) + 'px')
                    .style('top', (event.pageY - 28) + 'px');
            })
            .on('mouseout', () => {
                tooltip.transition()
                    .duration(500)
                    .style('opacity', 0);
            });

    }, [initialState]);

    return (
        <div className="w-full">
            <svg ref={svgRef} className="w-full h-auto"></svg>
            <Button onClick={exportSVG} className="mb-2">
                <Download className="mr-1" />
                Export SVG
            </Button>
        </div>
    );
}
export default Chart;