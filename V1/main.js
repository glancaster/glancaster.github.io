// List of p5.js sketch files you want to choose from
const sketches = [
  "sketches/lsystem.js",
  "sketches/flock.js",
  "sketches/gameoflife.js",
  "sketches/brownian.js",
  "sketches/particle.js",
];

// Randomly select a p5.js sketch file
const randomSketchIndex = Math.floor(Math.random() * sketches.length);
const randomSketch = sketches[randomSketchIndex];

// Dynamically load the selected p5.js sketch file
const scriptElement = document.createElement("script");
scriptElement.src = randomSketch;
document.getElementById("p5sketch").appendChild(scriptElement);
