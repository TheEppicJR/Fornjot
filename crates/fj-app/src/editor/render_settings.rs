use bevy::prelude::*;
use fj_host::Parameters;
use fj_kernel::algorithms::Tolerance;

#[derive(Component)]
pub struct ViweportSettings {
    control_scheme: ViewportControlScheme,
}

pub enum ViewportControlScheme {
    Solidworks,
    Fusion,
    Inventor,
    Custom,
}

#[derive(Component)]
pub struct RenderSettings {
    tolerance: f64,
    params: Parameters,
}

impl Default for RenderSettings {
    fn default() -> Self {
        Self {
            tolerance: 0.01,
            params: Parameters::empty(),
        }
    }
}
