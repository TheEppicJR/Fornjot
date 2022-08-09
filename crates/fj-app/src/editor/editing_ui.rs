use bevy::prelude::*;
use egui_extras::{Size, StripBuilder};
use fj::Shape;
use fj_host::{Model, Parameters};
use fj_interop::status_report::StatusReport;
use std::path::PathBuf;

pub struct EditingUI {
    language: String,
    project_code: String,
    project_path: PathBuf,
    name: String,
    // model: Model,
    // shape: Shape,
    left: f32,
    top: f32,
    right: f32,
    bottom: f32,
}

impl Default for EditingUI {
    fn default() -> Self {
        let code = "use fj::syntax::*;

#[fj::model]
pub fn model(
    #[param(default = 1.0, min = inner * 1.01)] outer: f64,
    #[param(default = 0.5, max = outer * 0.99)] inner: f64,
    #[param(default = 1.0)] height: f64,
) -> fj::Shape {
    let outer_edge = fj::Sketch::from_circle(fj::Circle::from_radius(outer));
    let inner_edge = fj::Sketch::from_circle(fj::Circle::from_radius(inner));

    let footprint = outer_edge.difference(&inner_edge);
    let spacer = footprint.sweep([0., 0., height]);

    spacer.into()
}"
        .into();
        let path = PathBuf::new();
        // let mut model = Model::from_path(path).with_context(|| {
        //     format!("Failed to load model: {}", path.display())
        // })?;
        let mut status = StatusReport::new();
        // let mut shape = model.load_once(Parameters::empty, &mut status);
        Self {
            project_code: code,
            project_path: path,
            // model,
            name: "New Project".into(),
            language: "rs".into(),
            // shape,
            right: 0.0,
            left: 0.0,
            top: 0.0,
            bottom: 0.0,
        }
    }
}

impl EditingUI {
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        let mut theme =
            super::code_editor::syntax_highlighting::CodeTheme::from_memory(
                ui.ctx(),
            );

        let mut layouter = |ui: &egui::Ui, string: &str, wrap_width: f32| {
            let mut layout_job =
                super::code_editor::syntax_highlighting::highlight(
                    ui.ctx(),
                    &theme,
                    string,
                    &self.language,
                );
            layout_job.wrap.max_width = wrap_width;
            ui.fonts().layout_job(layout_job)
        };

        self.top = egui::TopBottomPanel::top("top_panel")
            .resizable(true)
            .min_height(32.0)
            .show_inside(ui, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.heading("Expandable Upper Panel");
                    });
                });
            })
            .response
            .rect
            .height();

        self.left = egui::SidePanel::left("left_panel")
            .resizable(true)
            .default_width(150.0)
            .width_range(80.0..=200.0)
            .show_inside(ui, |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading("Left Panel");
                });
            })
            .response
            .rect
            .width();

        self.right = egui::SidePanel::right("right_panel")
            .resizable(true)
            .default_width(150.0)
            .width_range(80.0..=200.0)
            .show_inside(ui, |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading("Right Panel");
                });
            })
            .response
            .rect
            .width();

        self.bottom = egui::TopBottomPanel::bottom("bottom_panel")
            .resizable(true)
            .min_height(0.0)
            .show_inside(ui, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.add(
                        egui::TextEdit::multiline(&mut self.project_code)
                            .font(egui::TextStyle::Monospace) // for cursor height
                            .code_editor()
                            .lock_focus(true)
                            .desired_width(f32::INFINITY)
                            .layouter(&mut layouter)
                            .desired_rows(100),
                    );
                });
            })
            .response
            .rect
            .height();

        // egui::CentralPanel::default().show_inside(ui, |ui| {
        //     ui.vertical_centered(|ui| {
        //         ui.heading("Central Panel");
        //     });
        // });
    }

    // fn render_model(&mut self) {
    //     if let Some(new_shape) = watcher.receive(&mut status) {
    //         match shape_processor.process(&new_shape) {
    //             Ok(new_shape) => {
    //                 renderer.update_geometry(
    //                     (&new_shape.mesh).into(),
    //                     (&new_shape.debug_info).into(),
    //                     new_shape.aabb,
    //                 );

    //                 if camera.is_none() {
    //                     camera = Some(Camera::new(&new_shape.aabb));
    //                 }

    //                 shape = Some(new_shape);
    //             }
    //             Err(err) => {
    //                 // Can be cleaned up, once `Report` is stable:
    //                 // https://doc.rust-lang.org/std/error/struct.Report.html

    //                 println!("Shape processing error: {}", err);

    //                 let mut current_err = &err as &dyn error::Error;
    //                 while let Some(err) = current_err.source() {
    //                     println!();
    //                     println!("Caused by:");
    //                     println!("    {}", err);

    //                     current_err = err;
    //                 }
    //             }
    //         }
    //     }
    // }
}
