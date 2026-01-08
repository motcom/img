#![allow(dead_code)]
use image::RgbImage;

use crate::domain::image_clips_domains::ImageClipDomain;
use crate::domain::types::ZoomFactor;
use crate::domain::{
    image_pathes_domain::ImagePathes, images_domains_trait::ImageDomainTrait, types::PasteItem,
};
use std::path::Path;

/// 画像の管理・操作を行うアプリケーション層の構造体です。
/// 画像の貼り付け、切り替え、表示用テクスチャの管理などを担当します。
#[derive(Default)]
pub struct ImageApplication {
    /// 画像ドメインのトレイトオブジェクト
    image_domain_trait: Option<Box<dyn ImageDomainTrait>>,
}

/// ImageApplicationのユーティリティメソッド群
impl ImageApplication {
    /// 新しいImageApplicationインスタンスを生成します。
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    /// 画像やテキストを貼り付けます。
    /// テキストの場合はパスとして扱い、存在すればImagePathesとして管理します。
    /// 画像の場合はImageClipDomainとして管理します。
    pub fn paste(&mut self, paste_item: PasteItem) {
        match paste_item {
            PasteItem::Text(txt) => {
                let path = Path::new(&txt);
                let mut image_pathes_domain = ImagePathes::new();
                image_pathes_domain.pasete(PasteItem::Text(txt.clone()));
                if path.exists() {
                    self.image_domain_trait = Some(Box::<ImagePathes>::new(image_pathes_domain));
                }
            }

            PasteItem::Image(img) => {
                let domain = self
                    .image_domain_trait
                    .get_or_insert_with(|| Box::new(ImageClipDomain::new()));

                domain.pasete(PasteItem::Image(img));
            }
        }
    }

    pub fn get_image(&mut self) -> Option<&RgbImage> {
        if let Some(img_trait) = &mut self.image_domain_trait {
            return img_trait.get_image();
        }
        None
    }

    /// 次の画像に切り替えます。
    pub fn next(&mut self) {
        if let Some(img_trait) = self.image_domain_trait.as_mut() {
            img_trait.next();
        }
    }

    /// 前の画像に切り替えます。
    pub fn previous(&mut self) {
        if let Some(img_trait) = self.image_domain_trait.as_mut() {
            img_trait.previous();
        }
    }

    pub fn get_cur_zoomfactor(&self) -> ZoomFactor {
        if let Some(img_trait) = &self.image_domain_trait {
            return img_trait.get_curimage_factor();
        }
        ZoomFactor::default()
    }

    pub fn set_cur_zoomfactor(&mut self, zoom_factor: ZoomFactor) {
        if let Some(img_trait) = &mut self.image_domain_trait {
            img_trait.set_curimage_factor(zoom_factor);
        }
    }
}
