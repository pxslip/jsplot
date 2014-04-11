/**
 * jsPlot (c) 2014 Will Kruse
 * jsPlot is a web-based replacement for xPlot or jPlot. In theory this will run on any server, or locally
 * NOTE: at the time of creation only some browsers support the File api
 *
 * jsPlot is released under the MIT License whose text can be found in the included LICENSE file
 */
/*jshint camelcase: false, unused: false */
/*global isNumber, Colors, ColorLookup, PlotData, CanvasHandler*/
var TextCmd = {
	init:function(x_coord, y_coord, text, align, baseline, color) {
		'use strict';
		this.x_coord = parseFloat(x_coord);
		this.y_coord = parseFloat(y_coord);
		this.text = text;
		this.align = align;
		this.baseline = baseline;
		if(color && Math.isNumeric(color)) {
			this.color = '#'+Colors[ColorLookup[color]];
		} else if(color) {
			this.color = '#'+Colors[color];
		}
	},
	draw:function(ctx) {
		'use strict';
		var px_coords;
		px_coords = CanvasHandler.unitCoordToPxCoord(this.x_coord, this.y_coord);
		this.x_px_coord = px_coords[0]+this.offset[0];
		this.y_px_coord = px_coords[1]+this.offset[1];
		if(this.color) {
			ctx.fillStyle = this.color;
			ctx.strokeStyle = this.color;
		} else {
			ctx.fillStyle = PlotData.curr_color;
			ctx.strokeStyle = PlotData.curr_color;
		}
		ctx.textAlign = this.align;
		ctx.textBaseline = this.baseline;
		ctx.fillText(this.text, this.x_px_coord, this.y_px_coord);
	}
};

var AboveText = function(x_coord, y_coord, text, color) {
	'use strict';
	TextCmd.init.call(this, x_coord, y_coord, text, 'center', 'bottom', color);
	this.draw = function(ctx) {
		this.offset = [0, -5];
		TextCmd.draw.call(this, ctx);
	};
};

var BelowText = function(x_coord, y_coord, text, color) {
	'use strict';
	TextCmd.init.call(this, x_coord, y_coord, text, 'center', 'top', color);
	this.draw = function(ctx) {
		this.offset = [0, 5];
		TextCmd.draw.call(this, ctx);
	};
};

var CenterText = function(x_coord, y_coord, text, color) {
	'use strict';
	TextCmd.init.call(this, x_coord, y_coord, text, 'center', 'middle', color);
	this.draw = function(ctx) {
		this.offset = [0, 0];
		TextCmd.draw.call(this, ctx);
	};
};

var LeftText = function(x_coord, y_coord, text, color) {
	'use strict';
	TextCmd.init.call(this, x_coord, y_coord, text, 'right', 'middle', color);
	this.draw = function(ctx) {
		this.offset = [-5, 0];
		TextCmd.draw.call(this, ctx);
	};
};

var RightText = function(x_coord, y_coord, text, color) {
	'use strict';
	TextCmd.init.call(this, x_coord, y_coord, text, 'left', 'middle', color);
	this.draw = function(ctx) {
		this.offset = [5, 0];
		TextCmd.draw.call(this, ctx);
	};
};
