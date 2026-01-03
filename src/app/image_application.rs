#![allow(dead_code)]
use eframe::egui::{self, TextureHandle, TextureOptions};
use std::path::Path;

use crate::domain::types::ImageTraitKind;
use crate::domain::{
    image_pathes_domain::ImagePathes, images_domains_trait::ImageDomainTrait, types::PasteItem,
};

use crate::domain::image_clips_domains::ImageClipDomain;

#[derive(Default)]
pub struct ImageApplication {
    image_domain_trait: Option<Box<dyn ImageDomainTrait>>,
    tex_handle_opt: Option<TextureHandle>,
}
impl ImageApplication {
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
                if let Some(img_trait) = &self.image_domain_trait
                    && let ImageTraitKind::Image = img_trait.kind()
                {
                    self.image_domain_trait
                        .as_mut()
                        .unwrap()
                        .pasete(PasteItem::Image(img));
                } else {
                    let mut image_clips_domain = ImageClipDomain::new();
                    image_clips_domain.pasete(PasteItem::Image(img));
                    self.image_domain_trait = Some(Box::<ImageClipDomain>::new(image_clips_domain));
                }
            }
        }
    }

    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
    pub fn get_image(&self) -> Option<&TextureHandle> {
        self.tex_handle_opt.as_ref()
    }
    pub fn next(&mut self) {
        if let Some(img_trait) = self.image_domain_trait.as_mut() {
            img_trait.next();
        }
    }
    pub fn previous(&mut self) {
        if let Some(img_trait) = self.image_domain_trait.as_mut() {
            img_trait.previous();
        }
    }

    pub fn load_img(&mut self, ctx: &egui::Context) {
        if let Some(img_domain_trait) = self.image_domain_trait.as_mut() {
            let img = img_domain_trait.get_image().unwrap().clone();
            let tex = ctx.load_texture("egui_img", img, TextureOptions::default());
            self.tex_handle_opt = Some(tex);
        }
    }
}

// test ----------------------------------------------->
#[cfg(test)]
mod tests {

    use eframe::egui;

    use crate::{app::image_application::ImageApplication, domain::types::PasteItem};

    #[test]
    fn test_clipboad() {
        let mut clip = arboard::Clipboard::new().unwrap();
        clip.set_text("contents/").unwrap();
        let mut img_app = ImageApplication::new();
        let past_item: PasteItem = PasteItem::Text(clip.get_text().unwrap());
        img_app.paste(past_item);
    }

    #[test]
    fn test_img_application() {
        let mut img_app = ImageApplication::new();
        img_app.paste(PasteItem::Text("contents/".into()));
        let ctx = egui::Context::default();
        img_app.load_img(&ctx);
        let img_size = img_app.tex_handle_opt.as_ref().unwrap().size();
        println!("next = img_size:{:?}", img_size);
        img_app.next();
        img_app.next();
        img_app.load_img(&ctx);
        let img_size = img_app.tex_handle_opt.as_ref().unwrap().size();
        println!("next = img_size:{:?}", img_size);
        img_app.previous();
        img_app.previous();
        img_app.previous();
        img_app.previous();
        img_app.previous();
        img_app.load_img(&ctx);
        let img_size = img_app.tex_handle_opt.as_ref().unwrap().size();
        println!("next = img_size:{:?}", img_size);
    }
}

// test <-----------------------------------------------
