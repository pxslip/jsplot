use std::{
    cmp::Ordering,
    convert::{TryFrom, TryInto},
    ops::{Add, Mul},
};

use strum_macros::EnumString;
/**
 * The types of units that are supported
 */
#[derive(EnumString, Clone)]
#[strum(serialize_all = "snake_case", ascii_case_insensitive)]
pub enum Unit {
    Unsigned(u32),
    Signed(i32),
    Timeval(f32),
    Double(f32),
    DTime(f32),
}

impl Default for Unit {
    fn default() -> Self {
        Unit::Unsigned(0)
    }
}

impl Unit {
    pub fn from_value(unit_type: &str, unit_value: String) -> Unit {
        match unit_type {
            "unsigned" => Unit::Unsigned(
                unit_value
                    .parse::<u32>()
                    .expect("Error parsing an unsigned unit"),
            ),
            "signed" => Unit::Signed(
                unit_value
                    .parse::<i32>()
                    .expect("Error parsing a signed unit"),
            ),
            "timeval" => Unit::Timeval(
                unit_value
                    .parse::<f32>()
                    .expect("Error parsing a timeval unit"),
            ),
            "double" => Unit::Double(
                unit_value
                    .parse::<f32>()
                    .expect("Error parsing a double unit"),
            ),
            "dtime" => Unit::DTime(
                unit_value
                    .parse::<f32>()
                    .expect("Error parsing a dtime unit"),
            ),
            _ => panic!("Reached the wildcard arm of the from_value Unit parsing code"),
        }
    }

    pub fn abs(&self) -> Unit {
        match self {
            Unit::Unsigned(self_val) => Unit::Unsigned(*self_val),
            Unit::Signed(self_val) => Unit::Signed(self_val.abs()),
            Unit::Timeval(self_val) => Unit::Timeval(self_val.abs()),
            Unit::Double(self_val) => Unit::Double(self_val.abs()),
            Unit::DTime(self_val) => Unit::DTime(self_val.abs()),
        }
    }

    pub fn mul_by_f32(self, rhs: f32) -> Self {
        match self {
            Unit::Unsigned(self_val) => Unit::Unsigned(self_val / rhs as u32),
            Unit::Signed(self_val) => Unit::Signed(self_val / rhs as i32),
            Unit::Timeval(self_val) => Unit::Timeval(self_val / rhs),
            Unit::Double(self_val) => Unit::Double(self_val / rhs),
            Unit::DTime(self_val) => Unit::DTime(self_val / rhs),
        }
    }
}

impl TryFrom<u64> for Unit {
    type Error = &'static str;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        match value.try_into() {
            Ok(val) => Ok(Unit::Unsigned(val)),
            Err(_) => Err("There was an issue converting a u64 value to a Unit::Unsigned"),
        }
    }
}

impl TryFrom<i64> for Unit {
    type Error = &'static str;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        match value.try_into() {
            Ok(val) => Ok(Unit::Signed(val)),
            Err(_) => Err("There was an issue converting a u64 value to a Unit::Unsigned"),
        }
    }
}

impl From<f64> for Unit {
    fn from(value: f64) -> Self {
        Unit::Double(value as f32)
    }
}

impl From<Unit> for f64 {
    fn from(unit: Unit) -> Self {
        match unit {
            Unit::Signed(val) => val as f64,
            Unit::Unsigned(val) => val as f64,
            Unit::Double(val) | Unit::Timeval(val) | Unit::DTime(val) => val as f64,
        }
    }
}

impl PartialOrd for Unit {
    fn partial_cmp(&self, other: &Self) -> std::option::Option<std::cmp::Ordering> {
        match self {
            Unit::Unsigned(self_val) => {
                if let Unit::Unsigned(other_val) = other {
                    self_val.partial_cmp(other_val)
                } else {
                    unreachable!()
                }
            }
            Unit::Signed(self_val) => {
                if let Unit::Signed(other_val) = other {
                    self_val.partial_cmp(other_val)
                } else {
                    unreachable!()
                }
            }
            Unit::Timeval(self_val) => {
                if let Unit::Timeval(other_val) = other {
                    self_val.partial_cmp(other_val)
                } else {
                    unreachable!()
                }
            }
            Unit::Double(self_val) => {
                if let Unit::Double(other_val) = other {
                    self_val.partial_cmp(other_val)
                } else {
                    unreachable!()
                }
            }
            Unit::DTime(self_val) => {
                if let Unit::DTime(other_val) = other {
                    self_val.partial_cmp(other_val)
                } else {
                    unreachable!()
                }
            }
        }
    }
}

impl PartialEq for Unit {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Unit::Unsigned(self_val) => {
                if let Unit::Unsigned(other_val) = other {
                    self_val.eq(other_val)
                } else {
                    unreachable!()
                }
            }
            Unit::Signed(self_val) => {
                if let Unit::Signed(other_val) = other {
                    self_val.eq(other_val)
                } else {
                    unreachable!()
                }
            }
            Unit::Timeval(self_val) => {
                if let Unit::Timeval(other_val) = other {
                    self_val.eq(other_val)
                } else {
                    unreachable!()
                }
            }
            Unit::Double(self_val) => {
                if let Unit::Double(other_val) = other {
                    self_val.eq(other_val)
                } else {
                    unreachable!()
                }
            }
            Unit::DTime(self_val) => {
                if let Unit::DTime(other_val) = other {
                    self_val.eq(other_val)
                } else {
                    unreachable!()
                }
            }
        }
    }
}

