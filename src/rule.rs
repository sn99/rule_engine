use crate::status::Status;
use crate::Constraint;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

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
