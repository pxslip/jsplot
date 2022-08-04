mod commands;
mod units;

use std::boxed::Box as StdBox;
use std::str::FromStr;

use commands::kind::CommandKind;
use commands::Command;
use units::Unit;

use js_sys::Array;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub struct Plotter {
    canvas: HtmlCanvasElement,
    render_context: CanvasRenderingContext2d,
    commands: Vec<StdBox<dyn Command>>,
    title: Option<String>,
    x_label: Option<String>,
    x_unit: Option<String>,
    x_type: String,
    y_label: Option<String>,
    y_unit: Option<String>,
    y_type: String,
    max_y: Unit,
    min_y: Unit,
    max_x: Unit,
    min_x: Unit,
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
            max_x: Unit::Unsigned(0),
            min_x: Unit::Unsigned(0),
            max_y: Unit::Unsigned(0),
            min_y: Unit::Unsigned(0),
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
        let kind = CommandKind::from_str(command.as_str())
            .expect("Error parsing the command to a CommandKind");
        kind.is_multiline()
    }

    pub fn parse_command(&mut self, command_string: String, js_args: js_sys::Array) {
        let mut args = Vec::new();
        for arg in js_args.iter() {
            args.push(
                arg.as_string()
                    .expect("There was an error parsing a command argument as a rust string"),
            )
        }
        let kind = CommandKind::from_str(command_string.as_str())
            .expect("There was an error parsing the command string to a CommandKind");
        match kind {
            CommandKind::Title => {
                self.title = Some(
                    args.get(0)
                        .expect("Error getting the title from the title command")
                        .to_owned(),
                )
            }
            CommandKind::XLabel => {
                self.x_label = Some(
                    args.get(0)
                        .expect("Error getting the title from the title command")
                        .to_owned(),
                )
            }
            CommandKind::XUnits => {
                self.x_unit = Some(
                    args.get(0)
                        .expect("Error getting the title from the title command")
                        .to_owned(),
                )
            }
            CommandKind::YLabel => {
                self.y_label = Some(
                    args.get(0)
                        .expect("Error getting the title from the title command")
                        .to_owned(),
                )
            }
            CommandKind::YUnits => {
                self.y_unit = Some(
                    args.get(0)
                        .expect("Error getting the title from the title command")
                        .to_owned(),
                )
            }
            _ => {
                let cmd = CommandKind::new_command(self, kind, args);
                self.commands.push(cmd)
            }
        }
    }
}

impl Plotter {
    pub fn parse_x_unit(&self, value: String) -> Unit {
        Unit::from_value(&self.x_type, value)
    }

    pub fn parse_y_unit(&self, value: String) -> Unit {
        Unit::from_value(&self.y_type, value)
    }
}
