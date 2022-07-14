use fj_interop::debug::DebugInfo;
use fj_kernel::{
    algorithms::{sweep, Tolerance},
    objects::{Face, Sketch},
    validation::{validate, Validated, ValidationConfig, ValidationError},
};
use fj_math::{Aabb, Vector};

use super::Shape;

impl Shape for fj::Sweep {
    type Brep = Vec<Face>;

    fn compute_brep(
        &self,
        config: &ValidationConfig,
        tolerance: Tolerance,
        debug_info: &mut DebugInfo,
    ) -> Result<Validated<Self::Brep>, ValidationError> {
        let sketch =
            self.shape().compute_brep(config, tolerance, debug_info)?;
        let path = Vector::from(self.path());
        let color = self.shape().color();

        let solid = sweep(
            Sketch::from_faces(sketch.into_inner()),
            path,
            tolerance,
            color,
        );

        validate(solid.into_faces(), config)
    }

    fn bounding_volume(&self) -> Aabb<3> {
        self.shape()
            .bounding_volume()
            .merged(&Aabb::<3>::from_points(
                self.shape()
                    .bounding_volume()
                    .vertices()
                    .map(|v| v + self.path()),
            ))
    }
}
