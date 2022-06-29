use crate::status::Status;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Constraint {
    StringEquals(String),
    IntEquals(isize),
    IntRange(isize, isize),
    Boolean(bool),
}

impl Constraint {
    pub fn check(&self, val: &str) -> Status {
        match *self {
            Constraint::StringEquals(ref s) => {
                if val == s {
                    Status::Met
                } else {
                    Status::NotMet
                }
            }
            Constraint::IntEquals(i) => {
                let parse_res = val.parse::<isize>();
                if let Ok(val) = parse_res {
                    if val == i {
                        Status::Met
                    } else {
                        Status::NotMet
                    }
                } else {
                    Status::NotMet
                }
            }
            Constraint::IntRange(start, end) => {
                let parse_res = val.parse::<isize>();
                if let Ok(val) = parse_res {
                    if start <= val && val <= end {
                        Status::Met
                    } else {
                        Status::NotMet
                    }
                } else {
                    Status::NotMet
                }
            }
            Constraint::Boolean(b) => {
                let bool_val = &val.to_lowercase() == "true";
                if bool_val == b {
                    Status::Met
                } else {
                    Status::NotMet
                }
            }
        }
    }
}
