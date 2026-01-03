#![allow(dead_code)]
use arboard::Clipboard;
use eframe::egui::{self, Ui, ViewportBuilder};

use crate::app::image_application::ImageApplication;
use crate::domain::types::PasteItem;

pub fn exe() {
    let native_options = eframe::NativeOptions {
        viewport: ViewportBuilder {
            window_level: Some(egui::WindowLevel::AlwaysOnTop),
            ..Default::default()
        },
        ..Default::default()
    };

    eframe::run_native(
        "img",
        native_options,
        Box::new(|cc| Ok(Box::new(ImageViewer::new(cc)))),
    )
    .expect("failed run loop");
}

pub struct ImageViewer {
    img_app: ImageApplication,
}

impl ImageViewer {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            img_app: ImageApplication::new(),
        }
    }

    fn process_ui(
        &mut self,
        ui: &mut Ui,
        ctx: &egui::Context,
        _frame: &mut eframe::Frame,
    ) -> Option<()> {
        // render
        if let Some(tex_handle) = self.img_app.get_image() {
            ui.image(tex_handle);
        }
        //input ----------------------------------------------->
        // Paste
        if ui.input(|i| i.key_pressed(egui::Key::P)) {
            let mut clip = Clipboard::new().ok()?;
            if let Ok(path_str) = clip.get_text() {
                self.img_app.paste(PasteItem::Text(path_str));
            }
            if let Ok(img) = clip.get_image() {
                let pixels: Vec<egui::Color32> = img
                    .bytes
                    .chunks(4)
                    .map(|c| egui::Color32::from_rgba_unmultiplied(c[0], c[1], c[2], c[3]))
                    .collect();

                let color_image = egui::ColorImage::new([img.width, img.height], pixels);
                self.img_app.paste(PasteItem::Image(color_image));
            }

            self.img_app.load_img(ctx);
        }
        // Next
        if ui.input(|i| i.key_pressed(egui::Key::L)) {
            self.img_app.next();
            self.img_app.load_img(ctx);
        }
        // Previous
        if ui.input(|i| i.key_pressed(egui::Key::H)) {
            self.img_app.previous();
            self.img_app.load_img(ctx);
        }

        //input <-----------------------------------------------
        Some(())
    }
}

impl eframe::App for ImageViewer {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.process_ui(ui, ctx, frame);
        });
    }
}
