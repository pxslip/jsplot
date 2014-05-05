var PlotData = {
	'command_array':undefined,
	'command_obj_array':[],
	'x_coord_type':undefined,
	'x_axis_label':undefined,
	'x_axis_units':undefined,
	'y_coord_type':undefined,
	'y_axis_label':undefined,
	'y_axis_units':undefined,
	'title':undefined,
	'orig_max_x_coord':undefined,
	'orig_min_x_coord':undefined,
	'curr_max_x_coord':undefined,
	'curr_min_x_coord':undefined,
	'prev_max_x_coord':[],
	'prev_min_x_coord':[],
	'drag_start_x_coord':undefined,
	'drag_end_x_coord':undefined,
	'orig_max_y_coord':undefined,
	'orig_min_y_coord':undefined,
	'curr_max_y_coord':undefined,
	'curr_min_y_coord':undefined,
	'drag_start_y_coord':undefined,
	'drag_end_y_coord':undefined,
	'prev_max_y_coord':[],
	'prev_min_y_coord':[],
	'max_ticks':50,
	'curr_color':'#000000',
	'axis_text':'10pt Arial',
	'is_dragging':false,
	reset: function() {
		'use strict';
		PlotData.command_array = undefined;
		PlotData.command_obj_array = [];
		PlotData.x_coord_type = undefined;
		PlotData.x_axis_label = undefined;
		PlotData.x_axis_units = undefined;
		PlotData.y_coord_type = undefined;
		PlotData.y_axis_label = undefined;
		PlotData.y_axis_units = undefined;
		PlotData.title = undefined;
		PlotData.orig_max_x_coord = undefined;
		PlotData.orig_min_x_coord = undefined;
		PlotData.curr_max_x_coord = undefined;
		PlotData.curr_min_x_coord = undefined;
		PlotData.prev_max_x_coord = [];
		PlotData.prev_min_x_coord = [];
		PlotData.drag_start_x_coord = undefined;
		PlotData.drag_end_x_coord = undefined;
		PlotData.orig_max_y_coord = undefined;
		PlotData.orig_min_y_coord = undefined;
		PlotData.curr_max_y_coord = undefined;
		PlotData.curr_min_y_coord = undefined;
		PlotData.drag_start_y_coord = undefined;
		PlotData.drag_end_y_coord = undefined;
		PlotData.prev_max_y_coord = [];
		PlotData.prev_min_y_coord = [];
		PlotData.curr_color = '#000000';
	}
};

