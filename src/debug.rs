use bevy::{
    diagnostic::{Diagnostics, EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin},
    prelude::*,
};
use bevy_egui::{egui, EguiContext};
use bevy_inspector_egui::WorldInspectorPlugin;
use iyes_loopless::prelude::*;

use crate::States;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        if cfg!(debug_assertions) {
            app.add_system(debug_current_state)
                .add_plugin(WorldInspectorPlugin::new())
                // Get frame time diagnostics
                .add_plugin(FrameTimeDiagnosticsPlugin::default())
                // Get entity count diagnostics
                .add_plugin(EntityCountDiagnosticsPlugin::default())
                // Add the diagnostics window
                .add_system(diagnostic_window);
        }
    }
}

fn debug_current_state(state: Res<CurrentState<States>>) {
    if state.is_changed() {
        println!("Detected state change to {:?}!", state);
    }
}

/// System to create the diagnostics window
fn diagnostic_window(mut egui_context: ResMut<EguiContext>, diagnostics: Res<Diagnostics>) {
    egui::Window::new("Diagnostics").show(egui_context.ctx_mut(), |ui| {
        if let Some(fps) = diagnostics.get_measurement(FrameTimeDiagnosticsPlugin::FPS) {
            ui.label(format!("FPS: {:.0}", fps.value));
        }
        if let Some(frame_time) =
            diagnostics.get_measurement(FrameTimeDiagnosticsPlugin::FRAME_TIME)
        {
            ui.label(format!("FRAME_TIME: {:.2}", frame_time.value));
        }
        if let Some(entity_count) =
            diagnostics.get_measurement(EntityCountDiagnosticsPlugin::ENTITY_COUNT)
        {
            ui.label(format!("ENTITY_COUNT: {:.0}", entity_count.value));
        }
    });
}
