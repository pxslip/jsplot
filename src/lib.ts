import { Plotter } from '../pkg';

const DISABLED_COMMANDS = ['new_plotter', 'aspect_ratio'];

export class JSPlot {
	private fileInput;
	private plotter: Plotter;
	private canvas;
	constructor(fileInput: HTMLInputElement, canvas: HTMLCanvasElement) {
		this.canvas = canvas;
		this.fileInput = fileInput;
		this.fileInput.addEventListener('input', () => {
			this.#fileSelectedHandler();
		});
	}

	get title() {
		return this.plotter?.title();
	}

	async #fileSelectedHandler() {
		const file = this.fileInput.files.item(0);
		let lastLine = '';
		let first = true; // need to yoink the units off the first actual line
		for await (let line of this.#parseFileToLines(file)) {
			// handle the first line being `new_plotter` or the unit types
			if (line.includes('new_plotter')) {
				continue;
			}
			if (first) {
				const units = line.split(/\s+/);
				const xUnit = units.shift();
				const yUnit = units.shift();
				this.plotter = new Plotter(this.canvas, xUnit, yUnit);
			}
			// confirm we haven't somehow bypassed getting units and creating the plotter
			if (this.plotter) {
				if (this.plotter.is_multiline_command(line.split(/\s+/).shift())) {
					lastLine = line;
					continue;
				}
				if (lastLine !== '') {
					const args = lastLine.split(/\s+/);
					const command = args.shift();
					args.push(line);
					this.plotter.parse_command(command, args);
					lastLine = '';
				} else {
					const args = line.split(/\s+/);
					const command = args.shift();
					this.plotter.parse_command(command, args);
				}
			} else {
				throw new Error(
					'The WASM Plotter class was not properly instantiated, confirm your plot file is correctly formatted namely that the first line contains the unit types'
				);
			}
		}
	}

	async *#parseFileToLines(file: File) {
		const reader = file.stream().pipeThrough(new TextDecoderStream()).getReader();
		let re = /\r\n|\n|\r/gm;
		let startIndex = 0;
		let { value: chunk, done: readerDone } = await reader.read();
		chunk = chunk ?? '';
		for (;;) {
			let result = re.exec(chunk);
			if (!result) {
				if (readerDone) {
					break;
				}
				let remainder = chunk.substring(startIndex);
				({ value: chunk, done: readerDone } = await reader.read());
				chunk = remainder + (chunk ?? '');
				startIndex = re.lastIndex = 0;
				continue;
			}
			yield chunk.substring(startIndex, result.index);
			startIndex = re.lastIndex;
		}
		if (startIndex < chunk.length) {
			// last line didn't end in a newline char
			yield chunk.substring(startIndex);
		}
	}
}
