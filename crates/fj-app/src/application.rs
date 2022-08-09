use crate::ecs::tabs::EditorTabId;
use crate::tabs::tab_ecs_helper;
use crate::tabs::{show, Tree};
use crate::ui_tabs::add_tab;
use crate::ui_tabs::tab_wrapper::EditorUiWrapper;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::{prelude::*, window::PresentMode, winit::WinitSettings};
use bevy_egui::{egui, EguiContext, EguiPlugin, EguiSettings};

#[derive(Default)]
struct UiState {
    is_window_open: bool,
}

pub fn start_app() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(WinitSettings::desktop_app())
        .insert_resource(WindowDescriptor {
            present_mode: PresentMode::Mailbox,
            ..Default::default()
        })
        .init_resource::<UiState>()
        .init_resource::<Tree>()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_startup_system(configure_visuals)
        .add_startup_system(add_tab::add_debug_tab)
        .add_startup_system(add_tab::add_welcome_tab)
        .add_startup_system(add_tab::add_settings_tab)
        .add_startup_system(add_tab::add_editing_tab)
        .add_startup_system(configure_ui_state)
        .add_system(ui_renderer)
        .add_system(tab_ecs_helper::check_for_new_tabs)
        .run();
}

fn configure_visuals(mut egui_ctx: ResMut<EguiContext>) {
    egui_ctx.ctx_mut().set_visuals(egui::Visuals {
        window_rounding: 0.0.into(),
        ..Default::default()
    });
}

fn configure_ui_state(mut ui_state: ResMut<UiState>) {
    ui_state.is_window_open = true;
}

fn ui_renderer(
    mut egui_ctx: ResMut<EguiContext>,
    mut ui_state: ResMut<UiState>,
    mut tab_tree: ResMut<Tree>,
    query: Query<(&mut EditorUiWrapper, Entity), With<EditorTabId>>,
    commands: Commands,
) {
    egui::TopBottomPanel::top("top_panel").show(egui_ctx.ctx_mut(), |ui| {
        // The top panel is often a good place for a menu bar:
        egui::menu::bar(ui, |ui| {
            egui::menu::menu_button(ui, "File", |ui| {
                if ui.button("Quit").clicked() {
                    std::process::exit(0);
                }
                if ui.button("Open").clicked() {
                    std::process::exit(0);
                }
            });
        });
    });

    egui::SidePanel::left("side_panel")
        .default_width(200.0)
        .show(egui_ctx.ctx_mut(), |ui| {
            // show(ui, id, style, tree, context);
        });

    egui::CentralPanel::default().show(egui_ctx.ctx_mut(), |ui| {
        show(
            ui,
            egui::Id::new("some hashable string"),
            &mut *tab_tree,
            &commands,
            query,
        )
    });

    egui::Window::new("Window")
        .vscroll(true)
        .open(&mut ui_state.is_window_open)
        .show(egui_ctx.ctx_mut(), |ui| {
            ui.label("Windows can be moved by dragging them.");
            ui.label("They are automatically sized based on contents.");
            ui.label("You can turn on resizing and scrolling if you like.");
            ui.label("You would normally chose either panels OR windows.");
        });
}
