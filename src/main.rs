#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

pub mod cell;
pub mod consts;
pub mod game;

use eframe::egui;
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
                ui.label(format!("Tick: {}", self.tick));
                ui.add(egui::Slider::new(&mut self.zoom_level, 0.1..=10.).text("Zoom"));

                ui.checkbox(&mut self.paused, "Pause");

                if ui.button("Close").clicked() {
                    frame.close();
                }
            });

            let painter = egui::Painter::new(
                ui.ctx().clone(),
                ui.layer_id(),
                ui.available_rect_before_wrap(),
            );

            let start_paint = Instant::now();
            self.paint(&painter);
            println!("Paint {:.2?}", start_paint.elapsed());
        });

        let start_tick = Instant::now();
        self.game_tick();
        println!("Tick {:.2?}", start_tick.elapsed());
    }
}
