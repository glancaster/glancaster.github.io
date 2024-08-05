// List of p5.js sketch files you want to choose from
const sketches = [
  "V1/sketches/lsystem.js",
  "V1/sketches/flock.js",
  "V1/sketches/gameoflife.js",
  "V1/sketches/brownian.js",
  "V1/sketches/particle.js",
];

// Randomly select a p5.js sketch file
const randomSketchIndex = Math.floor(Math.random() * sketches.length);
const randomSketch = sketches[randomSketchIndex];

// Dynamically load the selected p5.js sketch file
const scriptElement = document.createElement("script");
scriptElement.src = randomSketch;
document.getElementById("p5sketch").appendChild(scriptElement);
