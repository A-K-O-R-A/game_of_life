#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

pub mod cell;
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

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            //ui.heading("Try to close the window");

            let painter = egui::Painter::new(
                ui.ctx().clone(),
                ui.layer_id(),
                ui.available_rect_before_wrap(),
            );

            self.paint(&painter);
        });

        //let start = Instant::now();
        self.game_tick();
        //println!("Elapsed {:.2?}", start.elapsed());
    }
}
