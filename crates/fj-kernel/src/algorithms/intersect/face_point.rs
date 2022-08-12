//! Intersection between faces and points in 2D

use fj_math::Point;

use crate::objects::Face;

use super::{
    ray_segment::RaySegmentIntersection, HorizontalRayToTheRight, Intersect,
};

impl Intersect for (&Face, &Point<2>) {
    type Intersection = FacePointIntersection;

    fn intersect(self) -> Option<Self::Intersection> {
        let (face, point) = self;

        let ray = HorizontalRayToTheRight { origin: *point };

        let mut num_hits = 0;

        for cycle in face.all_cycles() {
            // We need to properly detect the ray passing the boundary at the
            // "seam" of the polygon, i.e. the vertex between the last and the
            // first segment. The logic in the loop properly takes care of that,
            // as long as we initialize the `previous_hit` variable with the
            // result of the last segment.
            let mut previous_hit = cycle
                .edges()
                .last()
                .copied()
                .and_then(|edge| (&ray, &edge).intersect());

            for edge in cycle.edges() {
                let hit = (&ray, edge).intersect();

                let count_hit = match (hit, previous_hit) {
                    (
                        Some(
                            RaySegmentIntersection::RayStartsOnSegment
                            | RaySegmentIntersection::RayStartsOnOnFirstVertex
                            | RaySegmentIntersection::RayStartsOnSecondVertex,
                        ),
                        _,
                    ) => {
                        // If the ray starts on the boundary of the face,
                        // there's nothing to else check. By the definition of
                        // this intersection test, the face contains the point.
                        return Some(FacePointIntersection::FaceContainsPoint);
                    }
                    (Some(RaySegmentIntersection::RayHitsSegment), _) => {
                        // We're hitting a segment right-on. Clear case.
                        true
                    }
                    (
                        Some(RaySegmentIntersection::RayHitsUpperVertex),
                        Some(RaySegmentIntersection::RayHitsLowerVertex),
                    )
                    | (
                        Some(RaySegmentIntersection::RayHitsLowerVertex),
                        Some(RaySegmentIntersection::RayHitsUpperVertex),
                    ) => {
                        // If we're hitting a vertex, only count it if we've hit
                        // the other kind of vertex right before.
                        //
                        // That means, we're passing through the polygon
                        // boundary at where two edges touch. Depending on the
                        // order in which edges are checked, we're seeing this
                        // as a hit to one edge's lower/upper vertex, then the
                        // other edge's opposite vertex.
                        //
                        // If we're seeing two of the same vertices in a row,
                        // we're not actually passing through the polygon
                        // boundary. Then we're just touching a vertex without
                        // passing through anything.
                        true
                    }
                    (Some(RaySegmentIntersection::RayHitsSegmentAndAreParallel), _) => {
                        // A parallel edge must be completely ignored. Its
                        // presence won't change anything, so we can treat it as
                        // if it wasn't there, and its neighbors were connected
                        // to each other.
                        continue;
                    }
                    _ => {
                        // Any other case is not a valid hit.
                        false
                    }
                };

                if count_hit {
                    num_hits += 1;
                }

                previous_hit = hit;
            }
        }

        if num_hits % 2 == 1 {
            Some(FacePointIntersection::FaceContainsPoint)
        } else {
            None
        }
    }
}

/// The intersection between a face and a point
#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum FacePointIntersection {
    /// The point is inside of the face
    FaceContainsPoint,
}

#[cfg(test)]
mod tests {
    use fj_math::Point;

    use crate::{
        algorithms::intersect::{face_point::FacePointIntersection, Intersect},
        objects::{Face, Surface},
    };

    #[test]
    fn point_is_outside_face() {
        let face = Face::build(Surface::xy_plane())
            .polygon_from_points([[0., 0.], [1., 1.], [0., 2.]])
            .into_face();
        let point = Point::from([2., 1.]);

        let intersection = (&face, &point).intersect();
        assert_eq!(intersection, None);
    }

    #[test]
    fn ray_hits_vertex_while_passing_outside() {
        let face = Face::build(Surface::xy_plane())
            .polygon_from_points([[0., 0.], [2., 1.], [0., 2.]])
            .into_face();
        let point = Point::from([1., 1.]);

        let intersection = (&face, &point).intersect();
        assert_eq!(
            intersection,
            Some(FacePointIntersection::FaceContainsPoint)
        );
    }

    #[test]
    fn ray_hits_vertex_at_cycle_seam() {
        let face = Face::build(Surface::xy_plane())
            .polygon_from_points([[4., 2.], [0., 4.], [0., 0.]])
            .with_hole([[1., 1.], [2., 1.], [1., 3.]])
            .into_face();
        let point = Point::from([1., 2.]);

        let intersection = (&face, &point).intersect();
        assert_eq!(
            intersection,
            Some(FacePointIntersection::FaceContainsPoint)
        );
    }

    #[test]
    fn ray_hits_vertex_while_staying_inside() {
        let face = Face::build(Surface::xy_plane())
            .polygon_from_points([[0., 0.], [2., 1.], [3., 0.], [3., 4.]])
            .into_face();
        let point = Point::from([1., 1.]);

        let intersection = (&face, &point).intersect();
        assert_eq!(
            intersection,
            Some(FacePointIntersection::FaceContainsPoint)
        );
    }

    #[test]
    fn ray_hits_parallel_edge_and_leaves_face_at_vertex() {
        let face = Face::build(Surface::xy_plane())
            .polygon_from_points([[0., 0.], [2., 1.], [3., 1.], [0., 2.]])
            .into_face();
        let point = Point::from([1., 1.]);

        let intersection = (&face, &point).intersect();
        assert_eq!(
            intersection,
            Some(FacePointIntersection::FaceContainsPoint)
        );
    }

    #[test]
    fn ray_hits_parallel_edge_and_does_not_leave_face_there() {
        let face = Face::build(Surface::xy_plane())
            .polygon_from_points([
                [0., 0.],
                [2., 1.],
                [3., 1.],
                [4., 0.],
                [4., 5.],
            ])
            .into_face();
        let point = Point::from([1., 1.]);

        let intersection = (&face, &point).intersect();
        assert_eq!(
            intersection,
            Some(FacePointIntersection::FaceContainsPoint)
        );
    }
}
