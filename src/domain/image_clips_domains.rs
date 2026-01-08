use crate::domain::{
    images_domains_trait::ImageDomainTrait,
    types::{PasteItem, ZoomFactor},
};
use image::RgbImage;
use std::collections::HashMap;

/// 複数の画像をリストで管理し、インデックスで操作できる構造体です。
#[derive(Default)]
pub struct ImageClipDomain {
    /// 管理している画像のリスト
    image_lst: Vec<RgbImage>,
    /// 現在選択されている画像のインデックス
    index: usize,

    /// indexでzoomを保持
    index_to_zoomfactor: HashMap<usize, ZoomFactor>,
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
    /// 画像を貼り付けリストに追加します。
    fn pasete(&mut self, paste_item: super::types::PasteItem) {
        if let PasteItem::Image(img) = paste_item {
            self.image_lst.push(img);
            self.index = self.image_lst.len() - 1;
        }
    }

    /// 現在選択されている画像を取得します。
    fn get_image(&mut self) -> Option<&RgbImage> {
        self.image_lst.get(self.index)
    }

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

    fn get_curimage_factor(&self) -> ZoomFactor {
        self.index_to_zoomfactor
            .get(&self.index)
            .copied()
            .unwrap_or_default()
    }

    fn set_curimage_factor(&mut self, zoom_factor: ZoomFactor) {
        self.index_to_zoomfactor.insert(self.index, zoom_factor);
    }

    fn get_samnails(
        &self,
        samnail_cur_index: usize,
        samnail_nums_per_page: usize,
    ) -> Vec<RgbImage> {
        todo!()
    }

    fn get_image_nums(&self) -> usize {
        self.image_lst.len()
    }
}
