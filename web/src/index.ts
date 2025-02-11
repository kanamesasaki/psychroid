import * as d3 from 'd3';
import init, { get_relative_humidity_line } from '../../pkg/psychroid';

interface DataPoint {
    x: number;
    y: number;
    rh: number;
}

// Create array of RH values from 0.1 to 1.0
const rhValues = Array.from({ length: 10 }, (_, i) => (i + 1) * 0.1);
const hValues = Array.from({ length: 11 }, (_, i) => (i - 1) * 10000);

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
// const margin = { top: 20, right: 60, bottom: 30, left: 50 };
// const width = 800 - margin.left - margin.right;
// const height = 600 - margin.top - margin.bottom;

// Set up the chart dimensions
function getChartDimensions() {
    const container = document.getElementById('chart-container');
    const width = container?.clientWidth ?? 800;
    const height = container?.clientHeight ?? 600;
    const margin = { top: 20, right: 60, bottom: 30, left: 50 };

    return {
        width: width - margin.left - margin.right,
        height: height - margin.top - margin.bottom,
        margin
    };
}

let { width, height, margin } = getChartDimensions();

// Create SVG element
const svg = d3.select('#chart-container')
    .append('svg')
    .attr('width', '100%')
    .attr('height', '100%')
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


const rh100Line = allLines[allLines.length - 1]; // Get the 100% RH line data

// Create a clipping path that includes the area below and to the right of RH100% line
svg.append("defs")
    .append("clipPath")
    .attr("id", "grid-clip")
    .append("path")
    .datum([
        // Start from top-right corner
        { x: xMax, y: yMin },
        // Add all points from RH100% line in reverse order
        ...rh100Line.slice().reverse(),
        // Complete the polygon by going around the chart edges
        { x: xMin, y: yMin },
        { x: xMax, y: yMin }
    ])
    .attr("d", d3.line<{ x: number, y: number }>()
        .x(d => xScale(d.x))
        .y(d => yScale(d.y))
        .curve(d3.curveLinear) // Use linear interpolation for the closing edges
    );

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
    .grid line {
        stroke: #dddddd;
        stroke-width: 0.5;
    }
    .grid path {
        stroke: none;
    }
`;
document.head.appendChild(style);

// Add tooltip div
const tooltip = d3.select('body')
    .append('div')
    .attr('class', 'tooltip')
    .style('opacity', 0)
    .style('position', 'absolute')
    .style('background-color', 'white')
    .style('border', '1px solid #ddd')
    .style('padding', '10px')
    .style('border-radius', '3px')
    .style('pointer-events', 'none');

// Update addPoint function to include hover effects
function addPoint(temperature: number, humidityRatio: number) {
    svg.append('circle')
        .attr('cx', xScale(temperature))
        .attr('cy', yScale(humidityRatio))
        .attr('r', 4)
        .attr('fill', 'red')
        .attr('stroke', 'black')
        .attr('stroke-width', 1)
        .on('mouseover', (event) => {
            tooltip.transition()
                .duration(200)
                .style('opacity', .9);
            tooltip.html(`Temperature: ${temperature.toFixed(1)}°C<br/>` +
                `Humidity Ratio: ${humidityRatio.toFixed(4)} kg/kg`)
                .style('left', (event.pageX + 10) + 'px')
                .style('top', (event.pageY - 28) + 'px');
        })
        .on('mouseout', () => {
            tooltip.transition()
                .duration(500)
                .style('opacity', 0);
        });
}

// Add event listener for the plot button
document.getElementById('plot-point')?.addEventListener('click', () => {
    const tempInput = document.getElementById('temperature') as HTMLInputElement;
    const humidityInput = document.getElementById('humidity-ratio') as HTMLInputElement;

    const temperature = parseFloat(tempInput.value);
    const humidityRatio = parseFloat(humidityInput.value);

    if (!isNaN(temperature) && !isNaN(humidityRatio)) {
        addPoint(temperature, humidityRatio);
    } else {
        alert('Please enter valid numbers');
    }
});

