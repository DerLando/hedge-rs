//! Operator interface and a set of initial implementations.

use std::collections::VecDeque;
use failure::Error;
use super::utils;
use super::Mesh;
use super::core::*;
use super::query::Selection;
use super::function_sets::*;

#[derive(Debug, Fail)]
pub enum OperatorError {
    #[fail(display = "invalid arguments provided to operator.")] InvalidArguments,
}

///////////////////////////////////////////////////////////////////////////////

pub enum OperatorData {}

pub struct OperatorContext {
    input: VecDeque<OperatorData>,
    selection: Selection,
}

impl OperatorContext {
    pub fn new() -> OperatorContext {
        OperatorContext {
            input: VecDeque::new(),
            selection: Selection::Empty,
        }
    }

    pub fn enqueue(&mut self, next_input: OperatorData) {
        self.input.push_back(next_input);
    }

    pub fn dequeue(&mut self) -> Option<OperatorData> {
        self.input.pop_front()
    }
}

pub trait Operator {
    fn cook(
        &mut self,
        mesh: &mut Mesh,
        context: &mut OperatorContext
    ) -> Result<OperatorData, Error>;
}

///////////////////////////////////////////////////////////////////////////////

pub struct Append {}
