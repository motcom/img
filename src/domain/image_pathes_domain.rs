#![allow(dead_code)]
use crate::domain::images_domains_trait::ImageDomainTrait;
use crate::domain::types::{PasteItem, ZoomFactor};
use eframe::egui::ColorImage;
use image;
use image::RgbImage;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

/// サポートされている画像拡張子のリスト
const IMAGE_FORMAT: [&str; 4] = ["png", "jpg", "tga", "webp"];

/// ディレクトリ内の画像ファイルパスを管理し、画像の切り替えや取得を行う構造体です。
#[derive(Default)]
pub struct ImagePathes {
    /// 現在読み込まれている画像
    cur_img: Option<RgbImage>,
    /// 管理している画像ファイルのパスリスト
    image_lst: Vec<PathBuf>,
    /// 現在選択されている画像のインデックス
    index: usize,

    /// indexでzoomを保持
    index_to_zoomfactor: HashMap<usize, ZoomFactor>,
}

/// ImagePathesのユーティリティメソッド群
impl ImagePathes {
    /// 指定したパスから対応する画像ファイルを収集します。
    fn collect_images_from_path<T: AsRef<Path>>(&mut self, path: T) {
        self.image_lst.clear();
        if let Ok(entries) = fs::read_dir(path) {
            for ent in entries.flatten() {
                let path = ent.path();
                if let Some(ext) = path.extension().and_then(|e| e.to_str())
                    && IMAGE_FORMAT.contains(&ext)
                {
                    self.image_lst.push(path);
                }
            }
        }
    }

    /// 新しいImagePathesインスタンスを生成します。
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}

/// 画像パスリストの切り替えや画像の取得などのトレイト実装
impl ImageDomainTrait for ImagePathes {
    /// テキスト（パス）を貼り付け、画像ファイルリストを更新します。
    fn pasete(&mut self, paste_item: PasteItem) {
        if let PasteItem::Text(text) = paste_item {
            let path = Path::new(&text);
            self.collect_images_from_path(path);
        }
    }

    /// 現在選択されている画像を読み込み、返します。
    fn get_image(&mut self) -> Option<&RgbImage> {
        if let Some(img_path) = self.image_lst.get(self.index) {
            self.cur_img = image::open(img_path).ok().and_then(|px| Some(px.to_rgb8()));
        }
        None
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

    fn get_image_nums(&self) -> usize {
        self.image_lst.len()
    }

    fn get_samnails(
        &self,
        samnail_cur_index: usize,
        samnail_nums_per_page: usize,
    ) -> Vec<RgbImage> {
        todo!()
    }
}

// Test -------------------------------------------------------------------->
#[cfg(test)]
mod tests {
    use crate::domain::image_pathes_domain::ImagePathes;

    /// 指定ディレクトリ内の画像ファイル収集テスト
    #[test]
    fn test_image_path() {
        let mut img_pathes = ImagePathes::new();
        img_pathes.collect_images_from_path("contents/");
        assert_eq!(img_pathes.image_lst.len(), 5);
    }
}
// Test <--------------------------------------------------------------------
