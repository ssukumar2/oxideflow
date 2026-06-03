//! Compose multiple transforms into a single pipeline.

use crate::parser::LogLine;

#[allow(dead_code)]
type Step = Box<dyn Fn(Vec<LogLine>) -> Vec<LogLine>>;

#[allow(dead_code)]
pub struct Pipeline {
    steps: Vec<Step>,
}

impl Default for Pipeline {
    fn default() -> Self {
        Self::new()
    }
}

impl Pipeline {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self { steps: Vec::new() }
    }

    #[allow(dead_code)]
    pub fn then<F>(mut self, step: F) -> Self
    where
        F: Fn(Vec<LogLine>) -> Vec<LogLine> + 'static,
    {
        self.steps.push(Box::new(step));
        self
    }

    #[allow(dead_code)]
    pub fn run(&self, input: Vec<LogLine>) -> Vec<LogLine> {
        self.steps.iter().fold(input, |acc, step| step(acc))
    }
}
