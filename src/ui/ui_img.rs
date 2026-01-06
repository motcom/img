use crate::app::image_application::ImageApplication;
use crate::domain::types::PasteItem;
use crate::domain::types::ZoomFactor;
use arboard::Clipboard;
use eframe::egui::TextureHandle;
use eframe::egui::TextureOptions;
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

/// 画像アプリケーションの状態を保持する構造体です。
pub struct ImageViewer {
    /// 画像管理用のアプリケーション層
    img_app: ImageApplication,
}

/// eframe::Appトレイトの実装。フレームごとにUIを更新します。
impl eframe::App for ImageViewer {
    /// 毎フレーム呼ばれ、UIの描画・イベント処理を行います。
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.input(ui, ctx, frame);
            self.render(ui, ctx, frame);
        });
    }
}

/// ImageViewerのユーティリティメソッド群
impl ImageViewer {
    /// 新しいImageViewerインスタンスを生成します。
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            img_app: ImageApplication::new(),
        }
    }
    fn color_image_to_texture(&mut self, ctx: &egui::Context) -> Option<TextureHandle> {
        let color_img_ref_opt = self.img_app.get_image();
        if let Some(color_img_ref) = color_img_ref_opt {
            let color_img_ref = color_img_ref.clone();
            return Some(ctx.load_texture(
                "main_render_img",
                color_img_ref,
                TextureOptions::default(),
            ));
        };
        None
    }

    fn render(
        &mut self,
        ui: &mut Ui,
        ctx: &egui::Context,
        _frame: &mut eframe::Frame,
    ) -> Option<()> {
        // 画像の描画
        let texture_handle_opt = self.color_image_to_texture(ctx);
        let zoom_factor = self.img_app.get_cur_zoomfactor();
        if let Some(tex_handle) = texture_handle_opt {
            let tex_size = tex_handle.size();
            let tex_size = egui::Vec2::new(
                tex_size[0] as f32 * zoom_factor.get(),
                tex_size[1] as f32 * zoom_factor.get(),
            );
            ui.add(egui::Image::new((tex_handle.id(), tex_size)));
        }
        Some(())
    }

    fn input(
        &mut self,
        ui: &mut Ui,
        _ctx: &egui::Context,
        _frame: &mut eframe::Frame,
    ) -> Option<()> {
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
        }

        // Jキーで画像ズームアップ
        if ui.input(|i| i.key_pressed(egui::Key::J)) {
            let zoom_factor = self.img_app.get_cur_zoomfactor();
            let val = zoom_factor.get() + 0.25;
            self.img_app.set_cur_zoomfactor(ZoomFactor::new(val));
        }

        // Kキーで画像ズームダウン
        if ui.input(|i| i.key_pressed(egui::Key::K)) {
            let zoom_factor = self.img_app.get_cur_zoomfactor();
            let val = zoom_factor.get() - 0.25;
            self.img_app.set_cur_zoomfactor(ZoomFactor::new(val));
        }

        // Lキーで次の画像へ
        if ui.input(|i| i.key_pressed(egui::Key::L)) {
            self.img_app.next();
        }
        // Hキーで前の画像へ
        if ui.input(|i| i.key_pressed(egui::Key::H)) {
            self.img_app.previous();
        }

        Some(())
    }
}
