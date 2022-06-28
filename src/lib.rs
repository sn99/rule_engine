use std::collections::BTreeMap;
use std::ops::{BitAnd, BitOr};

use serde::{Serialize, Deserialize};

// ***********************************************************************
// STATUS
// **********************************************************************
/// The status of a rule check
#[derive(Debug, Eq, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub enum Status {
    /// Rule was satisfied
    Met,
    /// Rule was not satisfied
    NotMet,
    /// There was not enough information to evaluate
    Unknown,
}

impl BitAnd for Status {
    type Output = Status;
    fn bitand(self, rhs: Status) -> Status {
        match (self, rhs) {
            (Status::Met, Status::Met) => Status::Met,
            (Status::NotMet, _) | (_, Status::NotMet) => Status::NotMet,
            (_, _) => Status::Unknown,
        }
    }
}

impl BitOr for Status {
    type Output = Status;
    fn bitor(self, rhs: Status) -> Status {
        match (self, rhs) {
            (Status::NotMet, Status::NotMet) => Status::NotMet,
            (Status::Met, _) | (_, Status::Met) => Status::Met,
            (_, _) => Status::Unknown,
        }
    }
}

// ***********************************************************************
// Rule
// **********************************************************************

/// Representation of a node in the rules tree
///
/// It is unnecessary to interact with this type outside of calling `Rule::check()`,
/// to construct the rules tree use the [convenience functions][1] in the module root.
///
/// [1]: index.html#functions
#[derive(Debug, Serialize, Deserialize)]
pub enum Rule {
    And {
        rules: Vec<Rule>,
    },
    Or {
        rules: Vec<Rule>,
    },
    NumberOf {
        n: usize,
        rules: Vec<Rule>,
    },
    // Rule(Description, Field, Constraint)
    Rule {
        desc: String,
        field: String,
        constraint: Constraint,
    },
}

impl Rule {
    /// Starting at this node, recursively check (depth-first) any child nodes and
    /// aggregate the results
    pub fn check(&self, info: &BTreeMap<String, String>) -> RuleResult {
        match *self {
            Rule::And { ref rules } => {
                let mut status = Status::Met;
                let children = rules
                    .iter()
                    .map(|c| c.check(info))
                    .inspect(|r| status = status & r.status)
                    .collect::<Vec<_>>();
                RuleResult {
                    name: "And".into(),
                    status,
                    children,
                }
            }
            Rule::Or { ref rules } => {
                let mut status = Status::NotMet;
                let children = rules
                    .iter()
                    .map(|c| c.check(info))
                    .inspect(|r| status = status | r.status)
                    .collect::<Vec<_>>();
                RuleResult {
                    name: "Or".into(),
                    status,
                    children,
                }
            }
            Rule::NumberOf {
                n: count,
                ref rules,
            } => {
                let mut met_count = 0;
                let mut failed_count = 0;
                let children = rules
                    .iter()
                    .map(|c| c.check(info))
                    .inspect(|r| {
                        if r.status == Status::Met {
                            met_count += 1;
                        } else if r.status == Status::NotMet {
                            failed_count += 1;
                        }
                    })
                    .collect::<Vec<_>>();
                let status = if met_count >= count {
                    Status::Met
                } else if failed_count > children.len() - count {
                    Status::NotMet
                } else {
                    Status::Unknown
                };
                RuleResult {
                    name: format!("At least {} of", count),
                    status,
                    children,
                }
            }
            Rule::Rule {
                desc: ref name,
                ref field,
                ref constraint,
            } => {
                let status = if let Some(s) = info.get(field) {
                    constraint.check(s)
                } else {
                    Status::Unknown
                };
                RuleResult {
                    name: name.to_owned(),
                    status,
                    children: Vec::new(),
                }
            }
        }
    }
}

// ***********************************************************************
// CONSTRAINT
// **********************************************************************
#[derive(Debug, Serialize, Deserialize)]
pub enum Constraint {
    StringEquals(String),
    IntEquals(i32),
    IntRange(i32, i32),
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
                let parse_res = val.parse::<i32>();
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
                let parse_res = val.parse::<i32>();
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

// ***********************************************************************
// Rule RESULT
// **********************************************************************
/// Result of checking a rules tree.
#[derive(Debug, Serialize, Deserialize)]
pub struct RuleResult {
    /// Human-friendly description of the rule
    pub name: String,
    /// top-level status of this result
    pub status: Status,
    /// Results of any sub-rules
    pub children: Vec<RuleResult>,
}

/// Creates a `Rule` where all child `Rule`s must be `Met`
///
/// * If any are `NotMet`, the result will be `NotMet`
/// * If the results contain only `Met` and `Unknown`, the result will be `Unknown`
/// * Only results in `Met` if all children are `Met`
pub fn and(rules: Vec<Rule>) -> Rule {
    Rule::And { rules }
}

/// Creates a `Rule` where any child `Rule` must be `Met`
///
/// * If any are `Met`, the result will be `Met`
/// * If the results contain only `NotMet` and `Unknown`, the result will be `Unknown`
/// * Only results in `NotMet` if all children are `NotMet`
pub fn or(rules: Vec<Rule>) -> Rule {
    Rule::Or { rules }
}

/// Creates a `Rule` where `n` child `Rule`s must be `Met`
///
/// * If `>= n` are `Met`, the result will be `Met`
/// * If `>= children.len() - n + 1` are `NotMet`, the result will be `NotMet` (No combination of `Met` + `Unknown` can be >= `n`)
/// * If neither of the above are met, the result is `Unknown`
pub fn n_of(n: usize, rules: Vec<Rule>) -> Rule {
    Rule::NumberOf { n, rules }
}

/// Creates a rule for string comparison
pub fn string_equals(description: &str, field: &str, val: &str) -> Rule {
    Rule::Rule {
        desc: description.into(),
        field: field.into(),
        constraint: Constraint::StringEquals(val.into()),
    }
}

/// Creates a rule for int comparison.
///
///If the checked value is not convertible to an integer, the result is `NotMet`
pub fn int_equals(description: &str, field: &str, val: i32) -> Rule {
    Rule::Rule {
        desc: description.into(),
        field: field.into(),
        constraint: Constraint::IntEquals(val),
    }
}

/// Creates a rule for int range comparison with the interval `[start, end]`.
///
/// If the checked value is not convertible to an integer, the result is `NotMet`
pub fn int_range(description: &str, field: &str, start: i32, end: i32) -> Rule {
    Rule::Rule {
        desc: description.into(),
        field: field.into(),
        constraint: Constraint::IntRange(start, end),
    }
}

/// Creates a rule for boolean comparison.
///
/// Only input values of `"true"` (case-insensitive) are considered `true`, all others are considered `false`
pub fn boolean(description: &str, field: &str, val: bool) -> Rule {
    Rule::Rule {
        desc: description.into(),
        field: field.into(),
        constraint: Constraint::Boolean(val),
    }
}
