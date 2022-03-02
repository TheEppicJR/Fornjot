pub mod edges;
pub mod faces;
pub mod vertices;

use kiddo::KdTree;

use crate::math::{Point, Scalar};

use self::{edges::Edges, faces::Faces, vertices::Vertices};

/// The boundary representation of a shape
///
/// # Implementation note
///
/// The goal for `Shape` is to enforce full self-consistency, through the API it
/// provides. Steps have been made in that direction, but right now, the API is
/// still full of holes, forcing callers to just be careful for the time being.
pub struct Shape {
    vertices: VerticesInner,

    pub edges: Edges,
    pub faces: Faces,
}

impl Shape {
    /// Construct a new shape
    pub fn new() -> Self {
        Self {
            vertices: VerticesInner::new(),
            edges: Edges { cycles: Vec::new() },
            faces: Faces(Vec::new()),
        }
    }

    /// Access and modify the shape's vertices
    pub fn vertices(&mut self) -> Vertices {
        Vertices {
            vertices: &mut self.vertices,
        }
    }
}

type VerticesInner = KdTree<Scalar, Point<3>, 3>;
