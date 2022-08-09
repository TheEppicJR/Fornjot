use crate::editor::code_editor::syntax_highlighting::CodeTheme;
use bevy::prelude::*;

use eframe::{egui::containers::CentralPanel, CreationContext};
use serde::{Deserialize, Serialize};

use std::{io, mem::size_of};

use fj_math::{Aabb, Point};
use thiserror::Error;
use tracing::debug;
use wgpu::util::DeviceExt as _;
use wgpu_glyph::ab_glyph::InvalidFont;

use fj_viewer::{
    camera::Camera,
    screen::{Screen, Size},
    window::Window,
};

use fj_viewer::graphics::{
    config_ui::ConfigUi, draw_config::DrawConfig, drawables::Drawables,
    geometries::Geometries, pipelines::Pipelines, transform::Transform,
    uniforms::Uniforms, vertices::Vertices, DEPTH_FORMAT,
};

#[derive(Component, Default)]
pub struct AppSetting;

pub fn ui(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.set_height(0.0);
        ui.label("Application Settings");
    });
    ui.add_space(16.0);
    let mut theme = CodeTheme::from_memory(ui.ctx());
    ui.label("Code Theme");
    ui.group(|ui| {
        theme.ui(ui);
        theme.clone().store_in_memory(ui.ctx());
    });
    ui.add_space(16.0);
    ui.label("egui settings");
    ui.group(|ui| {
        let tmp_ui = ui.ctx().clone();
        tmp_ui.settings_ui(ui);
    });
    ui.add_space(16.0);
    ui.label("Other app settings");
}
