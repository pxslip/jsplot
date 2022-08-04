pub mod kind;

use std::cmp::Ordering;

use web_sys::CanvasRenderingContext2d;

use crate::units::Unit;

pub trait Command {
    fn draw(&self, context: CanvasRenderingContext2d);
    fn max_x<'a>(&'a self, x_value: &'a Unit) -> &'a Unit {
        x_value
    }
    fn max_y<'a>(&'a self, y_value: &'a Unit) -> &'a Unit {
        y_value
    }
}

pub struct Title {
    text: String,
}

impl Title {
    fn new(text: String) -> Title {
        Title { text }
    }
}

impl Command for Title {
    fn draw(&self, context: CanvasRenderingContext2d) {}
}

pub struct XLabel {
    text: String,
}

impl XLabel {
    fn new(text: String) -> XLabel {
        XLabel { text }
    }
}

impl Command for XLabel {
    fn draw(&self, context: CanvasRenderingContext2d) {}
}

pub struct YLabel {
    text: String,
}

impl YLabel {
    fn new(text: String) -> YLabel {
        YLabel { text }
    }
}

impl Command for YLabel {
    fn draw(&self, context: CanvasRenderingContext2d) {}
}

pub struct XUnits {
    text: String,
}

impl XUnits {
    fn new(text: String) -> XUnits {
        XUnits { text }
    }
}

impl Command for XUnits {
    fn draw(&self, context: CanvasRenderingContext2d) {}
}

pub struct YUnits {
    text: String,
}

impl YUnits {
    fn new(text: String) -> YUnits {
        YUnits { text }
    }
}

impl Command for YUnits {
    fn draw(&self, context: CanvasRenderingContext2d) {}
}

pub struct Times {
    x_coordinate: Unit,
    y_coordinate: Unit,
}

impl Times {
    fn new(x_coordinate: Unit, y_coordinate: Unit) -> Times {
        Times {
            x_coordinate,
            y_coordinate,
        }
    }
}

impl Command for Times {
    fn draw(&self, context: CanvasRenderingContext2d) {}
    fn max_x<'a>(&'a self, x_value: &'a Unit) -> &'a Unit {
        match self
            .x_coordinate
            .partial_cmp(&x_value)
            .expect("Comparison of units failed unexpectedly")
        {
            Ordering::Less => x_value,
            Ordering::Equal => x_value,
            Ordering::Greater => &self.x_coordinate,
        }
    }

    // fn max_y<'a>(&'a self, y_value: &'a Unit) -> &'a Unit {}
}

pub struct Dot {
    x_coordinate: Unit,
    y_coordinate: Unit,
}

impl Dot {
    fn new(x_coordinate: Unit, y_coordinate: Unit) -> Dot {
        Dot {
            x_coordinate,
            y_coordinate,
        }
    }
}

impl Command for Dot {
    fn draw(&self, context: CanvasRenderingContext2d) {}
}

pub struct Plus {
    x_coordinate: Unit,
    y_coordinate: Unit,
}

impl Plus {
    fn new(x_coordinate: Unit, y_coordinate: Unit) -> Plus {
        Plus {
            x_coordinate,
            y_coordinate,
        }
    }
}

impl Command for Plus {
    fn draw(&self, context: CanvasRenderingContext2d) {}
}

pub struct Box {
    x_coordinate: Unit,
    y_coordinate: Unit,
}

impl Box {
    fn new(x_coordinate: Unit, y_coordinate: Unit) -> Box {
        Box {
            x_coordinate,
            y_coordinate,
        }
    }
}

impl Command for Box {
    fn draw(&self, context: CanvasRenderingContext2d) {}
}

pub struct Diamond {
    x_coordinate: Unit,
    y_coordinate: Unit,
}

impl Diamond {
    fn new(x_coordinate: Unit, y_coordinate: Unit) -> Diamond {
        Diamond {
            x_coordinate,
            y_coordinate,
        }
    }
}

