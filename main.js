let iter = 0;
const frametime = 16;

function flush(data) {
	const freq = 1000;

	analogWrite(A5, data[0], { freq : 1000 });
	analogWrite(A6, data[1], { freq : 1000 });
	analogWrite(A7, data[2], { freq : 1000 });
	analogWrite(B1, data[3], { freq : 1000 });
	analogWrite(B10, data[4], { freq : 1000 });
	analogWrite(B13, data[5], { freq : 1000 });
	analogWrite(B14, data[6], { freq : 1000 });
	analogWrite(B15, data[7], { freq : 1000 });
}

function testPattern(counter) {
	d = [
		Math.sin(counter) + 1,
		Math.cos(counter) + 1,
		Math.sin(counter) + 1,
		Math.cos(counter) + 1,
		Math.sin(counter) + 1,
		Math.cos(counter) + 1,
		Math.sin(counter) + 1,
		Math.cos(counter) + 1
	]

	flush(d)
}

function init() {
	return new Promise(function(resolve) {
		let counter = 0;

		let interval = setInterval(function() {
			flush([
				counter,
				0,
				counter,
				0,
				counter,
				0,
				counter,
				0,
			])

			counter += 0.005;

			if(counter >= 1) {
				clearInterval(interval)

				resolve()
			}
		}, frametime);
	})
}

function greenWave() {
	return new Promise(function(resolve) {
		let counter = 0;

		let interval = setInterval(function() {
			flush([
				0,
				Math.sin(counter) + 1,
				0,
				Math.sin(counter) + 1,
				0,
				Math.sin(counter) + 1,
				0,
				Math.sin(counter) + 1,
			])

			counter += 0.01;

			// if(counter >= 10) {
			// 	clearInterval(interval)

			// 	resolve()
			// }
		}, frametime);
	})
}

// init()
// 	.then(greenWave);

greenWave()

setInterval(function() {

}, frametime)