impl Add for Unit {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        match self {
            Unit::Unsigned(self_val) => {
                if let Unit::Unsigned(other_val) = other {
                    Unit::Unsigned(self_val.add(other_val))
                } else {
                    unreachable!()
                }
            }
            Unit::Signed(self_val) => {
                if let Unit::Signed(other_val) = other {
                    Unit::Signed(self_val.add(other_val))
                } else {
                    unreachable!()
                }
            }
            Unit::Timeval(self_val) => {
                if let Unit::Timeval(other_val) = other {
                    Unit::Timeval(self_val.add(other_val))
                } else {
                    unreachable!()
                }
            }
            Unit::Double(self_val) => {
                if let Unit::Double(other_val) = other {
                    Unit::Double(self_val.add(other_val))
                } else {
                    unreachable!()
                }
            }
            Unit::DTime(self_val) => {
                if let Unit::DTime(other_val) = other {
                    Unit::DTime(self_val.add(other_val))
                } else {
                    unreachable!()
                }
            }
        }
    }
}

impl Mul for Unit {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        match self {
            Unit::Unsigned(self_val) => match rhs {
                Unit::Unsigned(rhs_val) => Unit::Unsigned(self_val * rhs_val),
                Unit::Signed(rhs_val) => Unit::Signed(self_val as i32 * rhs_val),
                Unit::Timeval(rhs_val) => Unit::Timeval(self_val as f32 * rhs_val),
                Unit::Double(rhs_val) => Unit::Double(self_val as f32 * rhs_val),
                Unit::DTime(rhs_val) => Unit::DTime(self_val as f32 * rhs_val),
            },
            Unit::Signed(self_val) => match rhs {
                Unit::Unsigned(rhs_val) => Unit::Unsigned(self_val as u32 * rhs_val),
                Unit::Signed(rhs_val) => Unit::Signed(self_val * rhs_val),
                Unit::Timeval(rhs_val) => Unit::Timeval(self_val as f32 * rhs_val),
                Unit::Double(rhs_val) => Unit::Double(self_val as f32 * rhs_val),
                Unit::DTime(rhs_val) => Unit::DTime(self_val as f32 * rhs_val),
            },
            Unit::Timeval(self_val) => match rhs {
                Unit::Unsigned(rhs_val) => Unit::Unsigned(self_val as u32 * rhs_val),
                Unit::Signed(rhs_val) => Unit::Signed(self_val as i32 * rhs_val),
                Unit::Timeval(rhs_val) => Unit::Timeval(self_val * rhs_val),
                Unit::Double(rhs_val) => Unit::Double(self_val * rhs_val),
                Unit::DTime(rhs_val) => Unit::DTime(self_val * rhs_val),
            },
            Unit::Double(self_val) => match rhs {
                Unit::Unsigned(rhs_val) => Unit::Unsigned(self_val as u32 * rhs_val),
                Unit::Signed(rhs_val) => Unit::Signed(self_val as i32 * rhs_val),
                Unit::Timeval(rhs_val) => Unit::Timeval(self_val * rhs_val),
                Unit::Double(rhs_val) => Unit::Double(self_val * rhs_val),
                Unit::DTime(rhs_val) => Unit::DTime(self_val * rhs_val),
            },
            Unit::DTime(self_val) => match rhs {
                Unit::Unsigned(rhs_val) => Unit::Unsigned(self_val as u32 * rhs_val),
                Unit::Signed(rhs_val) => Unit::Signed(self_val as i32 * rhs_val),
                Unit::Timeval(rhs_val) => Unit::Timeval(self_val * rhs_val),
                Unit::Double(rhs_val) => Unit::Double(self_val * rhs_val),
                Unit::DTime(rhs_val) => Unit::DTime(self_val * rhs_val),
            },
        }
    }
}

impl std::ops::Div<Unit> for u32 {
    type Output = f32;
    fn div(self, rhs: Unit) -> Self::Output {
        let self_val = self as f32;
        match rhs {
            Unit::Unsigned(rhs_val) => self_val / rhs_val as f32,
            Unit::Signed(rhs_val) => self_val / rhs_val as f32,
            Unit::Timeval(rhs_val) => self_val / rhs_val,
            Unit::Double(rhs_val) => self_val / rhs_val,
            Unit::DTime(rhs_val) => self_val / rhs_val,
        }
    }
}

#[derive(Clone, Default)]
pub struct Point {
    pub x: Unit,
    pub y: Unit,
}

impl Point {
    pub fn new(x: Unit, y: Unit) -> Point {
        Point { x, y }
    }

    pub fn new_with_defaults() -> Point {
        Point {
            x: Unit::Unsigned(0),
            y: Unit::Unsigned(0),
        }
    }

    pub fn new_from_strings(x_type: &str, y_type: &str, x_str: String, y_str: String) -> Point {
        let x = Unit::from_value(x_type, x_str);
        let y = Unit::from_value(y_type, y_str);
        Point::new(x, y)
    }

    pub fn max_point(&self, other: &Point) -> Point {
        let max_x = match self.x.partial_cmp(&other.x).unwrap() {
            Ordering::Less => &other.x,
            Ordering::Equal => &other.x,
            Ordering::Greater => &self.x,
        };
        let max_y = match self.y.partial_cmp(&other.y).unwrap() {
            Ordering::Less => &other.y,
            Ordering::Equal => &other.y,
            Ordering::Greater => &self.y,
        };
        Point {
            x: max_x.clone(),
            y: max_y.clone(),
        }
    }

    pub fn min_point(&self, other: &Point) -> Point {
        let x = match self.x.partial_cmp(&other.x).unwrap() {
            Ordering::Less => &self.x,
            Ordering::Equal => &other.x,
            Ordering::Greater => &other.x,
        };
        let y = match self.y.partial_cmp(&other.y).unwrap() {
            Ordering::Less => &self.y,
            Ordering::Equal => &other.y,
            Ordering::Greater => &other.y,
        };
        Point {
            x: x.clone(),
            y: y.clone(),
        }
    }
}