impl Command for Diamond {
    fn draw(&self, context: CanvasRenderingContext2d) {}
}

pub struct UpTick {
    x_coordinate: Unit,
    y_coordinate: Unit,
}

impl UpTick {
    fn new(x_coordinate: Unit, y_coordinate: Unit) -> UpTick {
        UpTick {
            x_coordinate,
            y_coordinate,
        }
    }
}

impl Command for UpTick {
    fn draw(&self, context: CanvasRenderingContext2d) {}
}

pub struct DownTick {
    x_coordinate: Unit,
    y_coordinate: Unit,
}

impl DownTick {
    fn new(x_coordinate: Unit, y_coordinate: Unit) -> DownTick {
        DownTick {
            x_coordinate,
            y_coordinate,
        }
    }
}

impl Command for DownTick {
    fn draw(&self, context: CanvasRenderingContext2d) {}
}

pub struct LeftTick {
    x_coordinate: Unit,
    y_coordinate: Unit,
}

impl LeftTick {
    fn new(x_coordinate: Unit, y_coordinate: Unit) -> LeftTick {
        LeftTick {
            x_coordinate,
            y_coordinate,
        }
    }
}

impl Command for LeftTick {
    fn draw(&self, context: CanvasRenderingContext2d) {}
}

pub struct RightTick {
    x_coordinate: Unit,
    y_coordinate: Unit,
}

impl RightTick {
    fn new(x_coordinate: Unit, y_coordinate: Unit) -> RightTick {
        RightTick {
            x_coordinate,
            y_coordinate,
        }
    }
}

impl Command for RightTick {
    fn draw(&self, context: CanvasRenderingContext2d) {}
}

pub struct HorizontalTick {
    x_coordinate: Unit,
    y_coordinate: Unit,
}

impl HorizontalTick {
    fn new(x_coordinate: Unit, y_coordinate: Unit) -> HorizontalTick {
        HorizontalTick {
            x_coordinate,
            y_coordinate,
        }
    }
}

impl Command for HorizontalTick {
    fn draw(&self, context: CanvasRenderingContext2d) {}
}

pub struct VerticalTick {
    x_coordinate: Unit,
    y_coordinate: Unit,
}

impl VerticalTick {
    fn new(x_coordinate: Unit, y_coordinate: Unit) -> VerticalTick {
        VerticalTick {
            x_coordinate,
            y_coordinate,
        }
    }
}

impl Command for VerticalTick {
    fn draw(&self, context: CanvasRenderingContext2d) {}
}

pub struct UpArrow {
    x_coordinate: Unit,
    y_coordinate: Unit,
}

impl UpArrow {
    fn new(x_coordinate: Unit, y_coordinate: Unit) -> UpArrow {
        UpArrow {
            x_coordinate,
            y_coordinate,
        }
    }
}

impl Command for UpArrow {
    fn draw(&self, context: CanvasRenderingContext2d) {}
}

pub struct DownArrow {
    x_coordinate: Unit,
    y_coordinate: Unit,
}

impl DownArrow {
    fn new(x_coordinate: Unit, y_coordinate: Unit) -> DownArrow {
        DownArrow {
            x_coordinate,
            y_coordinate,
        }
    }
}

impl Command for DownArrow {
    fn draw(&self, context: CanvasRenderingContext2d) {}
}

pub struct LeftArrow {
    x_coordinate: Unit,
    y_coordinate: Unit,
}

impl LeftArrow {
    fn new(x_coordinate: Unit, y_coordinate: Unit) -> LeftArrow {
        LeftArrow {
            x_coordinate,
            y_coordinate,
        }
    }
}

impl Command for LeftArrow {
    fn draw(&self, context: CanvasRenderingContext2d) {}
}

pub struct RightArrow {
    x_coordinate: Unit,
    y_coordinate: Unit,
}

impl RightArrow {
    fn new(x_coordinate: Unit, y_coordinate: Unit) -> RightArrow {
        RightArrow {
            x_coordinate,
            y_coordinate,
        }
    }
}

