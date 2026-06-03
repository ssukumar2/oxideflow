//! Tiny SQL-ish filter: parse simple `level = X and contains "Y"` queries.

use crate::parser::LogLine;

#[allow(dead_code)]
pub enum Predicate {
    LevelEq(String),
    Contains(String),
    LineGt(usize),
    LineLt(usize),
    And(Box<Predicate>, Box<Predicate>),
    Or(Box<Predicate>, Box<Predicate>),
}

impl Predicate {
    #[allow(dead_code)]
    pub fn eval(&self, line: &LogLine) -> bool {
        match self {
            Predicate::LevelEq(lvl) => line
                .level
                .as_deref()
                .map(|s| s.eq_ignore_ascii_case(lvl))
                .unwrap_or(false),
            Predicate::Contains(s) => line.raw.contains(s.as_str()),
            Predicate::LineGt(n) => line.line_number > *n,
            Predicate::LineLt(n) => line.line_number < *n,
            Predicate::And(a, b) => a.eval(line) && b.eval(line),
            Predicate::Or(a, b) => a.eval(line) || b.eval(line),
        }
    }
}

#[allow(dead_code)]
pub fn apply<'a>(lines: &'a [LogLine], pred: &Predicate) -> Vec<&'a LogLine> {
    lines.iter().filter(|l| pred.eval(l)).collect()
}
