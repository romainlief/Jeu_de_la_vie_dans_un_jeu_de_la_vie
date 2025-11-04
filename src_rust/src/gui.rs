use crate::simulation::GameOfLifeInGameOfLife;
use eframe::{egui, App};
use egui::{ColorImage, TextureHandle, TextureOptions, TextureFilter};
use std::time::{Duration, Instant};
use display_info::DisplayInfo;

pub fn launch_gui_application() -> eframe::Result<()> {
        let display = DisplayInfo::all()
        .unwrap()
        .into_iter()
        .find(|d| d.is_primary)
        .unwrap_or(DisplayInfo {
            id: 0,
            name: String::new(),
            friendly_name: String::new(),
            width: 1200,
            height: 800,
            width_mm: 0,
            height_mm: 0,
            frequency: 60.0,
            rotation: 0.0,
            scale_factor: 1.0,
            is_primary: true,
            raw_handle: unsafe { std::mem::zeroed() },
            x: 0,
            y: 0,
        });
    let width = display.width as f32 * 0.8;
    let height = display.height as f32 * 0.8;

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([width, height])
            .with_min_inner_size([width, height])
            .with_title("Game of Life in Game of Life"),
        ..Default::default()
    };
    eframe::run_native(
        "Game of Life in Game of Life",
        options,
        Box::new(|_cc| Ok(Box::new(GOLApp::new(40, 30, 0.2, 200)))),
    )
}

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
        let mut options = TextureOptions::default();
        options.minification = TextureFilter::Nearest;
        options.magnification = TextureFilter::Nearest;

        let tex = ctx.load_texture("gol_texture", image, options);
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

