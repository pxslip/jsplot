# jsPlot is a browser based port of the venerable xPlot library

This is an attempt to build a performant web-based implementation of xPlot. The first attempt at building this tool used pure native ES and its interface to the canvas tag. v2 is an attempt to improve performance by converting to Rust/WASM and exporting the bindings for consumption as an ES module.

The original c-based implementation is perserved here for reference.

## Installation

```
npm i jsplot
```

```