var CanvasHandler = {
	'canvas':undefined,//pre-define these so you (and future me) know they are here
	'context':undefined,
	'consts': {
		'x_axis':'x',
		'y_axis':'y',
	},
	init:function(target, commands, coord_target) {
		'use strict';
		if(CanvasHandler.canvas) {
			CanvasHandler.detach();
		}
		CanvasHandler.canvas = document.getElementById(target);
		if(coord_target) {
			CanvasHandler.coord_target = document.getElementById(coord_target);
		}
		CanvasHandler.canvas.width = CanvasHandler.canvas.parentNode.offsetWidth - 100;
		CanvasHandler.canvas.height = Math.max(document.documentElement.clientHeight, window.innerHeight || 0) - 100;
		CanvasHandler.canvas.addEventListener('dblclick', CanvasHandler.dblclickListener);
		CanvasHandler.canvas.addEventListener('mousedown', CanvasHandler.mousedownListener);
		CanvasHandler.canvas.addEventListener('mouseup', CanvasHandler.mouseupListener);
		CanvasHandler.canvas.addEventListener('contextmenu', CanvasHandler.contextmenuListener);
		CanvasHandler.canvas.addEventListener('mousemove', CanvasHandler.mousemoveListener);
		CanvasHandler.context = CanvasHandler.canvas.getContext('2d');
		CanvasHandler.context.save();
		PlotData.reset();
		PlotData.command_array = commands;//store the array of commands for future reference
		PlotData.command_obj_array = CanvasHandler.parseLines(commands.split('\n'));
		CanvasHandler.redraw();
	},
	detach: function () {
		'use strict';
		CanvasHandler.canvas.removeEventListener('dblclick', CanvasHandler.dblclickListener);
		CanvasHandler.canvas.removeEventListener('mousedown', CanvasHandler.mousedownListener);
		CanvasHandler.canvas.removeEventListener('mouseup', CanvasHandler.mouseupListener);
		CanvasHandler.canvas.removeEventListener('contextmenu', CanvasHandler.contextmenuListener);
		CanvasHandler.canvas.removeEventListener('mousemove', CanvasHandler.mousemoveListener);
		PlotData.reset();
	},
	dblclickListener:function(evt) {
		'use strict';
		var unit_coords, new_x_range, new_y_range, px_coords;
		evt.preventDefault();
		//yoinked from jquery
		px_coords = CanvasHandler.canvas.elementCoords(evt);
		unit_coords = CanvasHandler.pxCoordToUnitCoord(px_coords[0], px_coords[1]);
		PlotData.prev_max_x_coord.push(PlotData.curr_max_x_coord);
		PlotData.prev_min_x_coord.push(PlotData.curr_min_x_coord);
		PlotData.prev_max_y_coord.push(PlotData.curr_max_y_coord);
		PlotData.prev_min_y_coord.push(PlotData.curr_min_y_coord);
		new_x_range = (PlotData.curr_max_x_coord - PlotData.curr_min_x_coord) / 4;
		new_y_range = (PlotData.curr_max_y_coord - PlotData.curr_min_y_coord) / 4;
		PlotData.curr_max_x_coord = unit_coords[0] + new_x_range;
		PlotData.curr_min_x_coord = unit_coords[0] - new_x_range;
		PlotData.curr_max_y_coord = unit_coords[1] + new_y_range;
		PlotData.curr_min_y_coord = unit_coords[1] - new_y_range;
		CanvasHandler.redraw();
		return false;
	},
	mousedownListener:function(evt) {
		'use strict';
		var new_x_range, new_y_range, unit_coords, px_coords;
		evt.preventDefault();
		if(evt.stopPropagation) {
			evt.stopPropagation();
		}
		if(evt.cancelBubble) {
			evt.cancelBubble = true;
		}
		if(evt.button && evt.button === 2) {
			//this is a right click so zoom out and redraw
			PlotData.curr_max_x_coord = (PlotData.prev_max_x_coord.length > 0) ? PlotData.prev_max_x_coord.pop() : PlotData.orig_max_x_coord;
			PlotData.curr_min_x_coord = (PlotData.prev_max_x_coord.length > 0) ? PlotData.prev_min_x_coord.pop() : PlotData.orig_min_x_coord;
			PlotData.curr_max_y_coord = (PlotData.prev_max_x_coord.length > 0) ? PlotData.prev_max_y_coord.pop() : PlotData.orig_max_y_coord;
			PlotData.curr_min_y_coord = (PlotData.prev_max_x_coord.length > 0) ? PlotData.prev_min_y_coord.pop() : PlotData.orig_min_y_coord;
			CanvasHandler.redraw();
			return false;
		} else {
			//handle drag start here
			px_coords = CanvasHandler.canvas.elementCoords(evt);
			unit_coords = CanvasHandler.pxCoordToUnitCoord(px_coords[0], px_coords[1]);
			PlotData.drag_start_x_coord = unit_coords[0];
			PlotData.drag_start_y_coord = unit_coords[1];
			PlotData.is_dragging = true;
		}
	},
	mouseupListener:function(evt) {
		'use strict';
		var unit_coords, x_diff, y_diff, px_coords;
		evt.preventDefault();
		PlotData.is_dragging = false;
		if(evt.button && evt.button !== 2) {
			px_coords = CanvasHandler.canvas.elementCoords(evt);
			unit_coords = CanvasHandler.pxCoordToUnitCoord(px_coords[0], px_coords[1]);
			PlotData.drag_end_x_coord = unit_coords[0];
			PlotData.drag_end_y_coord = unit_coords[1];
			x_diff = PlotData.drag_start_x_coord - PlotData.drag_end_x_coord;
			y_diff = PlotData.drag_start_y_coord - PlotData.drag_end_y_coord;
			PlotData.curr_max_x_coord = PlotData.curr_max_x_coord + x_diff;
			PlotData.curr_min_x_coord = PlotData.curr_min_x_coord + x_diff;
			PlotData.curr_max_y_coord = PlotData.curr_max_y_coord + y_diff;
			PlotData.curr_min_y_coord = PlotData.curr_min_y_coord + y_diff;
			CanvasHandler.redraw();
		}
		return false;
	},
	mousemoveListener:function(evt) {
		'use strict';
		var unit_coords, x_diff, y_diff, px_coords, hover_text;
		evt.preventDefault();
		px_coords = CanvasHandler.canvas.elementCoords(evt);
		unit_coords = CanvasHandler.pxCoordToUnitCoord(px_coords[0], px_coords[1]);
		if(PlotData.is_dragging) {//we're dealing with a drag event so redraw the updated canvas
			PlotData.drag_end_x_coord = unit_coords[0];
			PlotData.drag_end_y_coord = unit_coords[1];
			x_diff = PlotData.drag_start_x_coord - PlotData.drag_end_x_coord;
			y_diff = PlotData.drag_start_y_coord - PlotData.drag_end_y_coord;
			PlotData.curr_max_x_coord = PlotData.curr_max_x_coord + x_diff;
			PlotData.curr_min_x_coord = PlotData.curr_min_x_coord + x_diff;
			PlotData.curr_max_y_coord = PlotData.curr_max_y_coord + y_diff;
			PlotData.curr_min_y_coord = PlotData.curr_min_y_coord + y_diff;
			CanvasHandler.redraw();
		} else if(CanvasHandler.coord_target) {//no drag is occuring so show the current coords of the cursor
			CanvasHandler.coord_target.textContent = unit_coords[0].toPrecision(6)+', '+unit_coords[1].toPrecision(6);
		}
		return false;
	},
	contextmenuListener:function(evt) {
		'use strict';
		evt.preventDefault();
		return false;
	},
	redraw:function() {
		'use strict';
		var cmd_obj, i;
		CanvasHandler.context.restore();
		CanvasHandler.context.save();
		CanvasHandler.context.clearRect(0, 0, CanvasHandler.width(), CanvasHandler.height());
		CanvasHandler.drawAxes();
		CanvasHandler.drawAxisLabels();
		CanvasHandler.drawTitle();
		CanvasHandler.setClipBoundary();
		for (i = 0; i < PlotData.command_obj_array.length; i++) {
			cmd_obj = PlotData.command_obj_array[i];
			if(cmd_obj && cmd_obj.draw) {
				cmd_obj.draw(CanvasHandler.context);
			}
		}
	},
	width:function() {
		'use strict';
		return CanvasHandler.canvas.width;
	},
	height:function() {
		'use strict';
		return CanvasHandler.canvas.height;
	},
	parseLines: function (lines) {
		'use strict';
		var line, values, coord_types, skip_next, should_splice, i,
			commands = [];
		line = lines[0].trim();
		values = line.split(/\s+/);
		while(values.length === 1 && values[0] === 'new_plotter') {//what the HELL is new_plotter?????
			lines.shift();
			line = lines[0].trim();
			values = line.split(/\s+/);
		}
		//in theory the next set of lines after new_plotter are the coord types
		//TODO: check for incorrect data here
		coord_types = lines.shift().split(/\s+/);
		PlotData.x_coord_type = coord_types[0];
		PlotData.y_coord_type = coord_types[1];
		for (i = 0; i < lines.length; i++) {
			if(skip_next) {
				skip_next = false;
				continue;
			}
			line = lines[i].trim();
			values = line.split(/\s+/);
			if(line.length === 0) {//empty line check
				lines.splice(i, 1);//remove the empty line so we don't deal with it again
				i--;
				continue;
			} else if(values.length === 1 && values[0] !== 'go') {
				should_splice = false;
				//check if this line handles the titles or units for the axes
				if(values[0] === 'title') {
					//the next line has the plot title
					PlotData.title = lines[i+1];
					should_splice = true;
				} else if(values[0] === 'xlabel') {
					//the next line has the label for the x-axis
					PlotData.x_axis_label = lines[i+1];
					should_splice = true;
				} else if(values[0] === 'ylabel') {
					//the next line has the label for the y-axis
					PlotData.y_axis_label = lines[i+1];
					should_splice = true;
				} else if(values[0] === 'xunits') {
					//the next line has the units for the x-axis
					PlotData.x_axis_units = lines[i+1];
					should_splice = true;
				} else if(values[0] === 'yunits') {
					//the next line has the units for the y-axis
					PlotData.y_axis_units = lines[i+1];
					should_splice = true;
				} else {
					//in theory these are the color values so parse the line instead of removing and adding to PlotData
					commands.push(CanvasHandler.parseLine(values));
					should_splice = false;
				}
				if(should_splice) {
					lines.splice(i, 2);
					i = i-1;
				}
			} else {
				if(values[0] === 'ctext' || values[0] === 'ltext'|| values[0] === 'rtext'|| values[0] === 'atext'|| values[0] === 'btext') {
					values.push(lines[i+1]);
					skip_next = true;
				}
				commands.push(CanvasHandler.parseLine(values));
			}
		}
		return commands;
	},
	parseLine: function (values) {
		'use strict';
		var cmd, cmd_obj;
		cmd = values[0];
		switch(cmd) {
		case 'atext':
			if(values.length === 5) {
				cmd_obj = new AboveText(values[1], values[2], values[4], values[3]);
			} else {
				cmd_obj = new AboveText(values[1], values[2], values[3]);
			}
			CanvasHandler.checkRange(values[1], values[2]);
			break;
		case 'btext':
			if(values.length === 5) {
				cmd_obj = new BelowText(values[1], values[2], values[4], values[3]);
			} else {
				cmd_obj = new BelowText(values[1], values[2], values[3]);
			}
			CanvasHandler.checkRange(values[1], values[2]);
			break;
		case 'ctext':
			if(values.length === 5) {
				cmd_obj = new CenterText(values[1], values[2], values[4], values[3]);
			} else {
				cmd_obj = new CenterText(values[1], values[2], values[3]);
			}
			CanvasHandler.checkRange(values[1], values[2]);
			break;
		case 'ltext':
			if(values.length === 5) {
				cmd_obj = new LeftText(values[1], values[2], values[4], values[3]);
			} else {
				cmd_obj = new LeftText(values[1], values[2], values[3]);
			}
			CanvasHandler.checkRange(values[1], values[2]);
			break;
		case 'rtext':
			if(values.length === 5) {
				cmd_obj = new RightText(values[1], values[2], values[4], values[3]);
			} else {
				cmd_obj = new RightText(values[1], values[2], values[3]);
			}
			CanvasHandler.checkRange(values[1], values[2]);
			break;
		case 'dtick':
			cmd_obj = new DownTick(values[1], values[2], values[3]);
			CanvasHandler.checkRange(values[1], values[2]);
			break;
		case 'utick':
			cmd_obj = new UpTick(values[1], values[2], values[3]);
			CanvasHandler.checkRange(values[1], values[2]);
			break;
		case 'ltick':
			cmd_obj = new LeftTick(values[1], values[2], values[3]);
			CanvasHandler.checkRange(values[1], values[2]);
			break;
		case 'rtick':
			cmd_obj = new RightTick(values[1], values[2], values[3]);
			CanvasHandler.checkRange(values[1], values[2]);
			break;
		case 'htick':
			cmd_obj = new HorizTick(values[1], values[2], values[3]);
			CanvasHandler.checkRange(values[1], values[2]);
			break;
		case 'vtick':
			cmd_obj = new VertTick(values[1], values[2], values[3]);
			CanvasHandler.checkRange(values[1], values[2]);
			break;
		case 'diamond':
			cmd_obj = new Diamond(values[1], values[2], values[3]);
			CanvasHandler.checkRange(values[1], values[2]);
			break;
		case 'box':
			cmd_obj = new Box(values[1], values[2], values[3]);
			CanvasHandler.checkRange(values[1], values[2]);
			break;
		case 'uarrow':
			cmd_obj = new UpArrow(values[1], values[2], values[3]);
			CanvasHandler.checkRange(values[1], values[2]);
			break;
		case 'darrow':
			cmd_obj = new DownArrow(values[1], values[2], values[3]);
			CanvasHandler.checkRange(values[1], values[2]);
			break;
		case 'larrow':
			cmd_obj = new LeftArrow(values[1], values[2], values[3]);
			CanvasHandler.checkRange(values[1], values[2]);
			break;
		case 'rarrow':
			cmd_obj = new RightArrow(values[1], values[2], values[3]);
			CanvasHandler.checkRange(values[1], values[2]);
			break;
		case 'dot':
		case '.':
			cmd_obj = new Dot(values[1], values[2], values[3]);
			CanvasHandler.checkRange(values[1], values[2]);
			break;
		case 'invisible':
			cmd_obj = new Invisible(values[1], values[2]);
			CanvasHandler.checkRange(values[1], values[2]);
			break;
		case 'plus':
		case '+':
			cmd_obj = new Plus(values[1], values[2], values[3]);
			CanvasHandler.checkRange(values[1], values[2]);
			break;
		case 'x':
			cmd_obj = new Cross(values[1], values[2], values[3]);
			CanvasHandler.checkRange(values[1], values[2]);
			break;
		case 'line':
			cmd_obj = new Line(values[1], values[2], values[3], values[4], values[5]);
			CanvasHandler.checkRange(values[1], values[2]);
			CanvasHandler.checkRange(values[3], values[4]);
			break;
		case 'dline':
			cmd_obj = new DemarcatedLine(values[1], values[2], values[3], values[4], values[5]);
			CanvasHandler.checkRange(values[1], values[2]);
			CanvasHandler.checkRange(values[3], values[4]);
			break;
		case 'go':
			break;
		default:
			cmd_obj = new Color(values[0]);
			break;
		}
		return cmd_obj;
	},
	checkRange: function (x_coord, y_coord) {
		'use strict';
		x_coord = parseFloat(x_coord);
		y_coord = parseFloat(y_coord);
		if(x_coord > PlotData.orig_max_x_coord || PlotData.orig_max_x_coord === undefined) {
			PlotData.orig_max_x_coord = PlotData.curr_max_x_coord = x_coord;
		}
		if(x_coord < PlotData.orig_min_x_coord || PlotData.orig_min_x_coord === undefined) {
			PlotData.orig_min_x_coord = PlotData.curr_min_x_coord = x_coord;
		}
		if(y_coord > PlotData.orig_max_y_coord || PlotData.orig_max_y_coord === undefined) {
			PlotData.orig_max_y_coord = PlotData.curr_max_y_coord = y_coord;
		}
		if(y_coord < PlotData.orig_min_y_coord || PlotData.orig_min_y_coord === undefined) {
			PlotData.orig_min_y_coord = PlotData.curr_min_y_coord = y_coord;
		}
	},
	drawAxes: function () {
		'use strict';
		var ctx, x_axis_values, y_axis_values, i, unit_value, px_value;
		ctx = CanvasHandler.context;
		ctx.fillStyle = '#000000';
		ctx.strokeStyle = '#000000';
		ctx.lineWidth = 1.0;
		ctx.moveTo(CanvasHandler.yAxisCoords().end[0], CanvasHandler.yAxisCoords().end[1]);
		ctx.lineTo(CanvasHandler.yAxisCoords().start[0], CanvasHandler.yAxisCoords().start[1]);//draw the y-axis
		ctx.lineTo(CanvasHandler.xAxisCoords().end[0], CanvasHandler.xAxisCoords().end[1]);//draw the x-axis
		ctx.stroke();//fill the lines that were drawn
		ctx.font = PlotData.axis_text;//set the font size and family for the axis markers
		x_axis_values = CanvasHandler.getPrettyTickInfo(PlotData.curr_min_x_coord, PlotData.curr_max_x_coord);
		y_axis_values = CanvasHandler.getPrettyTickInfo(PlotData.curr_min_y_coord, PlotData.curr_max_y_coord);
		//start from bottom and add the calculated number of ticks
		for (i = 0; i <= x_axis_values.quantity; i++) {
			//x axis intermediate tick
			unit_value = i*x_axis_values.interval+x_axis_values.bottom;
			if(unit_value >= PlotData.curr_min_x_coord) {
				px_value = CanvasHandler.unitCoordToPxCoord(unit_value, PlotData.curr_min_y_coord)[0];
				CanvasHandler.drawAxisTick(unit_value, PlotData.x_axis_units, CanvasHandler.consts.x_axis, px_value, CanvasHandler.xAxisCoords().end[1], ((i % 5) === 0));
			}
		}
		for (i = 0; i < y_axis_values.quantity; i++) {
			//y axis intermediate tick
			unit_value = i*y_axis_values.interval+y_axis_values.bottom;
			if(unit_value >= PlotData.curr_min_y_coord) {
				px_value = CanvasHandler.unitCoordToPxCoord(PlotData.curr_min_x_coord, unit_value)[1];
				CanvasHandler.drawAxisTick(unit_value, PlotData.y_axis_units, CanvasHandler.consts.y_axis, CanvasHandler.yAxisCoords().end[0], px_value, ((i % 5) === 0));
			}
		}
	},
	//TODO: This can be cleaned up to only require one coordinate
	drawAxisTick:function(value, unit, axis, x_coord, y_coord, draw_text) {
		'use strict';
		var text_x, text_y, ctx, x_coord_end, y_coord_end, date, type, milliseconds, seconds, minutes, hours;
		ctx = CanvasHandler.context;
		if(axis === CanvasHandler.consts.x_axis) {
			if(x_coord > CanvasHandler.xAxisCoords().end[0]) {
				return;
			}
			text_x = x_coord;
			text_y = y_coord+10;
			x_coord_end = x_coord;
			y_coord_end = y_coord+5;
			type = PlotData.x_coord_type;
			ctx.textAlign = 'center';
			ctx.textBaseline = 'top';
		} else if(axis === CanvasHandler.consts.y_axis) {
			if(y_coord < CanvasHandler.yAxisCoords().end[1]) {
				return;
			}
			text_x = x_coord-10;
			text_y = y_coord;
			x_coord_end = x_coord-5;
			y_coord_end = y_coord;
			type = PlotData.y_coord_type;
			ctx.textAlign = 'right';
			ctx.textBaseline = 'middle';
		}
		//perform any massaging of the value to match the unit type, first convert to number then trim some number of decimal places
		//TODO: handle different unit types
		if(type === 'double') {
			value = parseFloat(value);
			value = value.toPrecision(6);
		} else if(type === 'signed' || type === 'unsigned') {
			value = parseInt(value, 10);
			value = value.toPrecision(6);
		} else if(type === 'timeval') {
			date = new Date(value*1000);
			milliseconds = date.getMilliseconds();
			milliseconds = (milliseconds < 10) ? '0'+milliseconds.toString() : milliseconds;
			seconds = date.getSeconds();
			seconds = (seconds < 10) ? '0'+seconds.toString() : seconds;
			minutes = date.getMinutes();
			minutes = (minutes < 10) ? '0'+minutes.toString() : minutes;
			hours = date.getHours();
			hours = (hours < 10) ? '0'+hours.toString() : hours;
			value = hours + ':' + minutes + ':' + seconds + ':' + milliseconds;
		} else if(type === 'dtime') {
			value = parseFloat(value);
			if(Math.abs(value) >= 1 || Math.abs(value) === 0) {
				unit = 's';
			} else if(Math.abs(value) < 1 && Math.abs(value) >= 0.001) {
				unit = 'ms';
				value = value*1000;
			} else {
				unit = String.fromCharCode(956)+'s';
				value = value*1000000;
			}
			value = value.toPrecision(6);
		}
		//check if the units actually have a value
		unit = (unit) ? unit : '';
		if(draw_text) {
			ctx.fillText(value+unit,text_x,text_y);
		}
		ctx.moveTo(x_coord,y_coord);
		ctx.lineTo(x_coord_end,y_coord_end);
		ctx.stroke();
	},
	drawTitle:function() {
		'use strict';
		if(PlotData.title) {
			var center;
			center = CanvasHandler.width() / 2;
			CanvasHandler.context.fillStyle = '#000000';
			CanvasHandler.context.strokeStyle = '#000000';
			CanvasHandler.context.beginPath();
			CanvasHandler.context.textAlign = 'center';
			CanvasHandler.context.textBaseline = 'middle';
			CanvasHandler.context.fillText(PlotData.title, center, 10);
			CanvasHandler.context.stroke();
		}
	},
	drawAxisLabels:function() {
		'use strict';
		var ctx, center;
		center = CanvasHandler.width() / 2;
		ctx = CanvasHandler.context;
		//draw x-axis label
		ctx.fillStyle = '#000000';
		ctx.strokeStyle = '#000000';
		ctx.beginPath();
		ctx.textAlign = 'right';
		ctx.textBaseline = 'middle';
		ctx.fillText((PlotData.x_axis_label) ? PlotData.x_axis_label : '', CanvasHandler.xAxisCoords().end[0], CanvasHandler.xAxisCoords().end[1]+50);
		ctx.stroke();
		//draw the y-axis label
		ctx.beginPath();
		ctx.textAlign = 'right';
		ctx.textBaseline = 'middle';
		ctx.fillText((PlotData.y_axis_label) ? PlotData.y_axis_label : '', CanvasHandler.yAxisCoords().end[0], CanvasHandler.yAxisCoords().end[1]-15);
		ctx.stroke();
	},
	changeDimensions:function() {
		'use strict';
		CanvasHandler.canvas.height = document.getElementById('plot_height').value;
		CanvasHandler.canvas.width = document.getElementById('plot_width').value;
	},
	setClipBoundary:function() {
		'use strict';
		CanvasHandler.context.beginPath();
		CanvasHandler.context.moveTo(CanvasHandler.yAxisCoords().end[0], CanvasHandler.yAxisCoords().end[1]);
		CanvasHandler.context.lineTo(CanvasHandler.yAxisCoords().start[0], CanvasHandler.yAxisCoords().start[1]);
		CanvasHandler.context.lineTo(CanvasHandler.xAxisCoords().end[0], CanvasHandler.xAxisCoords().end[1]);
		CanvasHandler.context.lineTo(CanvasHandler.xAxisCoords().end[0], CanvasHandler.yAxisCoords().end[1]);
		CanvasHandler.context.lineTo(CanvasHandler.yAxisCoords().end[0], CanvasHandler.yAxisCoords().end[1]);
		CanvasHandler.context.clip();
	},
	getPrettyTickInfo:function(min, max) {//inspired by http://cpansearch.perl.org/src/ADAMK/Chart-Math-Axis-1.06/lib/Chart/Math/Axis.pm
		'use strict';
		var range, interval, top, bottom, tickQuantity;
		range = Math.jsPlotPrettifyNumber(max - min, false);
		interval = Math.jsPlotPrettifyNumber(range / (PlotData.max_ticks - 1), true);
		bottom = Math.floor(min / interval) * interval;
		top = Math.ceil(min / interval) * interval;
		tickQuantity = range / interval;
		return {
			interval:interval,
			top:top,
			bottom:bottom,
			quantity:tickQuantity
		};
	},
	unitCoordToPxCoord:function(x_coord, y_coord) {
		'use strict';
		var y_axis_px_len, y_axis_unit_len, x_axis_px_len, x_axis_unit_len, x_px_coord, y_px_coord;
		//calculate y axis length in pixels
		y_axis_px_len = CanvasHandler.yAxisCoords().start[1] - CanvasHandler.yAxisCoords().end[1];
		y_axis_unit_len = PlotData.curr_max_y_coord - PlotData.curr_min_y_coord;
		x_axis_px_len = CanvasHandler.xAxisCoords().end[0] - CanvasHandler.xAxisCoords().start[0];
		x_axis_unit_len = PlotData.curr_max_x_coord - PlotData.curr_min_x_coord;
		x_px_coord = ((x_coord - PlotData.curr_min_x_coord) * x_axis_px_len / x_axis_unit_len)+CanvasHandler.yAxisCoords().start[0];
		y_px_coord = ((PlotData.curr_max_y_coord - y_coord) * y_axis_px_len / y_axis_unit_len)+CanvasHandler.yAxisCoords().end[1];
		return [x_px_coord, y_px_coord];
	},
	pxCoordToUnitCoord:function(x_coord, y_coord) {
		'use strict';
		var y_axis_px_len, y_axis_unit_len, x_axis_px_len, x_axis_unit_len, x_px_coord, y_px_coord, x_unit_coord, y_unit_coord;
		//calculate y axis length in pixels
		y_axis_px_len = CanvasHandler.yAxisCoords().start[1] - CanvasHandler.yAxisCoords().end[1];
		y_axis_unit_len = PlotData.curr_max_y_coord - PlotData.curr_min_y_coord;
		x_axis_px_len = CanvasHandler.xAxisCoords().end[0] - CanvasHandler.xAxisCoords().start[0];
		x_axis_unit_len = PlotData.curr_max_x_coord - PlotData.curr_min_x_coord;
		x_unit_coord = ((x_coord-CanvasHandler.yAxisCoords().start[0])*x_axis_unit_len/x_axis_px_len)+PlotData.curr_min_x_coord;
		y_unit_coord = ((CanvasHandler.yAxisCoords().start[1] - y_coord) * y_axis_unit_len / y_axis_px_len)+PlotData.curr_min_y_coord;
		return [x_unit_coord, y_unit_coord];
	},
	yAxisCoords:function() {
		'use strict';
		return {
			'start':[150, CanvasHandler.height() - 100],
			'end':[150, 20]
		};
	},
	xAxisCoords:function() {
		'use strict';
		return {
			'start':[150, CanvasHandler.height() - 100],
			'end':[CanvasHandler.width()-15, CanvasHandler.height() - 100]
		};
	}
};

var Colors = {
	'white':'000000',
	'green':'23d516',
	'red':'d92300',
	'blue':'1425e0',
	'yellow':'dedd57',
	'purple':'880a77',
	'orange':'ee6d1e',
	'magenta':'e711ca',
	'pink':'fa61b7'
};
var ColorLookup = ['white', 'green', 'red', 'blue', 'yellow', 'purple', 'orange', 'magenta', 'pink'];