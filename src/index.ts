import { JSPlot } from "./lib";

const canvas = document.getElementById("plot_canvas") as HTMLCanvasElement;
const filePicker = document.getElementById("file_chooser") as HTMLInputElement;

const plotter = new JSPlot(filePicker, canvas);
