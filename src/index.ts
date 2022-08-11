import { JSPlot } from './lib';

const wrapper = document.getElementById('plot_wrapper') as HTMLCanvasElement;
const filePicker = document.getElementById('file_chooser') as HTMLInputElement;

const plotter = new JSPlot(filePicker, wrapper);
