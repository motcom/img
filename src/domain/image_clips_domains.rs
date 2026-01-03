use crate::domain::{
    images_domains_trait::ImageDomainTrait,
    types::{ImageTraitKind, PasteItem},
};
use eframe::egui::ColorImage;

#[derive(Default)]
pub struct ImageClipDomain {
    image_lst: Vec<ColorImage>,
    index: usize,
}

impl ImageClipDomain {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}

impl ImageDomainTrait for ImageClipDomain {
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

    fn pasete(&mut self, paste_item: super::types::PasteItem) {
        if let PasteItem::Image(img) = paste_item {
            self.image_lst.push(img);
            self.index = self.image_lst.len() - 1;
        }
    }

    fn get_image(&mut self) -> Option<&ColorImage> {
        self.image_lst.get(self.index)
    }

    fn kind(&self) -> super::types::ImageTraitKind {
        ImageTraitKind::Image
    }
}
