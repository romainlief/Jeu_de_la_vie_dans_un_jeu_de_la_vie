use crate::simulation::GameOfLifeInGameOfLife;
use eframe::{egui, App};
use egui::{ColorImage, TextureHandle, TextureOptions};
use std::time::{Duration, Instant};

pub struct GOLApp {
    sim: GameOfLifeInGameOfLife,
    running: bool,
    interval: Duration,
    interval_ms: u64,
    last_update: Instant,
    texture: Option<TextureHandle>,
}

impl GOLApp {
    pub fn new(big_n: usize, small_n: usize, vie: f64, interval_ms: u64) -> Self {
        Self {
            sim: GameOfLifeInGameOfLife::new(big_n, small_n, vie),
            running: false,
            interval: Duration::from_millis(interval_ms),
            interval_ms,
            last_update: Instant::now(),
            texture: None,
        }
    }

    fn update_texture(&mut self, ctx: &egui::Context) {
        let big_n = self.sim.get_big_N();
        let densities = self.sim.density_grid();
        // build RGBA pixels
        let mut pixels: Vec<u8> = Vec::with_capacity(big_n * big_n * 4);
        for v in densities.iter() {
            let val = ((*v).clamp(0.0, 1.0) * 255.0) as u8;
            pixels.push(val);
            pixels.push(val);
            pixels.push(val);
            pixels.push(255);
        }

        let image = ColorImage::from_rgba_unmultiplied([big_n, big_n], &pixels);
        // (re)load the texture into the context and store the handle.
        // We always (re)load here to avoid needing mutable access to an existing handle.
        let tex = ctx.load_texture("gol_texture", image, TextureOptions::default());
        self.texture = Some(tex);
    }
}

impl App for GOLApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // timing: step when running and enough time elapsed
        if self.running && self.last_update.elapsed() >= self.interval {
            self.sim.step();
            self.last_update = Instant::now();
            self.update_texture(ctx);
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Game of Life dans Game of Life — densité");

            ui.horizontal(|ui| {
                if ui.button(if self.running { "Stop" } else { "Start" }).clicked() {
                    self.running = !self.running;
                    if self.running {
                        self.last_update = Instant::now();
                    }
                }

                if ui.button("Step").clicked() {
                    self.sim.step();
                    self.update_texture(ctx);
                }

                if ui.button("Randomize").clicked() {
                    self.sim = GameOfLifeInGameOfLife::new(self.sim.get_big_N(), self.sim.get_small_N(), 0.2);
                    self.update_texture(ctx);
                }

                if ui.add(egui::Slider::new(&mut self.interval_ms, 50..=2000).text("interval ms")).changed() {
                    self.interval = Duration::from_millis(self.interval_ms);
                }
            });

            ui.separator();

            if self.texture.is_none() {
                self.update_texture(ctx);
            }

            if let Some(tex) = &self.texture {
                // scale image to fit available width while keeping aspect
                let avail = ui.available_size();
                let size = egui::Vec2::new(avail.x.min(avail.y), avail.x.min(avail.y));
                 // pass a single argument which is (TextureId, Vec2) — ui.image accepts this
                 ui.image((tex.id(), size));
            }
        });

        // request repaint for animation
        ctx.request_repaint_after(Duration::from_millis(16));
    }
}