impl Command for RightArrow {
    fn draw(&self, context: CanvasRenderingContext2d) {}
}

pub struct Invisible {
    x_coordinate: Unit,
    y_coordinate: Unit,
}

impl Invisible {
    fn new(x_coordinate: Unit, y_coordinate: Unit) -> Invisible {
        Invisible {
            x_coordinate,
            y_coordinate,
        }
    }
}

impl Command for Invisible {
    fn draw(&self, context: CanvasRenderingContext2d) {}
}

pub struct Line {
    x_start_coordinate: Unit,
    y_start_coordinate: Unit,
    x_end_coordinate: Unit,
    y_end_coordinate: Unit,
}

impl Line {
    fn new(
        x_start_coordinate: Unit,
        y_start_coordinate: Unit,
        x_end_coordinate: Unit,
        y_end_coordinate: Unit,
    ) -> Line {
        Line {
            x_start_coordinate,
            y_start_coordinate,
            x_end_coordinate,
            y_end_coordinate,
        }
    }
}

impl Command for Line {
    fn draw(&self, context: CanvasRenderingContext2d) {}
}

pub struct DottedLine {
    x_start_coordinate: Unit,
    y_start_coordinate: Unit,
    x_end_coordinate: Unit,
    y_end_coordinate: Unit,
}

impl DottedLine {
    fn new(
        x_start_coordinate: Unit,
        y_start_coordinate: Unit,
        x_end_coordinate: Unit,
        y_end_coordinate: Unit,
    ) -> DottedLine {
        DottedLine {
            x_start_coordinate,
            y_start_coordinate,
            x_end_coordinate,
            y_end_coordinate,
        }
    }
}

impl Command for DottedLine {
    fn draw(&self, context: CanvasRenderingContext2d) {}
}

pub struct AboveText {
    text: String,
    x_coordinate: Unit,
    y_coordinate: Unit,
}

impl AboveText {
    fn new(text: String, x_coordinate: Unit, y_coordinate: Unit) -> AboveText {
        AboveText {
            text,
            x_coordinate,
            y_coordinate,
        }
    }
}

impl Command for AboveText {
    fn draw(&self, context: CanvasRenderingContext2d) {}
}

pub struct BelowText {
    text: String,
    x_coordinate: Unit,
    y_coordinate: Unit,
}

impl BelowText {
    fn new(text: String, x_coordinate: Unit, y_coordinate: Unit) -> BelowText {
        BelowText {
            text,
            x_coordinate,
            y_coordinate,
        }
    }
}

impl Command for BelowText {
    fn draw(&self, context: CanvasRenderingContext2d) {}
}

pub struct CenterText {
    text: String,
    x_coordinate: Unit,
    y_coordinate: Unit,
}

impl CenterText {
    fn new(text: String, x_coordinate: Unit, y_coordinate: Unit) -> CenterText {
        CenterText {
            text,
            x_coordinate,
            y_coordinate,
        }
    }
}

impl Command for CenterText {
    fn draw(&self, context: CanvasRenderingContext2d) {}
}

pub struct LeftText {
    text: String,
    x_coordinate: Unit,
    y_coordinate: Unit,
}

impl LeftText {
    fn new(text: String, x_coordinate: Unit, y_coordinate: Unit) -> LeftText {
        LeftText {
            text,
            x_coordinate,
            y_coordinate,
        }
    }
}

impl Command for LeftText {
    fn draw(&self, context: CanvasRenderingContext2d) {}
}

pub struct RightText {
    text: String,
    x_coordinate: Unit,
    y_coordinate: Unit,
}

impl RightText {
    fn new(text: String, x_coordinate: Unit, y_coordinate: Unit) -> RightText {
        RightText {
            text,
            x_coordinate,
            y_coordinate,
        }
    }
}

impl Command for RightText {
    fn draw(&self, context: CanvasRenderingContext2d) {}
}
