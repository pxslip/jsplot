use crate::commands::*;
use crate::Plotter;

use std::boxed::Box as StdBox;

use strum_macros::EnumString;
/**
 * All possible commands
 */
#[derive(EnumString)]
#[strum(serialize_all = "snake_case", ascii_case_insensitive)]
pub enum CommandKind {
    NewPlotter,
    Title,
    XLabel,
    YLabel,
    XUnits,
    YUnits,
    AspectRatio,
    X,
    #[strum(serialize = ".")]
    Dot,
    #[strum(serialize = "+")]
    Plus,
    Box,
    Diamond,
    UTick,
    DTick,
    LTick,
    RTick,
    HTick,
    VTick,
    UArrow,
    DArrow,
    LArrow,
    RArrow,
    Invisible,
    Line,
    DLine,
    AText,
    BText,
    CText,
    LText,
    RText,
}

/**
 * The different commands implementation details
 */
impl CommandKind {
    pub fn is_multiline(&self) -> bool {
        matches!(
            self,
            CommandKind::Title
                | CommandKind::XLabel
                | CommandKind::XUnits
                | CommandKind::YLabel
                | CommandKind::YUnits
                | CommandKind::AText
                | CommandKind::BText
                | CommandKind::CText
                | CommandKind::LText
                | CommandKind::RText
        )
    }
    pub fn is_disabled(&self) -> bool {
        matches!(self, CommandKind::NewPlotter | CommandKind::AspectRatio)
    }

