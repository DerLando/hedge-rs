//! Operator interface and a set of initial implementations.

use std::collections::VecDeque;
use failure::Error;
use super::utils;
use super::Mesh;
use super::core::*;
use super::function_sets::*;

#[derive(Debug, Fail)]
pub enum OperatorError {
    #[fail(display = "invalid arguments provided to operator.")]
    InvalidArguments,
}

///////////////////////////////////////////////////////////////////////////////

pub struct OperatorContext<Input> {
    input: VecDeque<Input>,
}

impl<Input> OperatorContext<Input> {
    pub fn enqueue(&mut self, next_input: Input) {
        self.input.push_back(next_input);
    }
    pub fn dequeue(&mut self) -> Option<Input> {
        self.input.pop_front()
    }
}

pub trait Operator {
    type Input;
    type Output;
    fn cook(&mut self, mesh: &mut Mesh, context: &mut OperatorContext<Self::Input>) -> Result<Self::Output, Error>;
}

///////////////////////////////////////////////////////////////////////////////

pub struct Append {}
