let flock;
var numWaves = 10;
var canvas;


function windowResized() {
	//console.log('resized');
	canvas = resizeCanvas(windowWidth, windowHeight/8);
  }


function setup() {
  canvas = createCanvas(windowWidth, windowHeight/8);
  canvas.parent("wavesketch")
};

function draw() {
  background(245,245,245);
  randomSeed(0);
  for(var i = 0 ; i < numWaves; i++){
	fill(85, 149, 176,map(i,0,numWaves,192,32));
	noStroke();
	drawSineWave(4*PI,0.0001 * (random(0,numWaves)),1 + (5 * random(0,numWaves)),20,width,height/2);
  }
  fill(255);
}
function drawSineWave(radians,speed,amplitude,detail,size,y){
	beginShape();
	vertex(0,height);//fix to bottom
	//compute the distance between each point
	var xoffset = size / detail;
	var angleIncrement = radians / detail;
	for(var i = 0 ; i <= detail; i++){
	  var px = xoffset * i;
	  var py = y + (sin((millis() * speed) + angleIncrement * i) * amplitude);
	  vertex(px,py);
	}
	vertex(size,height);//fix to bottom
	endShape();
  }
