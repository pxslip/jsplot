# XPlot File Specification

## Pre-Commands

- `new_plotter` - essentially a no-op, indicates the start of the plot definition
- `[unit] [unit]` - defines the units for the x and y axes
  - unsigned - unsigned integer
  - signed - signed integer
  - timeval - seconds since the epoch (12AM on Jan. 1st 1970), supports decimal
  - double - a floating point number
  - dtime - time represented as a double, value is in seconds
- `title` - the title of the plot
- `xlabel` - the label on the x axis
- `ylabel` - the label on the y axis
- `xunits` - the units of the y axis
- `yunits` - the units of the x axis
- `aspect_ratio` - *unsupported* defines the aspect ratio of the plot

## Plot Commands

After the preface defining the metadata of the graph the below commands are used to define items on the plot

- `X`
- `DOT` or `.`
- `PLUS` or `+`
- `BOX`
- `DIAMOND`
- `UTICK`
- `DTICK`
- `LTICK`
- `RTICK`
- `HTICK`
- `VTICK`
- `UARROW`
- `DARROW`
- `LARROW`
- `RARROW`
- `INVISIBLE`
- `LINE`
- `DLINE`
- `TEXT`
- `TITLE`
- `XLABEL`
- `YLABEL`