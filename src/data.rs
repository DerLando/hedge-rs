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

impl VertexID {
    pub fn new(fidx: Index, vidx: Index) -> VertexID {
        VertexID {
            fidx,
            vidx,
        }
    }
}

impl IsValid for VertexID {
    fn is_valid(&self) -> bool {
        self.fidx != INVALID_INDEX && self.vidx != INVALID_INDEX
            && self.vidx as usize <= MAX_EDGES
    }
}

impl Default for VertexID {
    fn default() -> Self {
        VertexID {
            fidx: INVALID_INDEX,
            vidx: INVALID_INDEX,
        }
    }
}

impl From<(Index, Index)> for VertexID {
    fn from(pair: (u32, u32)) -> Self {
        VertexID::new(pair.0, pair.1)
    }
}

#[derive(Debug, Default, Clone)]
pub struct Face {
    pub tag: Tag,
    num_vertices: u8,
    vertices: [Vertex; MAX_EDGES],
    adjacent: [VertexID; MAX_EDGES],
}

impl Face {
    pub fn from_points(points: &[Index]) -> Self {
        if points.len() > MAX_EDGES {
            panic!("Unable to create a face with more than {} points.", MAX_EDGES);
        }
        let mut face = Face {
            num_vertices: points.len() as u8,
            ..Default::default()
        };
        for (vidx, pidx) in points.iter().enumerate() {
            face.vertices[vidx] = Vertex::at_point(*pidx);
        }
        face
    }

    pub fn vertex(&self, index: Index) -> &Vertex {
        let index = (index as u8 % self.num_vertices) as usize;
        &self.vertices[index]
    }

    pub fn vertex_mut(&mut self, index: Index) -> &mut Vertex {
        let index = (index as u8 % self.num_vertices) as usize;
        &mut self.vertices[index]
    }

    pub fn vert_count(&self) -> u8 {
        self.num_vertices
    }
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

#[derive(Debug, Default, Clone)]
pub struct Vertex {
    pub point: Index,
    pub normal: Normal,
    pub color: Color,
    pub tex_coord: TexCoord,
}

impl Vertex {
    pub fn at_point(point: Index) -> Self {
        Vertex {
            point,
            ..Default::default()
        }
    }

    pub fn new(
        point: Index,
        normal: Normal,
        color: Color,
        tex_coord: TexCoord
    ) -> Self {
        Vertex {
            point,
            normal,
            color,
            tex_coord,
        }
    }
}

#[derive(Debug, Default, Clone)]
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

impl From<VertexID> for ComponentID {
    fn from(id: VertexID) -> Self {
        ComponentID::Vertex(id)
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

    pub fn id(&self) -> ComponentID {
        self.id
    }

    pub fn index(&self) -> Index {
        use ComponentID::*;
        match self.id {
            Invalid => INVALID_INDEX,
            Face(index) => index,
            Vertex(id) => id.vidx, // maybe the face instead?
            Point(index) => index,
        }
    }
}

impl From<ComponentID> for Handle {
    fn from(index: ComponentID) -> Self {
        Handle::for_id(index)
    }
}

/// Special case impl for getting vertices easily
impl From<(u32, u32)> for Handle {
    fn from(pair: (u32, u32)) -> Self {
        Handle::for_id(ComponentID::from(VertexID::from(pair)))
    }
}

impl Hash for Handle {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl IsValid for Handle {
    fn is_valid(&self) -> bool {
        use ComponentID::*;
        match self.id {
            Invalid => false,
            Face(fidx) => fidx != INVALID_INDEX,
            Vertex(vert_id) => vert_id.is_valid(),
            Point(pidx) => pidx != INVALID_INDEX,
        }
    }
}
