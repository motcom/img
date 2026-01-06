use eframe::egui::ColorImage;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ZoomFactor(f32);
impl Default for ZoomFactor {
    fn default() -> Self {
        Self(1.0)
    }
}

impl ZoomFactor {
    pub fn set(&mut self, val: f32) {
        self.0 = val.clamp(0.1, 2.0);
    }
    pub fn get(self) -> f32 {
        self.0
    }
}

/// 貼り付け可能なデータ種別を表す列挙型です。
pub enum PasteItem {
    /// テキスト（主にパス情報など）
    Text(String),
    /// 画像データ
    Image(ColorImage),
}

/// 画像ドメインの種別を表す列挙型です。
#[derive(PartialEq, Debug)]
pub enum ImageTraitKind {
    /// パスやテキストによる画像管理
    Text,
    /// 画像そのものによる管理
    Image,
}
