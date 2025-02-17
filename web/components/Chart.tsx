"use client";

import { useEffect, useRef, useState } from 'react';
import * as d3 from 'd3';
import { Point, Line, State } from '../app/page';
import { Button } from './ui/button';
import { Download } from "lucide-react"; // アイコン用

interface ChartProps {
    rhLines: Line[];
    enthalpyLines: Line[];
    states: State[];
}

const width = 400;
const height = 300;
const margin = { top: 5, right: 55, bottom: 40, left: 10 };
const xMin = -15.0;
const xMax = 40.0;
const yMin = 0.0;
const yMax = 0.03;


const Chart = ({ rhLines, enthalpyLines, states }: ChartProps) => {
    const svgRef = useRef<SVGSVGElement>(null);
    const [chartInit, setChartInit] = useState(false);

    //////////////////////////////////////////////////////////////////////////////////////////////////
    // exporting SVG, called when the button is clicked
    //////////////////////////////////////////////////////////////////////////////////////////////////
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

    //////////////////////////////////////////////////////////////////////////////////////////////////
    // exporting SVG, called when the button is clicked
    //////////////////////////////////////////////////////////////////////////////////////////////////
    useEffect(() => {
        if (!svgRef.current) return;
        if (rhLines.length === 0) return;
        // if (enthalpyLines.length === 0) return;

        // Set scales based on min/max values
        const xScale = d3.scaleLinear()
            .domain([xMin, xMax])
            .range([margin.left, width - margin.right]);

        const yScale = d3.scaleLinear()
            .domain([yMin, yMax])
            .range([height - margin.bottom, margin.top]);

        // SVG container
        const svg = d3.select(svgRef.current)
            .attr('viewBox', `0 0 ${width} ${height}`)
            .attr('preserveAspectRatio', 'xMidYMid meet');

        // Clear SVG
        svg.selectAll('*').remove();

        // Add clip path
        svg.append('defs')
            .append('clipPath')
            .attr('id', 'plot-area-clip')
            .append('rect')
            .attr('x', margin.left)
            .attr('y', margin.top)
            .attr('width', width - margin.left - margin.right)
            .attr('height', height - margin.top - margin.bottom);

        const rh100Line = rhLines[rhLines.length - 1];
        svg.append('defs')
            .append('clipPath')
            .attr('id', 'grid-area-clip')
            .append('path')
            .datum([
                { x: xMax, y: yMax },
                { x: xMax, y: yMin },
                { x: xMin, y: yMin },
                ...rh100Line.data
            ])
            .attr('d', d3.line<{ x: number; y: number }>()
                .x(d => xScale(d.x))
                .y(d => yScale(d.y))
                .curve(d3.curveLinear));

        /// グリッドを描画
        const gridContainer = svg.append('g')
            .attr('clip-path', 'url(#grid-area-clip)');

        // プロット領域を作成
        const plotArea = svg.append('g')
            .attr('clip-path', 'url(#plot-area-clip)');

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

        // Add vertical grid lines
        const xGrid = d3.axisBottom(xScale)
            .tickSize(-(height - margin.top - margin.bottom)) // マイナスを付けて上向きに
            .tickFormat(() => '')
            .ticks(12);

        gridContainer.append('g')
            .attr('class', 'grid vertical-grid')
            .attr('transform', `translate(0,${height - margin.bottom})`) // 位置調整
            .call(xGrid);

        // Add horizontal grid lines
        const yGrid = d3.axisRight(yScale)
            .tickSize(-(width - margin.left - margin.right)) // マイナスを付けて左向きに
            .tickFormat(() => '')
            .ticks(10);

        gridContainer.append('g')
            .attr('class', 'grid horizontal-grid')
            .attr('transform', `translate(${width - margin.right},0)`) // 位置調整
            .call(yGrid);

        // --- Add vertical minor grid lines ---
        const xMajorTicks = xScale.ticks(12);
        const xMinorTicks = xScale.ticks(12 * 4).filter(t =>
            !xMajorTicks.some(mt => Math.abs(mt - t) < 1e-6)
        );

        const xMinorAxis = d3.axisBottom(xScale)
            .tickValues(xMinorTicks)
            .tickSize(-(height - margin.top - margin.bottom))
            .tickFormat(() => '');

        gridContainer.append('g')
            .attr('class', 'grid minor-vertical-grid')
            .attr('transform', `translate(0,${height - margin.bottom})`)
            .call(xMinorAxis);

        // --- Add horizontal minor grid lines ---
        const yMajorTicks = yScale.ticks(10);
        const yMinorTicks = yScale.ticks(10 * 4).filter(t =>
            !yMajorTicks.some(mt => Math.abs(mt - t) < 1e-6)
        );

        const yMinorAxis = d3.axisRight(yScale)
            .tickValues(yMinorTicks)
            .tickSize(-(width - margin.left - margin.right))
            .tickFormat(() => '');

        gridContainer.append('g')
            .attr('class', 'grid minor-horizontal-grid')
            .attr('transform', `translate(${width - margin.right},0)`)
            .call(yMinorAxis);

        // Style for grid lines
        const style = document.createElement('style');
        style.textContent = `
                .vertical-grid line,
                .horizontal-grid line {
                    stroke: #aaaaaa;
                    stroke-width: 0.5;
                }
                .minor-vertical-grid line,
                .minor-horizontal-grid line {
                    stroke: #cccccc;
                    stroke-width: 0.3;
                }
            `;
        document.head.appendChild(style);

        // plot RH lines
        // Create line generator
        const rhLineFunc = d3.line<Point>()
            .x(d => xScale(d.x))
            .y(d => yScale(d.y))
            .curve(d3.curveCatmullRom);


        rhLines.forEach(lineData => {
            // Add spline path
            plotArea.append('path')
                .datum(lineData.data)
                .attr('fill', 'none')
                .attr('stroke', '#666666')
                .attr('stroke-width', 0.5)
                .attr('d', rhLineFunc);

            // Add RH labels at the end of each line
            const lastPoint = lineData.data[lineData.data.length - 1];
            svg.append('text')
                .attr('x', xScale(lastPoint.x) - 5)
                .attr('y', yScale(lastPoint.y))
                .attr('fill', '#666666')
                .attr('font-size', '6px')
                .attr('text-anchor', 'end')
                .text(`${lineData.label}`);
        });

        // plot enthalpy lines
        // Create line generator
        const enthalpyLineFunc = d3.line<Point>()
            .x(d => xScale(d.x))
            .y(d => yScale(d.y))
            .curve(d3.curveLinear);

        enthalpyLines.forEach(lineData => {
            // Add line
            gridContainer.append('path')
                .datum(lineData.data)
                .attr('fill', 'none')
                .attr('stroke', '#666666')
                .attr('stroke-width', 0.5)
                .attr('d', enthalpyLineFunc);

            // Add enthalpy labels at the end of each line
            const lastPoint = lineData.data[lineData.data.length - 3];
            svg.append('text')
                .attr('x', xScale(lastPoint.x) + 5)
                .attr('y', yScale(lastPoint.y))
                .attr('fill', '#666666')
                .attr('font-size', '6px')
                .attr('text-anchor', 'start')
                .text(`${lineData.label}`);
        });

        setChartInit(true);
    }, [rhLines, enthalpyLines]);

    //////////////////////////////////////////////////////////////////////////////////////////////////
    // Plot points for each state
    //////////////////////////////////////////////////////////////////////////////////////////////////
    useEffect(() => {
        if (!svgRef.current) return;
        if (states.length === 0) return;
        if (!chartInit) return;

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
            .attr('cx', xScale(states[0].tDryBulb))
            .attr('cy', yScale(states[0].humidityRatio))
            .attr('r', 3)
            .attr('fill', 'white')
            .attr('stroke', 'black')
            .attr('stroke-width', 2)
            .attr('class', 'initial-state-point')
            .on('mouseover', (event) => {
                tooltip.transition()
                    .duration(200)
                    .style('opacity', .9);
                tooltip.html(`Temperature: ${states[0].tDryBulb.toFixed(1)}°C<br/>` +
                    `Humidity Ratio: ${states[0].humidityRatio.toFixed(4)} kg/kg`)
                    .style('left', (event.pageX + 10) + 'px')
                    .style('top', (event.pageY - 28) + 'px');
            })
            .on('mouseout', () => {
                tooltip.transition()
                    .duration(500)
                    .style('opacity', 0);
            });
        console.log('Initial state:', states[0]);

    }, [states]);

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