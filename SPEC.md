# XPlot File Specification

## Pre-Commands

- `new_plotter` - essentially a no-op, indicates the start of the plot definition
- `[unit] [unit]` - defines the unit type for the x and y axes
  - unsigned - unsigned integer
    - using a `u64` internally
  - signed - signed integer
    - uses `i64` internally
  - timeval - seconds since the epoch (12AM on Jan. 1st 1970)
    - internal type is a `double`
  - double - a floating point number
    - using a `f64` internally
  - dtime - time represented as a double
    - internal type is a `double`
- `title` - the title of the plot
  - consumes the following line for the text
- `xlabel` - the label on the x axis\
  - consumes the following line for the text
- `ylabel` - the label on the y axis
  - consumes the following line for the text
- `xunits` - the unit label of the y axis
  - consumes the following line for the text
- `yunits` - the unit label of the x axis
  - consumes the following line for the text
- `aspect_ratio` - _unsupported_ defines the aspect ratio of the plot
  - TODO: This can be handled by allowing an additional config option that specifies which axis (width or height) of the canvas element to preserve, then resizing the other axis based on this value

## Plot Commands

After the preface defining the metadata of the graph the below commands are used to define items on the plot

- `X`
  - A multiplication, times, or `x` symbol centered on the point indicated
- `DOT` or `.`
  - A dot at the point indicated
- `PLUS` or `+`
  - The plus or additioon symbol centered on the point indicated
- `BOX`
  - A box drawn around the point indicated
- `DIAMOND`
  - A diamond drawn around the point indicated
- `UTICK`
  - A short line pointing up from the point indicated
- `DTICK`
  - A short line pointing down from the point indicated
- `LTICK`
  - A short line pointing left from the point indicated
- `RTICK`
  - A short line pointing right from the point indicated
- `HTICK`
  - A short line pointing left and right centered on the point indicated
- `VTICK`
  - A short line pointing up and down centered on the point indicated
- `UARROW`
  - An arrow pointing up, starting from the point indicated
- `DARROW`
  - An arrow pointing down, starting from the point indicated
- `LARROW`
  - An arrow pointing left, starting from the point indicated
- `RARROW`
  - An arrow pointing right, starting from the point indicated
- `INVISIBLE`
  - Nothing is drawn, used to set initial bounds of the plot
- `LINE`
  - A line drawn between the two points indicated
- `DLINE`
  - A dotted/demarcated line drawn between the two points indicated
- `ATEXT`
  - Text positioned above the point indicated
  - consumes the following line
- `BTEXT`
  - Text positioned below the point indicated
  - consumes the following line
- `CTEXT`
  - Text centered on the point indicated
  - consumes the following line
- `LTEXT`
  - Text to the left of the point indicated
  - consumes the following line
- `RTEXT`
  - Text to the right of the point indicated
  - consumes the following line
