#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use eframe::egui::{ self, RichText };
use egui::Key;
use copypasta::{ ClipboardContext, ClipboardProvider };
use std::time::{ Duration, Instant };
use eframe::egui::Color32;

fn main() -> eframe::Result {
    // env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 640.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Clipper",
        options,
        Box::new(|_| {
            // This gives us image support:
            // egui_extras::install_image_loaders(&cc.egui_ctx);

            Ok(Box::<MyApp>::default())
        })
    )
}
struct MyApp {
    id_selected: usize,
    history: Vec<String>,
    last_check: Instant, // Track the last time clipboard was checked
    previous_clip_content: String,
    show_notification: bool,
    notification_start: Option<Instant>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            id_selected: 0,
            history: Vec::new(),
            last_check: Instant::now(), // Initialize with the current time
            previous_clip_content: String::new(),
            show_notification: false,
            notification_start: None,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let check_interval = Duration::from_millis(500);
        let mut clipboard = ClipboardContext::new().expect("Failed to initialize clipboard");
        if self.last_check.elapsed() >= check_interval {
            self.last_check = Instant::now();
            let clip_content = clipboard.get_contents().unwrap();
            if clip_content != self.previous_clip_content && !self.history.contains(&clip_content) {
                self.history.push(clip_content.clone());
                self.previous_clip_content = clip_content;
            }
        }
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Select your clip");
            ui.add_space(3.0);
            egui::ScrollArea::vertical().show(ui, |ui| {
                for (index, text) in self.history.iter().enumerate().rev() {
                    let selectable = ui.selectable_value(&mut self.id_selected, index, text);
                    // Scroll to the selected item
                    if self.id_selected == index {
                        selectable.scroll_to_me(Some(egui::Align::Center));
                    }

                    ui.separator();
                }
            });
            if ctx.input(|i| i.key_pressed(Key::ArrowDown)) {
                if self.id_selected > 0 {
                    self.id_selected = self.id_selected - 1;
                }
            }
            if ctx.input(|i| i.key_pressed(Key::ArrowUp)) {
                if self.id_selected < self.history.len() - 1 {
                    self.id_selected = self.id_selected + 1;
                }
            }
            if ctx.input(|i| i.key_pressed(Key::Enter)) {
                // println!("{}", self.id_selected);
                if let Some(item) = self.history.get(self.id_selected) {
                    clipboard.set_contents(item.to_string()).unwrap();
                    self.show_notification = true;
                    self.notification_start = Some(Instant::now());
                }
            }
            if self.show_notification {
                ui.label(RichText::new("Text Copied!").color(Color32::GREEN));

                // Hide notification after 3 seconds
                if let Some(start) = self.notification_start {
                    if start.elapsed() >= Duration::from_secs(2) {
                        self.show_notification = false;
                    }
                }
            }
        });
    }
}
