use strum_macros::EnumString;
/**
 * The types of units that are supported
 */
#[derive(EnumString)]
#[strum(serialize_all = "snake_case", ascii_case_insensitive)]
pub enum Unit {
    Unsigned(u64),
    Signed(i64),
    Timeval(f64),
    Double(f64),
    DTime(f64),
}

impl Unit {
    pub fn from_value(unit_type: &str, unit_value: String) -> Unit {
        match unit_type {
            "unsigned" => Unit::Unsigned(
                unit_value
                    .parse::<u64>()
                    .expect("Error parsing an unsigned unit"),
            ),
            "signed" => Unit::Signed(
                unit_value
                    .parse::<i64>()
                    .expect("Error parsing a signed unit"),
            ),
            "timeval" => Unit::Timeval(
                unit_value
                    .parse::<f64>()
                    .expect("Error parsing a timeval unit"),
            ),
            "double" => Unit::Double(
                unit_value
                    .parse::<f64>()
                    .expect("Error parsing a double unit"),
            ),
            "dtime" => Unit::DTime(
                unit_value
                    .parse::<f64>()
                    .expect("Error parsing a dtime unit"),
            ),
            _ => panic!("Reached the wildcard arm of the from_value Unit parsing code"),
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
