use image::RgbImage;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ZoomFactor(f32);
impl Default for ZoomFactor {
    fn default() -> Self {
        Self(1.0)
    }
}

impl ZoomFactor {
    pub fn new(val: f32) -> Self {
        Self(val.clamp(0.1, 2.0))
    }

    #[allow(dead_code)]
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
    Image(RgbImage),
}

///　ビューワーのモード切り替え
pub enum ViewerMode {
    Normal,
    List,
}
