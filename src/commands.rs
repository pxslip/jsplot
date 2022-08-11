use std::convert::TryInto;

use strum_macros::EnumString;

use web_sys::CanvasRenderingContext2d;

use crate::{units::Point, Plotter};

#[derive(EnumString, Clone)]
pub enum Command {
    NewPlotter,
    AspectRatio,
    Title(String),
    XLabel(String),
    YLabel(String),
    XUnits(String),
    YUnits(String),
    X(Point),
    #[strum(serialize = ".", serialize = "dot")]
    Dot(Point),
    #[strum(serialize = "+", serialize = "plus")]
    Plus(Point),
    Box(Point),
    Diamond(Point),
    UTick(Point),
    DTick(Point),
    LTick(Point),
    RTick(Point),
    HTick(Point),
    VTick(Point),
    UArrow(Point),
    DArrow(Point),
    LArrow(Point),
    RArrow(Point),
    Invisible(Point),
    Line(Point, Point),
    DLine(Point, Point),
    AText(Point, String),
    BText(Point, String),
    CText(Point, String),
    LText(Point, String),
    RText(Point, String),
}

impl Command {
    pub fn new(
        command_string: String,
        command_args: Vec<String>,
        x_type: &str,
        y_type: &str,
    ) -> Command {
        match command_string.as_str() {
            "new_plotter" => Command::NewPlotter,
            "title" => Command::Title(command_args.get(0).unwrap().to_string()),
            "xlabel" => Command::XLabel(command_args.get(0).unwrap().to_string()),
            "ylabel" => Command::YLabel(command_args.get(0).unwrap().to_string()),
            "xunits" => Command::XUnits(command_args.get(0).unwrap().to_string()),
            "yunits" => Command::YUnits(command_args.get(0).unwrap().to_string()),
            "aspect_ratio" => Command::AspectRatio,
            "x" => Command::X(Point::new_from_strings(
                x_type,
                y_type,
                command_args.get(0).unwrap().to_string(),
                command_args.get(1).unwrap().to_string(),
            )),
            "dot" | "." => Command::Dot(Point::new_from_strings(
                x_type,
                y_type,
                command_args.get(0).unwrap().to_string(),
                command_args.get(1).unwrap().to_string(),
            )),
            "plus" | "+" => Command::Plus(Point::new_from_strings(
                x_type,
                y_type,
                command_args.get(0).unwrap().to_string(),
                command_args.get(1).unwrap().to_string(),
            )),
            "box" => Command::Box(Point::new_from_strings(
                x_type,
                y_type,
                command_args.get(0).unwrap().to_string(),
                command_args.get(1).unwrap().to_string(),
            )),
            "diamond" => Command::Diamond(Point::new_from_strings(
                x_type,
                y_type,
                command_args.get(0).unwrap().to_string(),
                command_args.get(1).unwrap().to_string(),
            )),
            "utick" => Command::UTick(Point::new_from_strings(
                x_type,
                y_type,
                command_args.get(0).unwrap().to_string(),
                command_args.get(1).unwrap().to_string(),
            )),
            "dtick" => Command::DTick(Point::new_from_strings(
                x_type,
                y_type,
                command_args.get(0).unwrap().to_string(),
                command_args.get(1).unwrap().to_string(),
            )),
            "ltick" => Command::LTick(Point::new_from_strings(
                x_type,
                y_type,
                command_args.get(0).unwrap().to_string(),
                command_args.get(1).unwrap().to_string(),
            )),
            "rtick" => Command::RTick(Point::new_from_strings(
                x_type,
                y_type,
                command_args.get(0).unwrap().to_string(),
                command_args.get(1).unwrap().to_string(),
            )),
            "htick" => Command::HTick(Point::new_from_strings(
                x_type,
                y_type,
                command_args.get(0).unwrap().to_string(),
                command_args.get(1).unwrap().to_string(),
            )),
            "vtick" => Command::VTick(Point::new_from_strings(
                x_type,
                y_type,
                command_args.get(0).unwrap().to_string(),
                command_args.get(1).unwrap().to_string(),
            )),
            "uarrow" => Command::UArrow(Point::new_from_strings(
                x_type,
                y_type,
                command_args.get(0).unwrap().to_string(),
                command_args.get(1).unwrap().to_string(),
            )),
            "darrow" => Command::DArrow(Point::new_from_strings(
                x_type,
                y_type,
                command_args.get(0).unwrap().to_string(),
                command_args.get(1).unwrap().to_string(),
            )),
            "larrow" => Command::LArrow(Point::new_from_strings(
                x_type,
                y_type,
                command_args.get(0).unwrap().to_string(),
                command_args.get(1).unwrap().to_string(),
            )),
            "rarrow" => Command::RArrow(Point::new_from_strings(
                x_type,
                y_type,
                command_args.get(0).unwrap().to_string(),
                command_args.get(1).unwrap().to_string(),
            )),
            "invisible" => Command::Invisible(Point::new_from_strings(
                x_type,
                y_type,
                command_args.get(0).unwrap().to_string(),
                command_args.get(1).unwrap().to_string(),
            )),
            "line" => Command::Line(
                Point::new_from_strings(
                    x_type,
                    y_type,
                    command_args.get(0).unwrap().to_string(),
                    command_args.get(1).unwrap().to_string(),
                ),
                Point::new_from_strings(
                    x_type,
                    y_type,
                    command_args.get(2).unwrap().to_string(),
                    command_args.get(3).unwrap().to_string(),
                ),
            ),
            "dline" => Command::DLine(
                Point::new_from_strings(
                    x_type,
                    y_type,
                    command_args.get(0).unwrap().to_string(),
                    command_args.get(1).unwrap().to_string(),
                ),
                Point::new_from_strings(
                    x_type,
                    y_type,
                    command_args.get(2).unwrap().to_string(),
                    command_args.get(3).unwrap().to_string(),
                ),
            ),
            "atext" => Command::AText(
                Point::new_from_strings(
                    x_type,
                    y_type,
                    command_args.get(0).unwrap().to_string(),
                    command_args.get(1).unwrap().to_string(),
                ),
                command_args.get(2).unwrap().to_string(),
            ),
            "btext" => Command::BText(
                Point::new_from_strings(
                    x_type,
                    y_type,
                    command_args.get(0).unwrap().to_string(),
                    command_args.get(1).unwrap().to_string(),
                ),
                command_args.get(2).unwrap().to_string(),
            ),
            "ctext" => Command::CText(
                Point::new_from_strings(
                    x_type,
                    y_type,
                    command_args.get(0).unwrap().to_string(),
                    command_args.get(1).unwrap().to_string(),
                ),
                command_args.get(2).unwrap().to_string(),
            ),
            "ltext" => Command::LText(
                Point::new_from_strings(
                    x_type,
                    y_type,
                    command_args.get(0).unwrap().to_string(),
                    command_args.get(1).unwrap().to_string(),
                ),
                command_args.get(2).unwrap().to_string(),
            ),
            "rtext" => Command::RText(
                Point::new_from_strings(
                    x_type,
                    y_type,
                    command_args.get(0).unwrap().to_string(),
                    command_args.get(1).unwrap().to_string(),
                ),
                command_args.get(2).unwrap().to_string(),
            ),
            _ => unreachable!(),
        }
    }

