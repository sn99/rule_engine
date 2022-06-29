use crate::constraint::Constraint;
use crate::rule::Rule;

pub mod constraint;
pub mod rule;
pub mod status;
pub use crate::status::Status;

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
pub fn int_equals(description: &str, field: &str, val: isize) -> Rule {
    Rule::Rule {
        desc: description.into(),
        field: field.into(),
        constraint: Constraint::IntEquals(val),
    }
}

/// Creates a rule for int range comparison with the interval `[start, end]`.
///
/// If the checked value is not convertible to an integer, the result is `NotMet`
pub fn int_range(description: &str, field: &str, start: isize, end: isize) -> Rule {
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
