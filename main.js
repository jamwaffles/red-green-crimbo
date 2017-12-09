const frametime = 16;
const freq = 5000;

function flush(data) {
	analogWrite(A5, data[0], { freq: freq });
	analogWrite(A6, data[1], { freq: freq });
	analogWrite(A7, data[2], { freq: freq });
	analogWrite(B1, data[3], { freq: freq });
	analogWrite(B10, data[4], { freq: freq });
	analogWrite(B13, data[5], { freq: freq });
	analogWrite(B14, data[6], { freq: freq });
	analogWrite(B15, data[7], { freq: freq });
}

function testPattern() {
	return new Promise(function(resolve) {
		let counter = 0;

		let interval = setInterval(function() {
		  counter += 0.05;

		  flush([
			Math.sin(counter) + 1,
			Math.cos(counter) + 1,
			Math.sin(counter) + 1,
			Math.cos(counter) + 1,
			Math.sin(counter) + 1,
			Math.cos(counter) + 1,
			Math.sin(counter) + 1,
			Math.cos(counter) + 1
		  ])

		  if(counter >= Math.PI * 2) {
		  	clearInterval(interval)

		  	resolve()
		  }
		}, frametime)
	})
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

function redWave() {
	return new Promise(function(resolve) {
		let counter = 0;

		const gimmeSin = function(i, offs) { return (Math.sin(i + offs) + 1) / 2 }

		let interval = setInterval(function() {
		  counter += 0.05;

		  flush([
			gimmeSin(counter, 0),
			0,
			gimmeSin(counter, Math.PI / 2),
			0,
			gimmeSin(counter, Math.PI),
			0,
			gimmeSin(counter, Math.PI * 1.5),
			0,
		  ])

		  if(counter >= Math.PI * 2) {
		  	clearInterval(interval)

		  	resolve()
		  }
		}, frametime)
	})
}

function greenWave() {
	return new Promise(function(resolve) {
		let counter = 0;

		const gimmeSin = function(i, offs) { return (Math.sin(i + offs) + 1) / 2 }

		let interval = setInterval(function() {
		  counter += 0.05;

		  flush([
			0,
			gimmeSin(counter, 0),
			0,
			gimmeSin(counter, Math.PI / 2),
			0,
			gimmeSin(counter, Math.PI),
			0,
			gimmeSin(counter, Math.PI * 1.5),
		  ])

		  if(counter >= Math.PI * 2) {
		  	clearInterval(interval)

		  	resolve()
		  }
		}, frametime)
	})
}


function redGreenOrangePulse() {
	return new Promise(function(resolve) {
		let counter = 0;
		let iter = 0;

		const gimmeColour = function(c, i, offs) {
			// Red
			if(i % 3 === 0) {
				return [
					(Math.sin(c + offs) + 1) / 2,
					0,
				]
			}
			// Green
			else if((i + 1) % 3 === 0) {
				return [
					0,
					(Math.sin(c + offs) + 1) / 2,
				]
			}
			// Orange
			else {
				return [
					(Math.sin(c + offs) + 1) / 2,
					(Math.sin(c + offs) + 1) / 2,
				]
			}

			return (Math.sin(c + offs) + 1) / 2
		}

		const baseOffset = Math.PI * 1.5

		let interval = setInterval(function() {
		  counter += 0.05;

		  flush(
			gimmeColour(counter, iter, baseOffset)
			.concat(
				gimmeColour(counter, iter, baseOffset),
				gimmeColour(counter, iter, baseOffset),
				gimmeColour(counter, iter, baseOffset)
			)
		  )

		  if(counter >= Math.PI * 2) {
		  	counter = 0;
		  	iter++;
		  }

		  if(iter >= 3) {
		  	clearInterval(interval)

		  	resolve()
		  }
		}, frametime)
	})
}

function loopN(num, func) {
	return func()
		.then(function() {
			if(num > 1) {
				return loopN(num - 1, func)
			}

			return Promise.resolve()
		})
}

function patterns() {
	return loopN(10, redWave)
		.then(function() {
			return loopN(5, redGreenOrangePulse)
		})
		.then(function() {
			return loopN(10, greenWave)
		})
		.then(function() {
			return loopN(10, testPattern)
		})
		.then(patterns)
	;
}

init().then(patterns)