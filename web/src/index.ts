import * as d3 from 'd3';
import init, { get_relative_humidity_line } from '../../pkg/psychroid';

interface DataPoint {
    x: number;
    y: number;
    rh: number;
}

// Create array of RH values from 0.1 to 1.0
const rhValues = Array.from({ length: 10 }, (_, i) => (i + 1) * 0.1);

// Generate data for all RH lines
const allLines = rhValues.map(rh => {
    const data = get_relative_humidity_line(
        rh,        // RH value (0.1 to 1.0)
        101325.0,  // Standard pressure
        true       // Use SI units
    );

    return Array.from(data.temperatures).map((t, i) => ({
        x: t,
        y: data.humidity_ratios[i],
        rh: rh * 100
    }));
});

// Find global min/max values for scales
const xMin = d3.min(allLines, line => d3.min(line, (d: DataPoint) => d.x)) ?? 0;
const xMax = d3.max(allLines, line => d3.max(line, (d: DataPoint) => d.x)) ?? 100;
const yMin = 0;
const yMax = d3.max(allLines, line => d3.max(line, (d: DataPoint) => d.y)) ?? 0.03;

// Set up the chart dimensions
const margin = { top: 20, right: 60, bottom: 30, left: 50 };
const width = 800 - margin.left - margin.right;
const height = 600 - margin.top - margin.bottom;

// Create SVG element
const svg = d3.select('body')
    .append('svg')
    .attr('width', width + margin.left + margin.right)
    .attr('height', height + margin.top + margin.bottom)
    .append('g')
    .attr('transform', `translate(${margin.left},${margin.top})`);

// Create scales
const xScale = d3.scaleLinear<number, number>()
    .domain([xMin, xMax])
    .range([0, width]);

const yScale = d3.scaleLinear<number, number>()
    .domain([yMin, yMax])
    .range([height, 0]);

// Create line generator
const line = d3.line<{ x: number, y: number }>()
    .x(d => xScale(d.x))
    .y(d => yScale(d.y))
    .curve(d3.curveCatmullRom);

// Add X axis
svg.append('g')
    .attr('transform', `translate(0,${height})`)
    .call(d3.axisBottom(xScale))
    .append('text')
    .attr('x', width / 2)
    .attr('y', 30)
    .attr('fill', 'black')
    .text('Dry-bulb Temperature (°C)');

// Add Y axis
svg.append('g')
    .attr('transform', `translate(${width},0)`)  // Move to right side
    .call(d3.axisRight(yScale))  // Change to axisRight
    .append('text')
    .attr('transform', 'rotate(-90)')
    .attr('x', - height / 2 - 50)
    .attr('y', 50)  // Adjusted position
    .attr('fill', 'black')
    .text('Humidity Ratio (kg/kg)');

// Create color scale for different RH lines
const colorScale = d3.scaleSequential(d3.interpolateBlues)
    .domain([0, rhValues.length - 1]);

// Add all RH lines
allLines.forEach((points, i) => {
    // Add the spline path
    svg.append('path')
        .datum(points)
        .attr('fill', 'none')
        .attr('stroke', '#666666')
        .attr('stroke-width', 0.5)
        .attr('d', line);

    // Add RH labels at the end of each line
    const lastPoint = points[points.length - 1];
    svg.append('text')
        .attr('x', xScale(lastPoint.x) - 25)
        .attr('y', yScale(lastPoint.y))
        .attr('fill', '#666666')
        .attr('font-size', '10px')
        .text(`${Math.round(lastPoint.rh)}%`);
});