    pub fn is_multiline(command: &str) -> bool {
        matches!(
            command,
            "title"
                | "xlabel"
                | "xunits"
                | "ylabel"
                | "yunits"
                | "atext"
                | "btext"
                | "ctext"
                | "ltext"
                | "rtext"
        )
    }

    pub fn draw(&self, plotter: &Plotter) {
        let context = &plotter.render_context;
        match self {
            Command::NewPlotter => unreachable!(),
            Command::AspectRatio => unreachable!(),
            Command::Title(text) => {}
            Command::XLabel(text) => {}
            Command::YLabel(text) => {}
            Command::XUnits(text) => {}
            Command::YUnits(text) => {}
            Command::X(point) => self.draw_x(plotter),
            Command::Dot(point) => {}
            Command::Plus(point) => {}
            Command::Box(point) => {}
            Command::Diamond(point) => {}
            Command::UTick(point) => {}
            Command::DTick(point) => {}
            Command::LTick(point) => {}
            Command::RTick(point) => {}
            Command::HTick(point) => {}
            Command::VTick(point) => {}
            Command::UArrow(point) => {}
            Command::DArrow(point) => {}
            Command::LArrow(point) => {}
            Command::RArrow(point) => {}
            Command::Invisible(point) => {}
            Command::Line(start, end) => {}
            Command::DLine(start, end) => {}
            Command::AText(point, text) => {}
            Command::BText(point, text) => {}
            Command::CText(point, text) => {}
            Command::LText(point, text) => {}
            Command::RText(point, text) => {}
            _ => unreachable!(),
        }
    }

