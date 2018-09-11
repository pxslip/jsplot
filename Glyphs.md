# All Glyphs that must be implemented #

- Left Tick
- Right Tick
- Up Tick
- Down Tick
- Horizontal Tick
- Vertical Tick
- Diamond
- Box
- Left Arrow
- Right Arrow
- Up Arrow
- Down Arrow
- Plus
- Cross
- Line
- Demarcated Line

```js
/**
 * jsPlot (c) 2014 Will Kruse
 * jsPlot is a web-based replacement for xPlot or jPlot. In theory this will run on any server, or locally
 * NOTE: at the time of creation only some browsers support the File api
 *
 * jsPlot is released under the MIT License whose text can be found in the included LICENSE file
 */
var Glyph = {
    init: function(x_coord, y_coord, color) {
        'use strict';
        this.x_coord = parseFloat(x_coord);
        this.y_coord = parseFloat(y_coord);
        if(color && Math.isNumeric(color)) {
            this.color = '#'+Colors[ColorLookup[color]];
        } else if(color) {
            this.color = '#'+Colors[color];
        }
    },
    setPxCoords:function() {
        'use strict';
        var px_coord;
        if(this.x_coord&&this.y_coord) {
            px_coord = CanvasHandler.unitCoordToPxCoord(this.x_coord, this.y_coord);
            this.x_px_coord = px_coord[0];
            this.y_px_coord = px_coord[1];
        }
    },
    draw:function(ctx) {
        'use strict';
        Glyph.setPxCoords.call(this);
        if(this.color) {
            ctx.fillStyle = this.color;
            ctx.strokeStyle = this.color;
        } else {
            ctx.fillStyle = PlotData.curr_color;
            ctx.strokeStyle = PlotData.curr_color;
        }
    }
};

var Color =  function(color) {
    'use strict';
    this.color = color;
};
Color.prototype.draw = function(ctx) {
    'use strict';
    // ctx.fillStyle = '#'+Colors[this.color];
    // ctx.strokeStyle = '#'+Colors[this.color];
    PlotData.curr_color = '#'+Colors[this.color];
};

var LeftTick = function(x_coord, y_coord, color) {
    'use strict';
    Glyph.init.call(this, x_coord, y_coord, color);
};
LeftTick.prototype.draw = function(ctx) {
    'use strict';
    Glyph.draw.call(this, ctx);
    ctx.beginPath();
    ctx.moveTo(this.x_px_coord, this.y_px_coord);
    ctx.lineTo(this.x_px_coord-5, this.y_px_coord);
    ctx.stroke();
};

var RightTick = function(x_coord, y_coord, color) {
    'use strict';
    Glyph.init.call(this, x_coord, y_coord, color);
};
RightTick.prototype.draw = function(ctx) {
    'use strict';
    Glyph.draw.call(this, ctx);
    ctx.beginPath();
    ctx.moveTo(this.x_px_coord, this.y_px_coord);
    ctx.lineTo(this.x_px_coord+5, this.y_px_coord);
    ctx.stroke();
};

var UpTick = function(x_coord, y_coord, color) {
    'use strict';
    Glyph.init.call(this, x_coord, y_coord, color);
};
UpTick.prototype.draw = function(ctx) {
    'use strict';
    Glyph.draw.call(this, ctx);
    ctx.beginPath();
    ctx.moveTo(this.x_px_coord, this.y_px_coord);
    ctx.lineTo(this.x_px_coord, this.y_px_coord-5);
    ctx.stroke();
};

var DownTick = function(x_coord, y_coord, color) {
    'use strict';
    Glyph.init.call(this, x_coord, y_coord, color);
};
DownTick.prototype.draw = function(ctx) {
    'use strict';
    Glyph.draw.call(this, ctx);
    ctx.beginPath();
    ctx.moveTo(this.x_px_coord, this.y_px_coord);
    ctx.lineTo(this.x_px_coord, this.y_px_coord+5);
    ctx.stroke();
};

var HorizTick = function(x_coord, y_coord, color) {
    'use strict';
    Glyph.init.call(this, x_coord, y_coord, color);
};
HorizTick.prototype.draw = function(ctx) {
    'use strict';
    Glyph.draw.call(this, ctx);
    ctx.beginPath();
    ctx.moveTo(this.x_px_coord-2.5, this.y_px_coord);
    ctx.lineTo(this.x_px_coord+2.5, this.y_px_coord);
    ctx.stroke();
};

var VertTick = function(x_coord, y_coord, color) {
    'use strict';
    Glyph.init.call(this, x_coord, y_coord, color);
};
VertTick.prototype.draw = function(ctx) {
    'use strict';
    Glyph.draw.call(this, ctx);
    ctx.beginPath();
    ctx.moveTo(this.x_px_coord, this.y_px_coord-2.5);
    ctx.lineTo(this.x_px_coord, this.y_px_coord+2.5);
    ctx.stroke();
};

var Diamond = function(x_coord, y_coord, color) {
    'use strict';
    Glyph.init.call(this, x_coord, y_coord, color);
};
Diamond.prototype.draw = function(ctx) {
    'use strict';
    Glyph.draw.call(this, ctx);
    ctx.beginPath();
    ctx.moveTo(this.x_px_coord, this.y_px_coord-5);
    ctx.lineTo(this.x_px_coord+5, this.y_px_coord);
    ctx.lineTo(this.x_px_coord, this.y_px_coord+5);
    ctx.lineTo(this.x_px_coord-5, this.y_px_coord);
    ctx.lineTo(this.x_px_coord, this.y_px_coord-5);
    ctx.stroke();
};

var Box = function(x_coord, y_coord, color) {
    'use strict';
    Glyph.init.call(this, x_coord, y_coord, color);
};
Box.prototype.draw = function(ctx) {
    'use strict';
    Glyph.draw.call(this, ctx);
    ctx.beginPath();
    ctx.moveTo(this.x_px_coord-5, this.y_px_coord-5);
    ctx.lineTo(this.x_px_coord+5, this.y_px_coord-5);
    ctx.lineTo(this.x_px_coord+5, this.y_px_coord+5);
    ctx.lineTo(this.x_px_coord-5, this.y_px_coord+5);
    ctx.lineTo(this.x_px_coord-5, this.y_px_coord-5);
    ctx.stroke();
};

var LeftArrow = function(x_coord, y_coord, color) {
    'use strict';
    Glyph.init.call(this, x_coord, y_coord, color);
};
LeftArrow.prototype.draw = function(ctx) {
    'use strict';
    Glyph.draw.call(this, ctx);
    ctx.beginPath();
    ctx.moveTo(this.x_px_coord, this.y_px_coord);
    ctx.lineTo(this.x_px_coord+5, this.y_px_coord-5);
    ctx.moveTo(this.x_px_coord, this.y_px_coord);
    ctx.lineTo(this.x_px_coord+5, this.y_px_coord+5);
    ctx.stroke();
};

var RightArrow = function(x_coord, y_coord, color) {
    'use strict';
    Glyph.init.call(this, x_coord, y_coord, color);
};
RightArrow.prototype.draw = function(ctx) {
    'use strict';
    Glyph.draw.call(this, ctx);
    ctx.beginPath();
    ctx.moveTo(this.x_px_coord, this.y_px_coord);
    ctx.lineTo(this.x_px_coord-5, this.y_px_coord-5);
    ctx.moveTo(this.x_px_coord, this.y_px_coord);
    ctx.lineTo(this.x_px_coord-5, this.y_px_coord+5);
    ctx.stroke();
};

var UpArrow = function(x_coord, y_coord, color) {
    'use strict';
    Glyph.init.call(this, x_coord, y_coord, color);
};
UpArrow.prototype.draw = function(ctx) {
    'use strict';
    Glyph.draw.call(this, ctx);
    ctx.beginPath();
    ctx.moveTo(this.x_px_coord, this.y_px_coord);
    ctx.lineTo(this.x_px_coord-5, this.y_px_coord+5);
    ctx.moveTo(this.x_px_coord, this.y_px_coord);
    ctx.lineTo(this.x_px_coord+5, this.y_px_coord+5);
    ctx.stroke();
};

var DownArrow = function(x_coord, y_coord, color) {
    'use strict';
    Glyph.init.call(this, x_coord, y_coord, color);
};
DownArrow.prototype.draw = function(ctx) {
    'use strict';
    Glyph.draw.call(this, ctx);
    ctx.beginPath();
    ctx.moveTo(this.x_px_coord, this.y_px_coord);
    ctx.lineTo(this.x_px_coord+5, this.y_px_coord-5);
    ctx.moveTo(this.x_px_coord, this.y_px_coord);
    ctx.lineTo(this.x_px_coord-5, this.y_px_coord-5);
    ctx.stroke();
};

var Dot = function(x_coord, y_coord, color) {
    'use strict';
    Glyph.init.call(this, x_coord, y_coord, color);
};
Dot.prototype.draw = function(ctx) {
    'use strict';
    Glyph.draw.call(this, ctx);
    ctx.beginPath();
    ctx.arc(this.x_px_coord, this.y_px_coord, 1, 0, 2*Math.PI, false);
    ctx.fill();
    ctx.stroke();
};

var Invisible = function(x_coord, y_coord, color) {
    'use strict';
    Glyph.init.call(this, x_coord, y_coord, color);
};
Invisible.prototype.draw = function(ctx) {
    'use strict';
    //no-op for invis cmd
};

var Plus = function(x_coord, y_coord, color) {
    'use strict';
    Glyph.init.call(this, x_coord, y_coord, color);
};
Plus.prototype.draw = function(ctx) {
    'use strict';
    Glyph.draw.call(this, ctx);
    ctx.beginPath();
    ctx.moveTo(this.x_px_coord, this.y_px_coord);
    ctx.lineTo(this.x_px_coord+5, this.y_px_coord);
    ctx.moveTo(this.x_px_coord, this.y_px_coord);
    ctx.lineTo(this.x_px_coord-5, this.y_px_coord);
    ctx.moveTo(this.x_px_coord, this.y_px_coord);
    ctx.lineTo(this.x_px_coord, this.y_px_coord+5);
    ctx.moveTo(this.x_px_coord, this.y_px_coord);
    ctx.lineTo(this.x_px_coord, this.y_px_coord-5);
    ctx.stroke();
};

//st andrew's cross...
var Cross = function(x_coord, y_coord, color) {
    'use strict';
    Glyph.init.call(this, x_coord, y_coord, color);
};
Cross.prototype.draw = function(ctx) {
    'use strict';
    Glyph.draw.call(this, ctx);
    ctx.beginPath();
    ctx.moveTo(this.x_px_coord, this.y_px_coord);
    ctx.lineTo(this.x_px_coord+4, this.y_px_coord+4);
    ctx.moveTo(this.x_px_coord, this.y_px_coord);
    ctx.lineTo(this.x_px_coord-4, this.y_px_coord+4);
    ctx.moveTo(this.x_px_coord, this.y_px_coord);
    ctx.lineTo(this.x_px_coord-4, this.y_px_coord-4);
    ctx.moveTo(this.x_px_coord, this.y_px_coord);
    ctx.lineTo(this.x_px_coord+4, this.y_px_coord-4);
    ctx.stroke();
};

//TODO: create BaseLine class to handle common stuff
var Line = function(x_coord_start, y_coord_start, x_coord_end, y_coord_end, color) {
    'use strict';
    if((x_coord_start > x_coord_end) || (x_coord_end === x_coord_start && y_coord_start < y_coord_end)) {
        this.x_coord_start = parseFloat(x_coord_end);
        this.y_coord_start = parseFloat(y_coord_end);
        this.x_coord_end = parseFloat(x_coord_start);
        this.y_coord_end = parseFloat(y_coord_start);
    } else {
        this.x_coord_start = parseFloat(x_coord_start);
        this.y_coord_start = parseFloat(y_coord_start);
        this.x_coord_end = parseFloat(x_coord_end);
        this.y_coord_end = parseFloat(y_coord_end);
    }
    if(color && Math.isNumeric(color)) {
        this.color = '#'+Colors[ColorLookup[color]];
    } else if(color) {
        this.color = '#'+Colors[color];
    }
};
Line.prototype.draw = function(ctx) {
    'use strict';
    var px_coord_start, px_coord_end;
    px_coord_start = CanvasHandler.unitCoordToPxCoord(this.x_coord_start, this.y_coord_start);
    px_coord_end = CanvasHandler.unitCoordToPxCoord(this.x_coord_end, this.y_coord_end);
    if(this.color) {
        ctx.fillStyle = this.color;
        ctx.strokeStyle = this.color;
    } else {
        ctx.fillStyle = PlotData.curr_color;
        ctx.strokeStyle = PlotData.curr_color;
    }
    ctx.beginPath();
    ctx.moveTo(px_coord_start[0], px_coord_start[1]);
    ctx.lineTo(px_coord_end[0], px_coord_end[1]);
    ctx.stroke();
};
//TODO: This should be a demarcated line, not dotted
var DemarcatedLine = function(x_coord_start, y_coord_start, x_coord_end, y_coord_end, color) {
    'use strict';
    if(x_coord_start > x_coord_end) {
        this.x_coord_start = parseFloat(x_coord_end);
        this.y_coord_start = parseFloat(y_coord_end);
        this.x_coord_end = parseFloat(x_coord_start);
        this.y_coord_end = parseFloat(y_coord_start);
    } else {
        this.x_coord_start = parseFloat(x_coord_start);
        this.y_coord_start = parseFloat(y_coord_start);
        this.x_coord_end = parseFloat(x_coord_end);
        this.y_coord_end = parseFloat(y_coord_end);
    }
    if(color && Math.isNumeric(color)) {
        this.color = '#'+Colors[ColorLookup[color]];
    } else if(color) {
        this.color = '#'+Colors[color];
    }
};
DemarcatedLine.prototype.draw = function(ctx) {
    'use strict';
    var px_coord_start, px_coord_end;
    px_coord_start = CanvasHandler.unitCoordToPxCoord(this.x_coord_start, this.y_coord_start);
    px_coord_end = CanvasHandler.unitCoordToPxCoord(this.x_coord_end, this.y_coord_end);
    if(this.color) {
        ctx.fillStyle = this.color;
        ctx.strokeStyle = this.color;
    } else {
        ctx.fillStyle = PlotData.curr_color;
        ctx.strokeStyle = PlotData.curr_color;
    }
    ctx.beginPath();
    ctx.moveTo(px_coord_start[0], px_coord_start[1]);
    ctx.lineTo(px_coord_start[0]+2, px_coord_start[1]+2);
    ctx.moveTo(px_coord_start[0], px_coord_start[1]);
    ctx.lineTo(px_coord_start[0]-2, px_coord_start[1]+2);
    ctx.moveTo(px_coord_start[0], px_coord_start[1]);
    ctx.lineTo(px_coord_start[0]-2, px_coord_start[1]-2);
    ctx.moveTo(px_coord_start[0], px_coord_start[1]);
    ctx.lineTo(px_coord_start[0]+2, px_coord_start[1]-2);
    ctx.lineTo(px_coord_end[0], px_coord_end[1]);
    ctx.lineTo(px_coord_end[0]+2, px_coord_end[1]+2);
    ctx.moveTo(px_coord_end[0], px_coord_end[1]);
    ctx.lineTo(px_coord_end[0]-2, px_coord_end[1]+2);
    ctx.moveTo(px_coord_end[0], px_coord_end[1]);
    ctx.lineTo(px_coord_end[0]-2, px_coord_end[1]-2);
    ctx.moveTo(px_coord_end[0], px_coord_end[1]);
    ctx.lineTo(px_coord_end[0]+2, px_coord_end[1]-2);
    ctx.stroke();
};
```
