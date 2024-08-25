/**************************************************
 * The Poetry Clouds by Kyle Geske (stungeye.com) *
 **************************************************/
// Modified by Graham Lancaster

const cloudPixelScale = 9;
const cloudCutOff = 0.40;
const panSpeed = 70;
const cloudEvolutionSpeed = 30;

const mapChar = "                                                   `.-':_,^=;><+!rc*/?)7(|{}31[5]26948#$0%&@";

function setup() {
  createCanvas(windowWidth, windowHeight);
  textSize(cloudPixelScale * 1.15);  // Precompute text size
}

function draw() {
  background(31);  // Use minimal background updates

  let tinyTimeOffset = millis() / 1000000;  // Precompute time offset
  let noiseScale = 0.0020;

  for (let x = 0; x <= width; x += cloudPixelScale) {
    for (let y = 0; y <= height; y += cloudPixelScale) {
      let n = noise(
        x * noiseScale + tinyTimeOffset * panSpeed,
        y * noiseScale + tinyTimeOffset * 0.25 * panSpeed,
        tinyTimeOffset * cloudEvolutionSpeed
      );

      if (n >= cloudCutOff) {
        fill(255);  // Only call fill when needed
        text(mapChar.charAt(int(n * 93)), x, y);
      }
    }
  }
}

function windowResized() {
  resizeCanvas(windowWidth, windowHeight);
  redraw();
}