    fn draw_title(&self, plotter: &Plotter) {}

    fn draw_xlabel(&self, plotter: &Plotter) {}

    fn draw_ylabel(&self, plotter: &Plotter) {}

    fn draw_xunits(&self, plotter: &Plotter) {}

    fn draw_yunits(&self, plotter: &Plotter) {}

    fn draw_x(&self, plotter: &Plotter) {}

    fn draw_dot(&self, plotter: &Plotter) {}

    fn draw_plus(&self, plotter: &Plotter) {}

    fn draw_box(&self, plotter: &Plotter) {}

    fn draw_diamond(&self, plotter: &Plotter) {}

    fn draw_utick(&self, plotter: &Plotter) {
        let context = &plotter.render_context;
        if let Command::UTick(point) = self {
            let Point { x, y } = plotter.transform_point(point.clone());
            let x_pos: f64 = x.into();
            let y_pos: f64 = y.into();
            context.begin_path();
            context.move_to(x_pos, y_pos);
            context.line_to(x_pos, y_pos + 5.0)
        }
    }

    fn draw_dtick(&self, plotter: &Plotter) {}

    fn draw_ltick(&self, plotter: &Plotter) {}

    fn draw_rtick(&self, plotter: &Plotter) {}

    fn draw_htick(&self, plotter: &Plotter) {}

    fn draw_vtick(&self, plotter: &Plotter) {}

    fn draw_uarrow(&self, plotter: &Plotter) {}

    fn draw_darrow(&self, plotter: &Plotter) {}

    fn draw_larrow(&self, plotter: &Plotter) {}

    fn draw_rarrow(&self, plotter: &Plotter) {}

    fn draw_invisible(&self, plotter: &Plotter) {}

    fn draw_line(&self, plotter: &Plotter) {}

    fn draw_dline(&self, plotter: &Plotter) {}

    fn draw_atext(&self, plotter: &Plotter) {}

    fn draw_btext(&self, plotter: &Plotter) {}

    fn draw_ctext(&self, plotter: &Plotter) {}

    fn draw_ltext(&self, plotter: &Plotter) {}

    fn draw_rtext(&self, plotter: &Plotter) {}

    pub fn max_point(&self, other: &Point) -> Point {
        match self {
            Command::Title(_)
            | Command::XLabel(_)
            | Command::YLabel(_)
            | Command::XUnits(_)
            | Command::YUnits(_)
            | Command::AspectRatio => other.clone(),
            Command::X(point)
            | Command::Dot(point)
            | Command::Plus(point)
            | Command::Box(point)
            | Command::Diamond(point)
            | Command::UTick(point)
            | Command::DTick(point)
            | Command::LTick(point)
            | Command::RTick(point)
            | Command::HTick(point)
            | Command::VTick(point)
            | Command::UArrow(point)
            | Command::DArrow(point)
            | Command::LArrow(point)
            | Command::RArrow(point)
            | Command::Invisible(point)
            | Command::AText(point, _)
            | Command::BText(point, _)
            | Command::CText(point, _)
            | Command::LText(point, _)
            | Command::RText(point, _) => point.max_point(other),
            Command::Line(start, end) | Command::DLine(start, end) => {
                let first = start.max_point(other);
                first.max_point(end)
            }
            _ => unreachable!(),
        }
    }

    pub fn min_point(&self, other: &Point) -> Point {
        match self {
            Command::Title(_)
            | Command::XLabel(_)
            | Command::YLabel(_)
            | Command::XUnits(_)
            | Command::YUnits(_)
            | Command::AspectRatio => other.clone(),
            Command::X(point)
            | Command::Dot(point)
            | Command::Plus(point)
            | Command::Box(point)
            | Command::Diamond(point)
            | Command::UTick(point)
            | Command::DTick(point)
            | Command::LTick(point)
            | Command::RTick(point)
            | Command::HTick(point)
            | Command::VTick(point)
            | Command::UArrow(point)
            | Command::DArrow(point)
            | Command::LArrow(point)
            | Command::RArrow(point)
            | Command::Invisible(point)
            | Command::AText(point, _)
            | Command::BText(point, _)
            | Command::CText(point, _)
            | Command::LText(point, _)
            | Command::RText(point, _) => point.min_point(other),
            Command::Line(start, end) | Command::DLine(start, end) => {
                let first = start.min_point(other);
                first.min_point(end)
            }
            _ => unreachable!(),
        }
    }
}
