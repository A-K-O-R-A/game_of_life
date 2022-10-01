#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

/*
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    let hi_from_thread = thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(0));
        }
    });

    let inf_messages = thread::spawn(move || {
        let mut counter = 0_u64;

        loop {
            let s = format!("Counter is: {}", counter);
            tx.send(s).unwrap();
            thread::sleep(Duration::from_millis(100));
            counter += 1;
        }
    });

    //handle.join().unwrap();

    thread::sleep(Duration::from_secs(3));

    let latest = rx.try_iter().last().unwrap();
    println!("Latest: {}", latest);

    for received in rx {
        println!("Got: {}", received);
    }
}
*/

pub mod cell;
pub mod consts;
pub mod game;

use eframe::egui::{self};
use std::time::Instant;

#[allow(arithmetic_overflow)]
fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Game of Life",
        options,
        Box::new(|_cc| Box::new(game::Game::init_with_tick_thread())),
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
            /*
            let mut hovered_cell_pos = self.mouse_hover(hover_pos);

            println!("getting hover pos");


            if hovered_cell_pos != None {
                if !ui.input().pointer.primary_clicked() {
                    hovered_cell_pos = None;
                }
            }
             */

            println!("updating pos");
            self.update_board();
            println!("updated pos");

            println!("Tick {:.2?}", start_tick.elapsed());
        });
    }
}
