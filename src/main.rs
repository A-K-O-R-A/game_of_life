#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

pub mod cell;
pub mod consts;
pub mod game;

use eframe::egui::{self, PointerState};
use std::time::Instant;

#[allow(arithmetic_overflow)]
fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Game of Life",
        options,
        Box::new(|_cc| Box::new(game::Game::default())),
    );
}

impl eframe::App for game::Game {
    fn on_close_event(&mut self) -> bool {
        true
    }

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        println!("\x1B[2J\x1B[1;1H");

        egui::CentralPanel::default().show(ctx, |ui| {
            //ui.heading("Try to close the window");

            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.add(egui::Slider::new(&mut self.zoom_level, 0.1..=10.).text("Zoom"))
                        .on_hover_text("A bit buggy");
                    ui.add(egui::Slider::new(&mut self.tps, 1..=200).text("TPS"))
                        .on_hover_text("Not yet implemented");
                });

                ui.vertical(|ui| {
                    ui.label(format!("Tick: {}", self.tick))
                        .on_hover_text("The generation counter");
                    ui.checkbox(&mut self.paused, "Pause")
                        .on_hover_text("Pause the game, equals 0 TPS");
                });

                if ui.button("Close").clicked() {
                    frame.close();
                }
            });

            let painter = egui::Painter::new(
                ui.ctx().clone(),
                ui.layer_id(),
                ui.available_rect_before_wrap(),
            );

            let hover_pos = ui.input().pointer.hover_pos();
            //let pointer = &PointerState::default();

            let start_paint = Instant::now();
            self.paint(&painter, hover_pos);
            println!("Paint {:.2?}", start_paint.elapsed());

            //Game update
            let start_tick = Instant::now();
            self.game_tick(&PointerState::default());
            println!("Tick {:.2?}", start_tick.elapsed());
        });
    }
}
