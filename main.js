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

			return null
		})
}

function patterns() {
	return loopN(10, redWave)
		.then(loopN(5, redGreenOrangePulse))
		.then(patterns)
	;
}

init().then(patterns)

setInterval(function() {

}, frametime)