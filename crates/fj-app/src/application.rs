use crate::editor::code_editor::code_editor::CodeEditor;
use crate::editor::window::EditWindow;
use crate::main_ui::{Fornjot, FornjotConfig};
use eframe::{
    egui::containers::{CentralPanel, SidePanel, TopBottomPanel},
    egui::Vec2,
    run_native, App, CreationContext, NativeOptions,
};

impl App for Fornjot {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        SidePanel::left("demo_ui").show(ctx, |ui| {
            ui.label("hi");
            ui.add_space(16.0);
            self.code_edit
                .show(ctx, &mut ui.button("Open Code Editor").clicked());
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

                    if ui
                        .checkbox(
                            &mut self.egui_options.show_debug_text_example,
                            label_text,
                        )
                        .hovered()
                    {
                        if self.egui_options.show_debug_text_example {
                            let hover_pos = ui
                                .input()
                                .pointer
                                .hover_pos()
                                .unwrap_or_default();
                            ui.painter().debug_text(
                                hover_pos,
                                egui::Align2::LEFT_TOP,
                                egui::Color32::DEBUG_COLOR,
                                format!("{:#?}", &self.config),
                            );
                        }
                    }
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

                    ui.scope(|ui| {
                        if self.egui_options.show_trace {
                            egui::trace!(ui, format!("{:?}", &self.config));
                        }
                    });

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
        // self.render_side_panel(ctx, frame);
        self.render_model_panel(ctx, frame);
    }
}

pub fn run_app() {
    // let app = Fornjot::default();
    let mut win_option = NativeOptions::default();
    // win_option.initial_window_size = Some(Vec2::new(540., 960.));
    run_native(
        "Fornjott",
        win_option,
        Box::new(|cc| Box::new(Fornjot::new(cc))),
    );
}
