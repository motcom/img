#![allow(dead_code)]
use crate::domain::images_domains_trait::ImageDomainTrait;
use crate::domain::types::{ImageTraitKind, PasteItem};
use eframe::egui::ColorImage;
use image;
use std::fs;
use std::path::{Path, PathBuf};

const IMAGE_FORMAT: [&str; 3] = ["png", "jpg", "tga"];

#[derive(Default)]
pub struct ImagePathes {
    cur_img: Option<ColorImage>,
    image_lst: Vec<PathBuf>,
    index: usize,
}

impl ImagePathes {
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
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}

impl ImageDomainTrait for ImagePathes {
    fn next(&mut self) {
        let len = self.image_lst.len();
        if len == 0 {
            return;
        }
        self.index = (self.index + 1) % len;
    }

    fn previous(&mut self) {
        let len = self.image_lst.len();
        if len == 0 {
            return;
        }
        self.index = (self.index + len - 1) % len;
    }

    fn pasete(&mut self, paste_item: PasteItem) {
        if let PasteItem::Text(text) = paste_item {
            let path = Path::new(&text);
            self.collect_images_from_path(path);
        }
    }

    fn get_image(&mut self) -> Option<&ColorImage> {
        if let Some(img_path) = self.image_lst.get(self.index) {
            let img = image::open(img_path).ok()?;
            self.cur_img = Some(ColorImage::from_rgb(
                [img.width() as usize, img.height() as usize],
                img.to_rgb8().as_raw(),
            ));
            return self.cur_img.as_ref();
        }
        None
    }

    fn kind(&self) -> super::types::ImageTraitKind {
        ImageTraitKind::Text
    }
}

// Test -------------------------------------------------------------------->
#[cfg(test)]
mod tests {
    use crate::domain::image_pathes_domain::ImagePathes;

    #[test]
    fn test_image_path() {
        let mut img_pathes = ImagePathes::new();
        img_pathes.collect_images_from_path("contents/");
        assert_eq!(img_pathes.image_lst.len(), 5);
    }
}

// Test <--------------------------------------------------------------------
