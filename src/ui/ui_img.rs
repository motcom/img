#![allow(dead_code)]
use crate::app::image_application::ImageApplication;
use crate::domain::types::PasteItem;
use arboard::Clipboard;
use eframe::egui::{self, Ui, ViewportBuilder};

/// アプリケーションのエントリポイント関数です。
/// ウィンドウの設定を行い、eframeアプリケーションを起動します。
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

/// eframe::Appトレイトの実装。フレームごとにUIを更新します。
impl eframe::App for ImageViewer {
    /// 毎フレーム呼ばれ、UIの描画・イベント処理を行います。
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.process_ui(ui, ctx, frame);
        });
    }
}

/// 画像アプリケーションの状態を保持する構造体です。
pub struct ImageViewer {
    /// 画像管理用のアプリケーション層
    img_app: ImageApplication,
}

/// ImageViewerのユーティリティメソッド群
impl ImageViewer {
    /// 新しいImageViewerインスタンスを生成します。
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            img_app: ImageApplication::new(),
        }
    }

    /// UIの描画と各種入力イベントの処理を行います。
    ///
    /// - Pキー: クリップボードから画像またはパスを貼り付け
    /// - Lキー: 次の画像へ
    /// - Hキー: 前の画像へ
    fn process_ui(
        &mut self,
        ui: &mut Ui,
        ctx: &egui::Context,
        _frame: &mut eframe::Frame,
    ) -> Option<()> {
        // 画像の描画
        if let Some(tex_handle) = self.img_app.get_image() {
            ui.image(tex_handle);
        }
        // インプット処理 ----------------------------------------------->
        // Pキーでクリップボードから画像またはパスを貼り付け
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

        // Lキーで次の画像へ
        if ui.input(|i| i.key_pressed(egui::Key::L)) {
            self.img_app.next();
            self.img_app.load_img(ctx);
        }
        // Hキーで前の画像へ
        if ui.input(|i| i.key_pressed(egui::Key::H)) {
            self.img_app.previous();
            self.img_app.load_img(ctx);
        }

        // インプット <-----------------------------------------------
        Some(())
    }
}