    pub fn new_command(
        plotter: &Plotter,
        kind: CommandKind,
        args: Vec<String>,
    ) -> StdBox<dyn Command> {
        match kind {
            CommandKind::Title => StdBox::new(Title::new(
                args.get(0)
                    .expect("Error getting the title from the args vec")
                    .to_string(),
            )),
            CommandKind::XLabel => StdBox::new(XLabel::new(
                args.get(0)
                    .expect("Error getting the x label from the args vec")
                    .to_string(),
            )),
            CommandKind::YLabel => StdBox::new(YLabel::new(
                args.get(0)
                    .expect("Error getting the y label from the args vec")
                    .to_string(),
            )),
            CommandKind::XUnits => StdBox::new(XUnits::new(
                args.get(0)
                    .expect("Error getting the x label from the args vec")
                    .to_string(),
            )),
            CommandKind::YUnits => StdBox::new(YUnits::new(
                args.get(0)
                    .expect("Error getting the x label from the args vec")
                    .to_string(),
            )),
            CommandKind::X => StdBox::new(Times::new(
                plotter.parse_x_unit(
                    args.get(0)
                        .expect("Error getting x unit value from args vec")
                        .to_string(),
                ),
                plotter.parse_y_unit(
                    args.get(1)
                        .expect("Error getting the y coordinate from the args vec")
                        .to_string(),
                ),
            )),
            CommandKind::Dot => StdBox::new(Dot::new(
                plotter.parse_x_unit(
                    args.get(0)
                        .expect("Error getting x unit value from args vec")
                        .to_string(),
                ),
                plotter.parse_y_unit(
                    args.get(1)
                        .expect("Error getting the y coordinate from the args vec")
                        .to_string(),
                ),
            )),
            CommandKind::Plus => StdBox::new(Plus::new(
                plotter.parse_x_unit(
                    args.get(0)
                        .expect("Error getting x unit value from args vec")
                        .to_string(),
                ),
                plotter.parse_y_unit(
                    args.get(1)
                        .expect("Error getting the y coordinate from the args vec")
                        .to_string(),
                ),
            )),
            CommandKind::Box => StdBox::new(Box::new(
                plotter.parse_x_unit(
                    args.get(0)
                        .expect("Error getting x unit value from args vec")
                        .to_string(),
                ),
                plotter.parse_y_unit(
                    args.get(1)
                        .expect("Error getting the y coordinate from the args vec")
                        .to_string(),
                ),
            )),
            CommandKind::Diamond => StdBox::new(Diamond::new(
                plotter.parse_x_unit(
                    args.get(0)
                        .expect("Error getting x unit value from args vec")
                        .to_string(),
                ),
                plotter.parse_y_unit(
                    args.get(1)
                        .expect("Error getting the y coordinate from the args vec")
                        .to_string(),
                ),
            )),
            CommandKind::UTick => StdBox::new(UpTick::new(
                plotter.parse_x_unit(
                    args.get(0)
                        .expect("Error getting x unit value from args vec")
                        .to_string(),
                ),
                plotter.parse_y_unit(
                    args.get(1)
                        .expect("Error getting the y coordinate from the args vec")
                        .to_string(),
                ),
            )),
            CommandKind::DTick => StdBox::new(DownTick::new(
                plotter.parse_x_unit(
                    args.get(0)
                        .expect("Error getting x unit value from args vec")
                        .to_string(),
                ),
                plotter.parse_y_unit(
                    args.get(1)
                        .expect("Error getting the y coordinate from the args vec")
                        .to_string(),
                ),
            )),
            CommandKind::LTick => StdBox::new(LeftTick::new(
                plotter.parse_x_unit(
                    args.get(0)
                        .expect("Error getting x unit value from args vec")
                        .to_string(),
                ),
                plotter.parse_y_unit(
                    args.get(1)
                        .expect("Error getting the y coordinate from the args vec")
                        .to_string(),
                ),
            )),
            CommandKind::RTick => StdBox::new(RightTick::new(
                plotter.parse_x_unit(
                    args.get(0)
                        .expect("Error getting x unit value from args vec")
                        .to_string(),
                ),
                plotter.parse_y_unit(
                    args.get(1)
                        .expect("Error getting the y coordinate from the args vec")
                        .to_string(),
                ),
            )),
            CommandKind::HTick => StdBox::new(HorizontalTick::new(
                plotter.parse_x_unit(
                    args.get(0)
                        .expect("Error getting x unit value from args vec")
                        .to_string(),
                ),
                plotter.parse_y_unit(
                    args.get(1)
                        .expect("Error getting the y coordinate from the args vec")
                        .to_string(),
                ),
            )),
            CommandKind::VTick => StdBox::new(VerticalTick::new(
                plotter.parse_x_unit(
                    args.get(0)
                        .expect("Error getting x unit value from args vec")
                        .to_string(),
                ),
                plotter.parse_y_unit(
                    args.get(1)
                        .expect("Error getting the y coordinate from the args vec")
                        .to_string(),
                ),
            )),
            CommandKind::UArrow => StdBox::new(UpArrow::new(
                plotter.parse_x_unit(
                    args.get(0)
                        .expect("Error getting x unit value from args vec")
                        .to_string(),
                ),
                plotter.parse_y_unit(
                    args.get(1)
                        .expect("Error getting the y coordinate from the args vec")
                        .to_string(),
                ),
            )),
            CommandKind::DArrow => StdBox::new(DownArrow::new(
                plotter.parse_x_unit(
                    args.get(0)
                        .expect("Error getting x unit value from args vec")
                        .to_string(),
                ),
                plotter.parse_y_unit(
                    args.get(1)
                        .expect("Error getting the y coordinate from the args vec")
                        .to_string(),
                ),
            )),
            CommandKind::LArrow => StdBox::new(LeftArrow::new(
                plotter.parse_x_unit(
                    args.get(0)
                        .expect("Error getting x unit value from args vec")
                        .to_string(),
                ),
                plotter.parse_y_unit(
                    args.get(1)
                        .expect("Error getting the y coordinate from the args vec")
                        .to_string(),
                ),
            )),
            CommandKind::RArrow => StdBox::new(RightArrow::new(
                plotter.parse_x_unit(
                    args.get(0)
                        .expect("Error getting x unit value from args vec")
                        .to_string(),
                ),
                plotter.parse_y_unit(
                    args.get(1)
                        .expect("Error getting the y coordinate from the args vec")
                        .to_string(),
                ),
            )),
            CommandKind::Invisible => StdBox::new(Invisible::new(
                plotter.parse_x_unit(
                    args.get(0)
                        .expect("Error getting x unit value from args vec")
                        .to_string(),
                ),
                plotter.parse_y_unit(
                    args.get(1)
                        .expect("Error getting the y coordinate from the args vec")
                        .to_string(),
                ),
            )),
            CommandKind::Line => StdBox::new(Line::new(
                plotter.parse_x_unit(
                    args.get(0)
                        .expect("Error getting x unit value from args vec")
                        .to_string(),
                ),
                plotter.parse_y_unit(
                    args.get(1)
                        .expect("Error getting the y coordinate from the args vec")
                        .to_string(),
                ),
                plotter.parse_x_unit(
                    args.get(2)
                        .expect("Error getting x unit value from args vec")
                        .to_string(),
                ),
                plotter.parse_y_unit(
                    args.get(3)
                        .expect("Error getting x unit value from args vec")
                        .to_string(),
                ),
            )),
            CommandKind::DLine => StdBox::new(Line::new(
                plotter.parse_x_unit(
                    args.get(0)
                        .expect("Error getting x unit value from args vec")
                        .to_string(),
                ),
                plotter.parse_y_unit(
                    args.get(1)
                        .expect("Error getting the y coordinate from the args vec")
                        .to_string(),
                ),
                plotter.parse_x_unit(
                    args.get(2)
                        .expect("Error getting x unit value from args vec")
                        .to_string(),
                ),
                plotter.parse_y_unit(
                    args.get(3)
                        .expect("Error getting x unit value from args vec")
                        .to_string(),
                ),
            )),
            CommandKind::AText => StdBox::new(AboveText::new(
                args.get(2)
                    .expect("Error getting the text from the args vec")
                    .to_string(),
                plotter.parse_x_unit(
                    args.get(0)
                        .expect("Error getting x unit value from args vec")
                        .to_string(),
                ),
                plotter.parse_y_unit(
                    args.get(1)
                        .expect("Error getting the y coordinate from the args vec")
                        .to_string(),
                ),
            )),
            CommandKind::BText => StdBox::new(BelowText::new(
                args.get(2)
                    .expect("Error getting the text from the args vec")
                    .to_string(),
                plotter.parse_x_unit(
                    args.get(0)
                        .expect("Error getting x unit value from args vec")
                        .to_string(),
                ),
                plotter.parse_y_unit(
                    args.get(1)
                        .expect("Error getting the y coordinate from the args vec")
                        .to_string(),
                ),
            )),
            CommandKind::CText => StdBox::new(CenterText::new(
                args.get(2)
                    .expect("Error getting the text from the args vec")
                    .to_string(),
                plotter.parse_x_unit(
                    args.get(0)
                        .expect("Error getting x unit value from args vec")
                        .to_string(),
                ),
                plotter.parse_y_unit(
                    args.get(1)
                        .expect("Error getting the y coordinate from the args vec")
                        .to_string(),
                ),
            )),
            CommandKind::LText => StdBox::new(LeftText::new(
                args.get(2)
                    .expect("Error getting the text from the args vec")
                    .to_string(),
                plotter.parse_x_unit(
                    args.get(0)
                        .expect("Error getting x unit value from args vec")
                        .to_string(),
                ),
                plotter.parse_y_unit(
                    args.get(1)
                        .expect("Error getting the y coordinate from the args vec")
                        .to_string(),
                ),
            )),
            CommandKind::RText => StdBox::new(RightText::new(
                args.get(2)
                    .expect("Error getting the text from the args vec")
                    .to_string(),
                plotter.parse_x_unit(
                    args.get(0)
                        .expect("Error getting x unit value from args vec")
                        .to_string(),
                ),
                plotter.parse_y_unit(
                    args.get(1)
                        .expect("Error getting the y coordinate from the args vec")
                        .to_string(),
                ),
            )),
            _ => {
                panic!("Unexpectedly hit the catch all arm of CommandKind::newCommand")
            }
        }
    }
}
