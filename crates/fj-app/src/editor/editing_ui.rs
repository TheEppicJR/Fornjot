use bevy::prelude::*;
use std::path::PathBuf;

pub struct EditingUI {
    language: String,
    project_code: String,
    file_path: PathBuf,
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
        Self {
            project_code: code,
            file_path: PathBuf::new(),
            name: "New Project".into(),
            language: "rs".into(),
        }
    }
}

impl EditingUI {
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        // let Self { language, code } = self;

        ui.horizontal(|ui| {
            ui.set_height(0.0);
            ui.label("An example of syntax highlighting in a TextEdit.");
        });

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

        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.add(
                egui::TextEdit::multiline(&mut self.project_code)
                    .font(egui::TextStyle::Monospace) // for cursor height
                    .code_editor()
                    .desired_rows(10)
                    .lock_focus(true)
                    .desired_width(f32::INFINITY)
                    .layouter(&mut layouter),
            );
        });
    }
}
