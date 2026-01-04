use eframe::egui::ColorImage;

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
