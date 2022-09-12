use std::time::Duration;

use crate::algorithms::{Algorithm, Option};
use crate::States;
use bevy::prelude::*;
use bevy_egui::EguiContext;
use bevy_egui::*;
use iyes_loopless::prelude::*;

pub struct Config {
    pub offset: f32,
    pub algorithm: Algorithm,
    pub width: i32,
    pub height: i32,
    pub options: Vec<Option>,
    delay: u64,
    pub speed_timer: Timer,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            algorithm: Algorithm::default(),
            width: 60,
            height: 40,
            offset: 0.,
            options: vec![],
            delay: 100,
            speed_timer: Timer::new(Duration::from_millis(100), true),
        }
    }
}

pub fn draw_ui(mut cmds: Commands, mut egui_ctx: ResMut<EguiContext>, mut cfg: ResMut<Config>) {
    let width = egui::SidePanel::left("my_side_panel")
        .show(egui_ctx.ctx_mut(), |ui| {
            ui.heading("Dungeon generation");
            ui.add_space(10.);

            {
                let old_algorithm = cfg.algorithm;

                ui.label("Algorithm:");
                egui::ComboBox::from_id_source("Algorithm")
                    .selected_text(cfg.algorithm.to_string())
                    .show_ui(ui, |ui| {
                        for alg in Algorithm::all() {
                            ui.selectable_value(&mut cfg.algorithm, alg, alg.to_string());
                        }
                    });

                ui.add_space(5.);
                if cfg.algorithm != Algorithm::None {
                    ui.label(cfg.algorithm.description());
                }
                ui.add_space(10.);

                if old_algorithm != cfg.algorithm {
                    let alg = cfg.algorithm;
                    cfg.options = alg.options().to_vec();
                }
            }

            {
                let old_speed = cfg.delay;
                ui.label("Map Options:");
                ui.group(|ui| {
                    ui.label("Width:");
                    ui.add(egui::Slider::new(&mut cfg.width, 2..=100));

                    ui.label("Height:");
                    ui.add(egui::Slider::new(&mut cfg.height, 2..=100));

                    ui.label("Delay:");
                    ui.add(egui::Slider::new(&mut cfg.delay, 1..=1000));
                });

                if old_speed != cfg.delay {
                    let duration = Duration::from_millis(cfg.delay);
                    cfg.speed_timer = Timer::new(duration, true);
                }
            }

            if cfg.algorithm != Algorithm::None {
                let alg = cfg.algorithm;
                ui.label("Algorithm Options:");

                ui.group(|ui| {
                    for (idx, option) in alg.options().iter().enumerate() {
                        ui.label(option.name);
                        ui.add(egui::Slider::new(
                            &mut cfg.options[idx].value,
                            option.min..=option.max,
                        ));
                    }
                });
            }

            ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                if ui
                    .add_enabled(
                        cfg.algorithm != Algorithm::None,
                        egui::Button::new("Generate"),
                    )
                    .clicked()
                {
                    cmds.insert_resource(NextState(States::Running));
                }
            });
        })
        .response
        .rect
        .width();

    cfg.offset = width;
}
