//! Rendering primitives, routines, and structures.

pub mod config_ui;
pub mod draw_config;
pub mod drawables;
pub mod geometries;
pub mod pipelines;
pub mod renderer;
pub mod shaders;
pub mod transform;
pub mod uniforms;
pub mod vertices;

pub use self::{
    draw_config::DrawConfig,
    renderer::{DrawError, InitError, Renderer},
};

pub const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;
