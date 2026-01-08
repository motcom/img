use crate::domain::types::{PasteItem, ZoomFactor};
use image::RgbImage;

/// 画像ドメインの共通インターフェースを定義するトレイトです。
/// 画像の貼り付け、取得、切り替え、種別取得などの操作を提供します。
pub trait ImageDomainTrait {
    /// 画像やテキストを貼り付けます。
    fn pasete(&mut self, paste_item: PasteItem);

    /// 現在選択されている画像を取得します。
    fn get_image(&mut self) -> Option<&RgbImage>;

    /// 次の画像に切り替えます。
    fn next(&mut self);

    /// 前の画像に切り替えます。
    fn previous(&mut self);

    fn get_curimage_factor(&self) -> ZoomFactor;
    fn set_curimage_factor(&mut self, zoom_factor: ZoomFactor);

    fn get_samnails(&self, samnail_cur_index: usize, samnail_nums_per_page: usize)
    -> Vec<RgbImage>;
    fn get_image_nums(&self) -> usize;
}
