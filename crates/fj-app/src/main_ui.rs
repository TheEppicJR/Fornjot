use eframe::{egui::containers::CentralPanel, CreationContext};
use serde::{Deserialize, Serialize};

use std::{io, mem::size_of};

use crate::editor::code_editor::CodeEditor;
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

#[derive(Serialize, Deserialize, Debug)]
pub struct FornjotConfig {
    pub dark_mode: bool,
    pub api_key: String,
}

#[derive(Default)]
pub struct EguiOptionsState {
    pub show_trace: bool,
    pub show_layout_debug_on_hover: bool,
    pub show_debug_text_example: bool,
    pub show_original_ui: bool,
    pub show_settings_ui: bool,
    pub show_inspection_ui: bool,
}

impl Default for FornjotConfig {
    fn default() -> Self {
        Self {
            dark_mode: Default::default(),
            api_key: String::new(),
        }
    }
}

fn get_bbox_size_text(aabb: &Aabb<3>) -> String {
    /* Render size of model bounding box */
    let bbsize = aabb.size().components;
    let info = format!(
        "Model bounding box size:\n{:0.1} {:0.1} {:0.1}",
        bbsize[0].into_f32(),
        bbsize[1].into_f32(),
        bbsize[2].into_f32()
    );
    info
}
#[derive(Default)]
pub struct Fornjot {
    pub config: FornjotConfig,
    pub api_key_initialized: bool,
    pub toggle_config: bool,
    pub toggle_about: bool,
    pub code_edit: CodeEditor,

    // surface: wgpu::Surface,
    // device: wgpu::Device,
    // queue: wgpu::Queue,

    // surface_config: wgpu::SurfaceConfiguration,
    // depth_view: wgpu::TextureView,

    // uniform_buffer: wgpu::Buffer,
    // bind_group: wgpu::BindGroup,
    // geometries: Geometries,
    // pipelines: Pipelines,
    pub egui_context: egui::Context,

    // egui_rpass: egui_wgpu::renderer::RenderPass,
    pub egui_options: EguiOptionsState,
}

impl Fornjot {
    pub fn init(mut self, cc: &CreationContext) -> Self {
        self
    }

    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self::default()
    }

    pub(crate) fn render_model_panel(
        &mut self,
        ctx: &egui::Context,
        frame: &mut eframe::Frame,
    ) {
        // define a TopBottomPanel widget
        CentralPanel::default().show(ctx, |ui| {});
    }

    pub(crate) fn render_side_panel(
        &mut self,
        ctx: &egui::Context,
        frame: &mut eframe::Frame,
    ) {
        // A simple UI
        egui::SidePanel::left("fj-left-panel").show(&self.egui_context, |ui| {
            //

            ui.add_space(16.0);

            ui.group(|ui| {
                // ui.checkbox(&mut config.draw_model, "Render model")
                //     .on_hover_text_at_pointer("Toggle with 1");
                // ui.checkbox(&mut config.draw_mesh, "Render mesh")
                //     .on_hover_text_at_pointer("Toggle with 2");
                // ui.checkbox(&mut config.draw_debug, "Render debug")
                //     .on_hover_text_at_pointer("Toggle with 3");
                ui.checkbox(
                    &mut self.egui_options.show_original_ui,
                    "Render original UI",
                );
                ui.add_space(16.0);
                // ui.strong(get_bbox_size_text(&self.geometries.aabb));
            });

            ui.add_space(16.0);

            {
                ui.group(|ui| {
                    ui.checkbox(
                        &mut self.egui_options.show_settings_ui,
                        "Show egui settings UI",
                    );
                    if self.egui_options.show_settings_ui {
                        self.egui_context.settings_ui(ui);
                    }
                });

                ui.add_space(16.0);

                ui.group(|ui| {
                    ui.checkbox(
                        &mut self.egui_options.show_inspection_ui,
                        "Show egui inspection UI",
                    );
                    if self.egui_options.show_inspection_ui {
                        ui.indent("indent-inspection-ui", |ui| {
                            self.egui_context.inspection_ui(ui);
                        });
                    }
                });
            }

            ui.add_space(16.0);

            {
                //
                // Originally this was only meant to be a simple demonstration
                // of the `egui` `trace!()` macro...
                //
                // ...but it seems the trace feature can't be enabled
                // separately from the layout debug feature, which all
                // gets a bit messy...
                //
                // ...so, this instead shows one possible way to implement
                // "trace only" style debug text on hover.
                //
                ui.group(|ui| {
                    let label_text = format!(
                        "Show debug text demo.{}",
                        if self.egui_options.show_debug_text_example {
                            " (Hover me.)"
                        } else {
                            ""
                        }
                    );

                    ui.style_mut().wrap = Some(false);

                    // if ui
                    //     .checkbox(
                    //         &mut self.egui_options.show_debug_text_example,
                    //         label_text,
                    //     )
                    //     .hovered()
                    // {
                    //     if self.egui_options.show_debug_text_example {
                    //         let hover_pos = ui
                    //             .input()
                    //             .pointer
                    //             .hover_pos()
                    //             .unwrap_or_default();
                    //         ui.painter().debug_text(
                    //             hover_pos,
                    //             egui::Align2::LEFT_TOP,
                    //             egui::Color32::DEBUG_COLOR,
                    //             format!("{:#?}", &config),
                    //         );
                    //     }
                    // }
                });
            }

            ui.add_space(16.0);

            {
                //
                // Demonstration of the `egui` layout debug functionality.
                //
                ui.group(|ui| {
                    //

                    if ui
                        .checkbox(
                            &mut self.egui_options.show_layout_debug_on_hover,
                            "Show layout debug on hover.",
                        )
                        .changed()
                    {
                        ui.ctx().set_debug_on_hover(
                            self.egui_options.show_layout_debug_on_hover,
                        );
                    }

                    // ui.scope(|ui| {
                    //     if self.egui_options.show_trace {
                    //         egui::trace!(ui, format!("{:?}", &config));
                    //     }
                    // });

                    ui.indent("indent-show-trace", |ui| {
                        ui.set_enabled(
                            self.egui_options.show_layout_debug_on_hover,
                        );

                        ui.checkbox(
                            &mut self.egui_options.show_trace,
                            "Also show egui trace.",
                        );

                        //
                    });
                });
            }

            ui.add_space(16.0);
        });
    }
}
