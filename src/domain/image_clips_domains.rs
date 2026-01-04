use crate::domain::{
    images_domains_trait::ImageDomainTrait,
    types::{ImageTraitKind, PasteItem},
};
use eframe::egui::ColorImage;

/// 複数の画像をリストで管理し、インデックスで操作できる構造体です。
#[derive(Default)]
pub struct ImageClipDomain {
    /// 管理している画像のリスト
    image_lst: Vec<ColorImage>,
    /// 現在選択されている画像のインデックス
    index: usize,
}

/// ImageClipDomainのユーティリティメソッド群
impl ImageClipDomain {
    /// 新しいImageClipDomainインスタンスを生成します。
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}

/// 画像リストの切り替えや貼り付けなどのトレイト実装
impl ImageDomainTrait for ImageClipDomain {
    /// 次の画像にインデックスを進めます。
    fn next(&mut self) {
        let len = self.image_lst.len();
        if len == 0 {
            return;
        }
        self.index = (self.index + 1) % len;
    }

    /// 前の画像にインデックスを戻します。
    fn previous(&mut self) {
        let len = self.image_lst.len();
        if len == 0 {
            return;
        }
        self.index = (self.index + len - 1) % len;
    }

    /// 画像を貼り付けリストに追加します。
    fn pasete(&mut self, paste_item: super::types::PasteItem) {
        if let PasteItem::Image(img) = paste_item {
            self.image_lst.push(img);
            self.index = self.image_lst.len() - 1;
        }
    }

    /// 現在選択されている画像を取得します。
    fn get_image(&mut self) -> Option<&ColorImage> {
        self.image_lst.get(self.index)
    }

    /// このドメインの種別を返します。
    fn kind(&self) -> super::types::ImageTraitKind {
        ImageTraitKind::Image
    }
}

// テスト ----------------------------------------------------------->
#[cfg(test)]
mod tests {
    use super::*;
    use eframe::egui::ColorImage;

    /// ダミー画像を生成します。
    fn dummy_image() -> ColorImage {
        ColorImage::example()
    }

    /// new()の動作確認テスト
    #[test]
    fn test_new() {
        let domain = ImageClipDomain::new();
        assert_eq!(domain.image_lst.len(), 0);
        assert_eq!(domain.index, 0);
    }

    /// 画像の貼り付けと取得のテスト
    #[test]
    fn test_paste_and_get_image() {
        let mut domain = ImageClipDomain::new();
        let img = dummy_image();
        let paste_item = PasteItem::Image(img.clone());
        domain.pasete(paste_item);
        assert_eq!(domain.image_lst.len(), 1);
        assert_eq!(domain.get_image().unwrap().size, img.size);
    }

    /// next/previousの動作確認テスト
    #[test]
    fn test_next_and_previous() {
        let mut domain = ImageClipDomain::new();
        for _ in 0..3 {
            let img = dummy_image();
            domain.pasete(PasteItem::Image(img));
        }
        let initial_index = domain.index;
        domain.next();
        assert_eq!(domain.index, (initial_index + 1) % 3);
        domain.previous();
        assert_eq!(domain.index, initial_index);
    }

    /// kind()の返り値テスト
    #[test]
    fn test_kind() {
        let domain = ImageClipDomain::new();
        assert_eq!(domain.kind(), ImageTraitKind::Image);
    }
}
// テスト <-----------------------------------------------------------
