#[derive(Default)]
pub struct WelcomeUI;

impl WelcomeUI {
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.set_height(0.0);
            ui.label("Welcome to the prototype fj-app tab system.");
        });
    }
}
