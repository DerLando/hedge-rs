use crate::data::{Index, Position, Normal, INVALID_COMPONENT_INDEX};
use crate::traits::IsValid;
use std::collections::HashSet;
use nalgebra as na;

pub const MAX_EDGES: usize = 8; // maximum number of sides for a face

pub type Tag = u32;

/// Whether or not a cell is current or 'removed'
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub enum ElementStatus {
    ACTIVE,
    INACTIVE,
}

////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub struct Face {
    pub tag: Tag,
    pub status: ElementStatus,
    pub vertices: [Vertex; MAX_EDGES],
    pub adjacent: [Index; MAX_EDGES],
}

#[derive(Debug)]
pub struct Vertex {
    pub position: Index,
    pub normal: Normal,
    pub edges: HashSet<Index>,
}

////////////////////////////////////////////////////////////////

impl Default for Face {
    fn default() -> Self {
        Face {
            vertices: [Default::default(); MAX_EDGES],
            adjacent: [INVALID_COMPONENT_INDEX; MAX_EDGES],
            ..Default::default()
        }
    }
}

pub enum MeshElement {
    Face,
    Vertex,
}
