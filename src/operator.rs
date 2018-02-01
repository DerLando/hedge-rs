//! Operator interface and a set of initial implementations.

use std::collections::VecDeque;
use std::cell::{RefCell, RefMut, Cell};
use failure::Error;
use ordermap;
use cgmath;
use super::utils;
use super::Mesh;
use super::core::*;
use super::query::{Selection, SelectionSet};
use super::function_sets::*;

#[derive(Debug, Fail)]
pub enum OperatorError {
    #[fail(display = "invalid arguments provided to operator.")]
    InvalidArguments,
}

///////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub enum OperatorData {
    Float(f64),
    Int(i64),
    Uint(u64),
    Vector2(cgmath::Vector2<f64>),
    Vector3(cgmath::Vector3<f64>),
    Vector4(cgmath::Vector4<f64>),
    Point(PointIndex),
    Vertex(VertexIndex),
    Edge(EdgeIndex),
    Face(FaceIndex),
    PointCloud(Vec<Point>), // <seinfeld>really?</seinfeld> :shrug:
}

pub struct OperatorContext {
    args: RefCell<VecDeque<OperatorData>>,
    selection: RefCell<SelectionSet>,
}

impl OperatorContext {
    pub fn new() -> OperatorContext {
        OperatorContext {
            args: RefCell::new(VecDeque::new()),
            selection: RefCell::new(SelectionSet::new()),
        }
    }

    pub fn reset(&self) {
        self.args.borrow_mut().clear();
        self.selection.borrow_mut().clear();
    }

    pub fn selection(&self) -> RefMut<SelectionSet> {
        self.selection.borrow_mut()
    }

    pub fn select<I: Into<Selection>>(&self, item: I) {
        self.selection.borrow_mut().insert(item.into());
    }

    pub fn extend_selection(&self, set: SelectionSet) {
        self.selection.borrow_mut().extend(set);
    }

    pub fn enqueue_arg(&mut self, arg: OperatorData) {
        self.args.borrow_mut().push_back(arg);
    }

    pub fn dequeue_arg(&mut self) -> Option<OperatorData> {
        self.args.borrow_mut().pop_front()
    }
}

pub type OperatorResult = Result<OperatorData, Error>;

pub trait Operator {
    fn cook(
        &mut self,
        mesh: &mut Mesh,
        context: &mut OperatorContext
    ) -> OperatorResult;
}

///////////////////////////////////////////////////////////////////////////////

pub struct PolyAppend {}

impl PolyAppend {
    pub fn new() -> PolyAppend {
        PolyAppend{}
    }

    pub fn from_point_slice(mesh: &mut Mesh, points: &[PointIndex]) -> FaceIndex {
        unimplemented!()
    }

    pub fn from_vertex_slice(mesh: &mut Mesh, verts: &[VertexIndex]) -> FaceIndex {
        unimplemented!()
    }

    pub fn from_edge_slice(mesh: &mut Mesh, edges: &[EdgeIndex]) -> FaceIndex {
        unimplemented!()
    }
}

impl Operator for PolyAppend {
    fn cook(&mut self, mesh: &mut Mesh, context: &mut OperatorContext) -> OperatorResult {
        unimplemented!()
    }
}
