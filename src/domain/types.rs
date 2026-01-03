use eframe::egui::ColorImage;

pub enum PasteItem {
    Text(String),
    Image(ColorImage),
}

pub enum ImageTraitKind {
    Text,
    Image,
}
