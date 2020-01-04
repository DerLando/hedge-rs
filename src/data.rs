use std::hash::{Hash, Hasher};
use ultraviolet as uv;
use crate::traits::{Storable, IsValid};

pub const INVALID_INDEX: Index = std::u32::MAX;

/// Maximum number of sides for a face.
pub const MAX_EDGES: usize = 8;

/// Generation tracks when the internal buffers have been compacted.
/// If you use a handle from a previous generation it will be refused.
pub type Generation = u32;
pub type Index = u32;
pub type Tag = u32;
pub type Position = uv::Vec3;
pub type Normal = uv::Vec3;
pub type Color = uv::Vec4;
pub type TexCoord = uv::Vec2;

#[derive(Debug, Copy, Clone, Eq, PartialOrd, PartialEq, Hash)]
pub struct VertexID {
    pub fidx: Index,
    pub vidx: Index,
}

impl Default for VertexID {
    fn default() -> Self {
        VertexID {
            fidx: INVALID_INDEX,
            vidx: INVALID_INDEX,
        }
    }
}

#[derive(Debug, Default)]
pub struct Face {
    pub tag: Tag,
    pub num_vertices: u8,
    pub vertices: [Vertex; MAX_EDGES],
    pub adjacent: [VertexID; MAX_EDGES],
}

impl Storable for Face {
    fn make_handle(index: u32, generation: u32) -> Handle {
        Handle::new(ComponentID::Face(index), generation)
    }
}

impl IsValid for Face {
    fn is_valid(&self) -> bool {
        unimplemented!()
    }
}

#[derive(Debug, Default)]
pub struct Vertex {
    pub point: Index,
    pub normal: Normal,
    pub color: Color,
    pub tex_coord: TexCoord,
}

#[derive(Debug, Default)]
pub struct Point {
    pub position: Position,
}

impl From<Position> for Point {
    fn from(position: Position) -> Self {
        Point {
            position,
        }
    }
}

impl Storable for Point {
    fn make_handle(index: Index, generation: u32) -> Handle {
        Handle::new(ComponentID::Point(index), generation)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialOrd, PartialEq)]
pub enum ComponentID {
    Invalid,
    Face(Index),
    Vertex(VertexID),
    Point(Index),
}

impl Hash for ComponentID {
    fn hash<H: Hasher>(&self, state: &mut H) {
        use ComponentID::*;
        match self {
            Invalid => 0.hash(state),
            Face(fidx) => fidx.hash(state),
            Vertex(vid) => vid.hash(state),
            Point(pid) => pid.hash(state),
        }
    }
}

impl Default for ComponentID {
    fn default() -> Self {
        ComponentID::Invalid
    }
}

#[derive(Debug, Default, Copy, Clone, Eq, PartialOrd, PartialEq)]
pub struct Handle {
    id: ComponentID,
    generation: Option<Generation>,
}

impl Handle {
    pub fn for_id(id: ComponentID) -> Self {
        Handle {
            id,
            generation: None,
        }
    }

    pub fn new(id: ComponentID, generation: Generation) -> Self {
        Handle {
            id,
            generation: Some(generation),
        }
    }
}

impl From<ComponentID> for Handle {
    fn from(index: ComponentID) -> Self {
        Handle::for_id(index)
    }
}

impl Hash for Handle {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl IsValid for Handle {
    fn is_valid(&self) -> bool {
        self.id != ComponentID::Invalid
    }
}
