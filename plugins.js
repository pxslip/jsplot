/*jshint camelcase: false, unused: false, bitwise: false */
// Avoid `console` errors in browsers that lack a console.
(function() {
	'use strict';
	var method;
	var noop = function () {};
	var methods = [
		'assert', 'clear', 'count', 'debug', 'dir', 'dirxml', 'error',
		'exception', 'group', 'groupCollapsed', 'groupEnd', 'info', 'log',
		'markTimeline', 'profile', 'profileEnd', 'table', 'time', 'timeEnd',
		'timeStamp', 'trace', 'warn'
	];
	var length = methods.length;
	var console = (window.console = window.console || {});

	while (length--) {
		method = methods[length];

		// Only stub undefined methods.
		if (!console[method]) {
			console[method] = noop;
		}
	}
}());

Function.prototype.extend = function(parent) {
	'use strict';
	var Child = this;
	Child.prototype = parent;
	Child.prototype.$super = (parent === this && parent.$super) ? parent.$super : parent;
	Child.prototype = new Child(Array.prototype.slice.call(arguments,1));
	Child.prototype.constructor = Child;
};

Math.isNumeric = function (n) {
	'use strict';
	return !isNaN(parseFloat(n)) && isFinite(n);
};

//source:http://stackoverflow.com/a/17156580
Math.getParts = function(x) {
	'use strict';
	var float = new Float64Array(1),
		bytes = new Uint8Array(float.buffer);

	float[0] = x;

	var sign = bytes[7] >> 7,
		exponent = ((bytes[7] & 0x7f) << 4 | bytes[6] >> 4) - 0x3ff;

	bytes[7] = 0x3f;
	bytes[6] |= 0xf0;

	return {
		sign: sign,
		exponent: exponent,
		mantissa: float[0],
	};
};

if(!Math.log10) {
	Math.log10 = function(x){
		'use strict';
		return Math.log(x) / Math.LN10;
	};
}

Math.orderOfMagnitude = function(x) {
	'use strict';
	return Math.floor(Math.log10(Math.abs(x)));
};


Math.jsPlotPrettifyNumber = function(x, round) {
	'use strict';
	var exponent, fraction, nice_fraction;
	exponent = Math.floor(Math.log10(x));
	fraction = x / Math.pow(10, exponent);
	if (round) {
		if (fraction < 1.5) {
			nice_fraction = 1;
		}
		else if (fraction < 3) {
			nice_fraction = 2;
		}
		else if (fraction < 7) {
			nice_fraction = 5;
		}
		else {
			nice_fraction = 10;
		}
	} else {
		if (fraction <= 1) {
			nice_fraction = 1;
		}
		else if (fraction <= 2) {
			nice_fraction = 2;
		}
		else if (fraction <= 5) {
			nice_fraction = 5;
		}
		else {
			nice_fraction = 10;
		}
	}

    return nice_fraction * Math.pow(10, exponent);
};

HTMLCanvasElement.prototype.elementCoords = function(event){
	'use strict';
	var totalOffsetX = 0;
	var totalOffsetY = 0;
	var canvasX = 0;
	var canvasY = 0;
	var currentElement = this;

	while(currentElement){
		totalOffsetX += currentElement.offsetLeft - currentElement.scrollLeft;
		totalOffsetY += currentElement.offsetTop - currentElement.scrollTop;
		currentElement = currentElement.offsetParent;
	}

	canvasX = event.pageX - totalOffsetX;
	canvasY = event.pageY - totalOffsetY;
	return [canvasX, canvasY];
};

// Place any jQuery/helper plugins in here.
