
use std::cell::{Cell, RefCell, Ref, RefMut};
use crate::data::{Generation, Tag, Position, ElementStatus};
use crate::handles::Handle;
use crate::traits::{
    ElementHandle, ElementData,
    Storable, Taggable, IsActive, IsValid
};

/// Trait for accessing Mesh element properties.
#[derive(Debug, Clone)]
pub struct MeshElement<D: ElementData + Default> {
    tag: Cell<Tag>,
    generation: Cell<Generation>,
    status: Cell<ElementStatus>,
    data: RefCell<D>,
}

impl<D: ElementData + Default> Default for MeshElement<D> {
    fn default() -> Self {
        MeshElement {
            tag: Cell::new(0),
            generation: Cell::new(1),
            status: Cell::new(ElementStatus::INACTIVE),
            data: RefCell::default()
        }
    }
}

impl<D: ElementData + Default> MeshElement<D> {
    pub fn with_data(data: D) -> Self {
        MeshElement {
            data: RefCell::new(data),
            ..MeshElement::default()
        }
    }

    pub fn data(&self) -> Ref<D> {
        self.data.borrow()
    }

    pub fn data_mut(&self) -> RefMut<D> {
        self.data.borrow_mut()
    }
}

impl<D: ElementData + Default> Storable for MeshElement<D> {
    fn generation(&self) -> Generation {
        self.generation.get()
    }

    fn set_generation(&self, generation: Generation) {
        self.generation.set(generation);
    }

    fn status(&self) -> ElementStatus {
        self.status.get()
    }

    fn set_status(&self, status: ElementStatus) {
        self.status.set(status);
    }
}

impl<D: ElementData + Default> Taggable for MeshElement<D> {
    fn tag(&self) -> Tag {
        self.tag.get()
    }

    fn set_tag(&self, tag: Tag) {
        self.tag.set(tag);
    }
}

impl<D: ElementData + Default> IsActive for MeshElement<D> {
    fn is_active(&self) -> bool {
        self.status.get() == ElementStatus::ACTIVE
    }
}

////////////////////////////////////////////////////////////////

/// TODO: Documentation
#[derive(Debug, Clone, Default)]
pub struct HalfEdgeData {
    /// The adjacent half-edge
    pub adjacent: HalfEdgeHandle,
    /// The Handle of the next edge in the loop
    pub next: HalfEdgeHandle,
    /// The Handle of the previous edge in the loop
    pub prev: HalfEdgeHandle,
    /// The Handle of the face this edge loop defines
    pub face: FaceHandle,
    /// The Handle of the Vertex for this edge.
    pub vertex: VertexHandle,
}
pub type HalfEdge = MeshElement<HalfEdgeData>;
pub type HalfEdgeHandle = Handle<HalfEdge>;
impl ElementData for HalfEdgeData {}
impl ElementHandle for HalfEdgeHandle {}
impl HalfEdge {
    /// Returns true when this edge has a previous and next edge.
    pub fn is_connected(&self) -> bool {
        let data = self.data();
        data.next.is_valid() && data.prev.is_valid()
    }
}
impl IsValid for HalfEdge {
    /// An Edge is valid when it has a valid twin index, a valid vertex index
    /// and `is_connected`
    fn is_valid(&self) -> bool {
        let data = self.data();
        self.is_active() &&
            data.vertex.is_valid() &&
            data.adjacent.is_valid() &&
            data.next.is_valid() &&
            data.prev.is_valid()
    }
}

/// TODO: Documentation
#[derive(Debug, Clone, Default)]
pub struct VertexData {
    /// Index of the outgoing edge
    pub edge: HalfEdgeHandle,
    /// Index of point this vertex belongs to
    pub point: PointHandle,
}
pub type Vertex = MeshElement<VertexData>;
pub type VertexHandle = Handle<Vertex>;
impl ElementData for VertexData {}
impl ElementHandle for VertexHandle {}
impl Vertex {
    pub fn new(edge: HalfEdgeHandle, point: PointHandle) -> Self {
        Vertex::with_data(VertexData { edge, point })
    }

    pub fn for_edge(edge: HalfEdgeHandle) -> Self {
        Vertex::with_data(VertexData {
            edge,
            ..VertexData::default()
        })
    }

    pub fn at_point(point: PointHandle) -> Self {
        Vertex::with_data(VertexData {
            point,
            ..VertexData::default()
        })
    }
}
impl IsValid for Vertex {
    /// A vertex is considered "valid" as long as it has a valid edge index.
    fn is_valid(&self) -> bool {
        self.is_active() && self.data().edge.is_valid()
    }
}

/// TODO: Documentation
#[derive(Debug, Clone, Default)]
pub struct FaceData {
    /// The "root" of an edge loop that defines this face.
    pub root_edge: HalfEdgeHandle,
}
pub type Face = MeshElement<FaceData>;
pub type FaceHandle = Handle<Face>;
impl ElementData for FaceData {}
impl ElementHandle for FaceHandle {}
impl Face {
    pub fn new(root_edge: HalfEdgeHandle) -> Self {
        Face::with_data(FaceData { root_edge })
    }
}
impl IsValid for Face {
    /// A face is considered "valid" as long as it has an edge index
    /// other than `INVALID_COMPONENT_INDEX`
    fn is_valid(&self) -> bool {
        self.is_active() && self.data().root_edge.is_valid()
    }
}

#[derive(Debug, Clone)]
pub struct PointData {
    pub position: Position,
}
impl PointData {
    pub fn new(position: Position) -> Self {
        PointData { position }
    }
}
impl Default for PointData {
    fn default() -> Self {
        PointData {
            position: [0.0; 3],
        }
    }
}
pub type Point = MeshElement<PointData>;
pub type PointHandle = Handle<Point>;
impl ElementData for PointData {}
impl ElementHandle for PointHandle {}
impl Point {
    pub fn new(position: Position) -> Self {
        Point::with_data(PointData::new(position))
    }

    pub fn from_coords(x: f32, y: f32, z: f32) -> Self {
        Point::with_data(PointData::new([x, y, z]))
    }

    pub fn from_slice(offset: usize, values: &[f32]) -> Self {
        assert!(values.len() >= (offset + 3));
        Point::with_data(PointData::new([
            values[offset],
            values[offset+1],
            values[offset+2],
        ]))
    }
}
impl IsValid for Point {
    fn is_valid(&self) -> bool {
        self.is_active()
    }
}
