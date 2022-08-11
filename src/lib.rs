mod commands;
mod units;

use units::Unit;

use js_sys::Array;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

use crate::commands::Command;
use crate::units::Point;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub struct Plotter {
    canvas: HtmlCanvasElement,
    render_context: CanvasRenderingContext2d,
    commands: Vec<Command>,
    title: Option<String>,
    x_label: Option<String>,
    x_unit: Option<String>,
    x_type: String,
    y_label: Option<String>,
    y_unit: Option<String>,
    y_type: String,
    max_point: Point,
    min_point: Point,
}

#[wasm_bindgen]
impl Plotter {
    #[wasm_bindgen(constructor)]
    pub fn new(
        canvas: HtmlCanvasElement,
        x_type: String,
        y_type: String,
    ) -> Result<Plotter, JsValue> {
        let render_context = canvas
            .get_context("2d")
            .expect("Therre was an error getting the canvas render context")
            .expect("No render context was returned by `get_context`")
            .dyn_into::<CanvasRenderingContext2d>()
            .expect("There was an error casting the render context");
        render_context
            .translate(0.0, canvas.height().into())
            .expect("Error translating the canvas context into a cartesian system");
        render_context
            .scale(1.0, -1.0)
            .expect("Error translating the canvas context into a cartesian system");

        Ok(Plotter {
            canvas,
            render_context,
            commands: Vec::new(),
            title: None,
            x_label: None,
            x_unit: None,
            x_type,
            y_label: None,
            y_unit: None,
            y_type,
            max_point: Point::new_with_defaults(),
            min_point: Point::new_with_defaults(),
        })
    }

    pub fn title(&self) -> String {
        return self
            .title
            .as_ref()
            .expect("Title of the plot is not yet set")
            .to_string();
    }

    pub fn render(&self) {
        self.canvas
            .class_list()
            .add(&Array::of1(&JsValue::from_str("rendering")))
            .expect("Error adding a class to the canvas");

        for command in self.commands.iter() {}
    }

    pub fn is_multiline_command(&self, command: String) -> bool {
        Command::is_multiline(command.as_str())
    }

    pub fn parse_command(&mut self, command_string: String, js_args: js_sys::Array) {
        let mut args = Vec::new();
        for arg in js_args.iter() {
            args.push(
                arg.as_string()
                    .expect("There was an error parsing a command argument as a rust string"),
            )
        }
        let command = Command::new(command_string, args, &self.x_type, &self.y_type);
        // update the max and min points of the graph
        self.min_point = command.min_point(&self.min_point);
        self.max_point = command.max_point(&self.max_point);
        self.commands.push(command);
    }
}

impl Plotter {
    pub fn parse_x_unit(&self, value: String) -> Unit {
        Unit::from_value(&self.x_type, value)
    }

    pub fn parse_y_unit(&self, value: String) -> Unit {
        Unit::from_value(&self.y_type, value)
    }

    /**
     * Convert a
     */
    pub fn transform_point(&self, point: Point) -> Point {
        let Point { x, y } = point;
        let min_x = &self.min_point.x;
        let min_y = &self.min_point.y;
        let max_x = &self.max_point.x;
        let max_y = &self.max_point.y;
        let range_x = min_x.abs() + max_x.abs();
        let range_y = min_y.abs() + max_y.abs();
        let canvas_y = self.canvas.height();
        let canvas_x = self.canvas.width();
        let scale_x = canvas_x / range_x;
        let scale_y = canvas_y / range_y;
        Point {
            x: x.mul_by_f32(scale_x),
            y: y.mul_by_f32(scale_y),
        }
    }
}
