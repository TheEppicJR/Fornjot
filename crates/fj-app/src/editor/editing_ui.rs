use bevy::prelude::*;
use fj::Shape;
use fj_host::{Model, Parameters};
use fj_interop::status_report::StatusReport;
use std::path::PathBuf;

pub struct EditingUI {
    language: String,
    project_code: String,
    project_path: PathBuf,
    name: String,
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

        Self {
            project_code: code,
            project_path: path,
            name: "New Project".into(),
            language: "rs".into(),
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

        ui.heading("Expandable Upper Panel");

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
