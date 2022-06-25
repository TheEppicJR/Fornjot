use Axis::*;
use Orentation::*;
use Plane::*;
use Point::*;

struct WCS {
    orentation: Orentation<f64>,
    origin: Point<3>,
    name: String,
    x_axis: Axis<3>,
    y_axis: Axis<3>,
    z_axis: Axis<3>,
    x_plane: Plane<3>,
    y_plane: Plane<3>,
    z_plane: Plane<3>,
}

impl WCS {
    fn new(
        orentation: Orentation<f64>,
        origin: Point<3>,
        name: String,
    ) -> Self {
        Self {
            orentation,
            origin,
            name,
        }
    }
}
