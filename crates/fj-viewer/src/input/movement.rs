use fj_math::{Point, Scalar, Transform, Vector};

use crate::{
    camera::{Camera, FocusPoint},
    screen::NormalizedPosition,
};

pub struct Movement;

impl Movement {
    pub fn apply(
        &mut self,
        previous: NormalizedPosition,
        current: NormalizedPosition,
        focus_point: FocusPoint,
        camera: &mut Camera,
    ) {
        let previous = camera.cursor_to_model_space(previous);
        let cursor = camera.cursor_to_model_space(current);

        let d1 = Point::distance(&camera.position(), &cursor);
        let d2 = Point::distance(&camera.position(), &focus_point.0);

        let diff = (cursor - previous) * d2 / d1;
        let offset = camera.camera_to_model().transform_vector(&diff);

        camera.translation = camera.translation
            * Transform::translation(Vector::from([
                offset.x,
                offset.y,
                Scalar::ZERO,
            ]));
    }
}
