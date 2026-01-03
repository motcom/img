use crate::domain::types::{ImageTraitKind, PasteItem};
use eframe::egui::ColorImage;

pub trait ImageDomainTrait {
    fn next(&mut self);
    fn previous(&mut self);
    fn pasete(&mut self, paste_item: PasteItem);
    fn get_image(&mut self) -> Option<&ColorImage>;
    fn kind(&self) -> ImageTraitKind;
}
