use crate::editor::editing_ui;
use crate::editor::render_settings::RenderSettings;
use crate::settings_ui;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::input::mouse::{MouseMotion, MouseWheel};
use bevy::ui;
use bevy::{prelude::*, window::PresentMode, winit::WinitSettings};
use bevy_egui::{egui, EguiContext, EguiPlugin};

#[derive(Default)]
struct UiState {
    is_window_open: bool,
    left: f32,
    top: f32,
    right: f32,
    bottom: f32,
}

#[derive(Component)]
struct PanOrbitCamera {
    /// The "focus point" to orbit around. It is automatically updated when panning the camera
    pub focus: Vec3,
    pub radius: f32,
    pub upside_down: bool,
}

impl Default for PanOrbitCamera {
    fn default() -> Self {
        PanOrbitCamera {
            focus: Vec3::ZERO,
            radius: 5.0,
            upside_down: false,
        }
    }
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
        .init_resource::<RenderSettings>()
        .init_resource::<editing_ui::EditingUI>()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_system(pan_orbit_camera)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_startup_system(configure_ui_state)
        .add_system(ui_renderer.after(pan_orbit_camera))
        .run();
}

fn configure_ui_state(
    mut ui_state: ResMut<UiState>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    ui_state.is_window_open = true;

    let camera_pos = Vec3::new(-2.0, 2.5, 5.0);
    let radius = camera_pos.length();
    let camera_transform = Transform::from_translation(camera_pos)
        .looking_at(CAMERA_TARGET, Vec3::Y);
    commands.insert_resource(OriginalCameraTransform(camera_transform));
    let mut spawn_bund = OrthographicCameraBundle::new_3d();
    spawn_bund.transform = camera_transform;
    commands.spawn_bundle(spawn_bund).insert(PanOrbitCamera {
        radius,
        ..Default::default()
    });
    // plane
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });
    // cube
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    });
    // light
    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
}

fn ui_renderer(
    mut egui_ctx: ResMut<EguiContext>,
    mut ui_state: ResMut<UiState>,
    mut ren_param: ResMut<RenderSettings>,
    mut edit_ui: ResMut<editing_ui::EditingUI>,
    commands: Commands,
) {
    egui::Window::new("Window")
        .vscroll(true)
        .open(&mut ui_state.is_window_open)
        .show(egui_ctx.ctx_mut(), |ui| {
            settings_ui::ui(ui);
        });

    ui_state.left = egui::SidePanel::left("left_panel")
        .resizable(true)
        .show(egui_ctx.ctx_mut(), |ui| {
            ui.label("Drag-and-drop files onto the window!");
        })
        .response
        .rect
        .width();
    ui_state.top = egui::TopBottomPanel::top("top_panel")
        .resizable(true)
        .show(egui_ctx.ctx_mut(), |ui| {
            // The top panel is often a good place for a menu bar:
            ui.label("Drag-and-drop files onto the window!");
        })
        .response
        .rect
        .height();

    ui_state.bottom = egui::TopBottomPanel::bottom("bottom_panel")
        .resizable(true)
        .show(egui_ctx.ctx_mut(), |ui| {
            // The top panel is often a good place for a menu bar:

            edit_ui.ui(ui);
        })
        .response
        .rect
        .height();
}

/// Pan the camera with middle mouse click, zoom with scroll wheel, orbit with right mouse click.
fn pan_orbit_camera(
    original_camera_transform: Res<OriginalCameraTransform>,
    mut ui_state: ResMut<UiState>,
    windows: Res<Windows>,
    mut ev_motion: EventReader<MouseMotion>,
    mut ev_scroll: EventReader<MouseWheel>,
    input_mouse: Res<Input<MouseButton>>,
    mut query: Query<(
        &mut PanOrbitCamera,
        &mut Transform,
        &OrthographicProjection,
    )>,
) {
    let window = windows.get_primary().unwrap();
    // change input mapping for orbit and panning here
    let orbit_button = MouseButton::Middle;
    let pan_button = MouseButton::Right;

    let mut pan = Vec2::ZERO;
    let mut rotation_move = Vec2::ZERO;
    let mut scroll = 0.0;
    let mut orbit_button_changed = false;

    if input_mouse.pressed(orbit_button) {
        for ev in ev_motion.iter() {
            rotation_move += ev.delta;
        }
    } else if input_mouse.pressed(pan_button) {
        // Pan only if we're not rotating at the moment
        for ev in ev_motion.iter() {
            pan += ev.delta;
        }
    }
    for ev in ev_scroll.iter() {
        scroll += ev.y;
    }
    if input_mouse.just_released(orbit_button)
        || input_mouse.just_pressed(orbit_button)
    {
        orbit_button_changed = true;
    }

    for (mut pan_orbit, mut transform, projection) in query.iter_mut() {
        if orbit_button_changed {
            // only check for upside down when orbiting started or ended this frame
            // if the camera is "upside" down, panning horizontally would be inverted, so invert the input to make it correct
            let up = transform.rotation * Vec3::Y;
            pan_orbit.upside_down = up.y <= 0.0;
        }

        let mut any = false;
        if rotation_move.length_squared() > 0.0 {
            any = true;
            let window = get_primary_window_size(&windows);
            let delta_x = {
                let delta =
                    rotation_move.x / window.x * std::f32::consts::PI * 2.0;
                if pan_orbit.upside_down {
                    -delta
                } else {
                    delta
                }
            };
            let delta_y = rotation_move.y / window.y * std::f32::consts::PI;
            let yaw = Quat::from_rotation_y(-delta_x);
            let pitch = Quat::from_rotation_x(-delta_y);
            transform.rotation = yaw * transform.rotation; // rotate around global y axis
            transform.rotation *= pitch; // rotate around local x axis
        } else if pan.length_squared() > 0.0 {
            any = true;
            // make panning distance independent of resolution and FOV,
            let window = get_primary_window_size(&windows);

            // translate by local axes
            let right = transform.rotation * Vec3::X * -pan.x;
            let up = transform.rotation * Vec3::Y * pan.y;
            // make panning proportional to distance away from focus point
            let translation = (right + up) * pan_orbit.radius;
            pan_orbit.focus += translation;
        } else if scroll.abs() > 0.0 {
            any = true;
            pan_orbit.radius -= scroll * pan_orbit.radius * 0.2;
            // dont allow zoom to reach zero or you get stuck
            pan_orbit.radius = f32::max(pan_orbit.radius, 0.05);
        }

        if any {
            // emulating parent/child to make the yaw/y-axis rotation behave like a turntable
            // parent = x and y rotation
            // child = z-offset
            let rot_matrix = Mat3::from_quat(transform.rotation);
            transform.translation = pan_orbit.focus
                + rot_matrix.mul_vec3(Vec3::new(0.0, 0.0, pan_orbit.radius));
        }
    }
}

fn get_primary_window_size(windows: &Res<Windows>) -> Vec2 {
    let window = windows.get_primary().unwrap();

    Vec2::new(window.width() as f32, window.height() as f32)
}
