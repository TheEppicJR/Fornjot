use crate::ecs::tabs::EditorTabId;
use crate::editor::render_settings::RenderSettings;
use crate::sidebar::preview_files_being_dropped;
use crate::tabs::tab_ecs_helper;
use crate::tabs::{show, Tree};
use crate::ui_tabs::add_tab;
use crate::ui_tabs::tab_wrapper::EditorUiWrapper;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::{prelude::*, window::PresentMode, winit::WinitSettings};
use bevy_egui::{egui, EguiContext, EguiPlugin};

#[derive(Default)]
struct UiState {
    is_window_open: bool,
    dropped_files: Vec<egui::DroppedFile>,
    picked_path: Option<String>,
}

const CAMERA_TARGET: Vec3 = Vec3::ZERO;

struct OriginalCameraTransform(Transform);

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
        .init_resource::<RenderSettings>()
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

fn configure_ui_state(mut ui_state: ResMut<UiState>, mut commands: Commands) {
    ui_state.is_window_open = true;
    let camera_pos = Vec3::new(-2.0, 2.5, 5.0);
    let camera_transform = Transform::from_translation(camera_pos)
        .looking_at(CAMERA_TARGET, Vec3::Y);
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: camera_transform,
        ..Default::default()
    });
}

fn ui_renderer(
    mut egui_ctx: ResMut<EguiContext>,
    mut ui_state: ResMut<UiState>,
    mut tab_tree: ResMut<Tree>,
    mut ren_param: ResMut<RenderSettings>,
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

    egui::SidePanel::left("left_panel").resizable(true).show(
        egui_ctx.ctx_mut(),
        |ui| {
            ui.label("Drag-and-drop files onto the window!");

            if ui.button("Open file…").clicked() {
                if let Some(path) = rfd::FileDialog::new().pick_file() {
                    ui_state.picked_path = Some(path.display().to_string());
                }
            }

            if let Some(picked_path) = &ui_state.picked_path {
                ui.horizontal(|ui| {
                    ui.label("Picked file:");
                    ui.monospace(picked_path);
                });
            }

            // Show dropped files (if any):
            if !ui_state.dropped_files.is_empty() {
                ui.group(|ui| {
                    ui.label("Dropped files:");

                    for file in &ui_state.dropped_files {
                        let mut info = if let Some(path) = &file.path {
                            path.display().to_string()
                        } else if !file.name.is_empty() {
                            file.name.clone()
                        } else {
                            "???".to_owned()
                        };
                        if let Some(bytes) = &file.bytes {
                            use std::fmt::Write as _;
                            write!(info, " ({} bytes)", bytes.len()).ok();
                        }
                        ui.label(info);
                    }
                });
            }
        },
    );
    preview_files_being_dropped(egui_ctx.ctx_mut());
    // Collect dropped files:
    if !egui_ctx.ctx_mut().input().raw.dropped_files.is_empty() {
        ui_state.dropped_files =
            egui_ctx.ctx_mut().input().raw.dropped_files.clone();
    }

    egui::CentralPanel::default().show(egui_ctx.ctx_mut(), |ui| {
        show(
            ui,
            egui::Id::new("some hashable string"),
            &mut *tab_tree,
            &mut *ren_param,
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

fn update_camera_transform_system(
    mut ui_state: ResMut<UiState>,
    original_camera_transform: Res<OriginalCameraTransform>,
    windows: Res<Windows>,
    mut camera_query: Query<(&PerspectiveProjection, &mut Transform)>,
) {
    let (camera_projection, mut transform) =
        camera_query.get_single_mut().unwrap();

    let distance_to_target =
        (CAMERA_TARGET - original_camera_transform.0.translation).length();
    let frustum_height =
        2.0 * distance_to_target * (camera_projection.fov * 0.5).tan();
    let frustum_width = frustum_height * camera_projection.aspect_ratio;

    let window = windows.get_primary().unwrap();

    let left_taken = 0.0 / window.width();
    let right_taken = 0.0 / window.width();
    let top_taken = 0.0 / window.height();
    let bottom_taken = 0.0 / window.height();
    transform.translation = original_camera_transform.0.translation
        + transform.rotation.mul_vec3(Vec3::new(
            (right_taken - left_taken) * frustum_width * 0.5,
            (top_taken - bottom_taken) * frustum_height * 0.5,
            0.0,
        ));
}
