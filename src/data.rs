use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use ultraviolet as uv;

/// Handles with this generation value will only have their index considered.
pub const IGNORED_GENERATION: Generation = 0;

/// Maximum number of sides for a face.
pub const MAX_EDGES: usize = 8;

/// Generation tracks when the internal buffers have been compacted.
/// If you use a handle from a previous generation it will be refused.
pub type Generation = u32;
pub type Tag = u32;
pub type Position = uv::Vec3;
pub type Normal = uv::Vec3;
pub type Color = uv::Vec4;
pub type TexCoord = uv::Vec2;

#[derive(Debug, Clone, Copy, Eq, PartialOrd, PartialEq)]
pub enum Index {
    Invalid,
    Face(u32),
    SubComponent(u32, u32),
}

impl Hash for Index {
    fn hash<H: Hasher>(&self, state: &mut H) {
        use Index::*;
        match self {
            Invalid => 0.hash(state),
            Face(fidx) => fidx.hash(state),
            SubComponent(fidx, cidx) => {
                fidx.hash(state);
                cidx.hash(state);
            },
        }
    }
}

impl Default for Index {
    fn default() -> Self {
        Index::Invalid
    }
}

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

#[derive(Debug, Default)]
pub struct Vertex {
    pub position: Index,
    pub normal: Normal,
    pub edges: HashSet<Index>,
}

////////////////////////////////////////////////////////////////

impl Default for Face {
    fn default() -> Self {
        Face {
            // I don't want Vertex to impl Copy so we can't use [foo; n] syntax
            vertices: [
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
            ],
            adjacent: [Default::default(); MAX_EDGES],
            ..Default::default()
        }
    }
}
