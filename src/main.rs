use std::io::BufRead ;
use std::path::PathBuf;
use atty;
use clap;
use eframe::{self};
use egui::{self, ColorImage};
use image::{self, GenericImageView};


// メイン関数
fn main() {
      debug_exe();
}

fn debug_exe() {
   eframe::run_native(
      "my_app",
      eframe::NativeOptions::default(),
      Box::new(|cc| Ok(Box::new(MyApp::new(cc)))),
   ).expect("Failed to run eframe");
 }


// 画像ビューワー構造体 -----------------------------------
impl eframe::App for MyApp {
   fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

      egui::CentralPanel::default().show(ctx, |ui| {
         if let Some(tex) = &self.tex_handle{
            ui.image(tex);
         }else{
            self.load_image(ctx,"../../../SamplePicture/image-1.jpg");
         }
      });

      // 終了処理
      ctx.request_repaint_after(std::time::Duration::from_millis(33));
   }

}


struct MyApp {
   tex_handle: Option<egui::TextureHandle>,
   last_window_size:Option<usize>,
}


impl MyApp {
   fn new(_cc: &eframe::CreationContext<'_>) -> Self {
      Self {
         tex_handle: None,
         last_window_size:None,
      }
   }

   fn load_image(&mut self,ctx:&egui::Context,path:&str){
      let img = match image::open(path) {
         Ok(img) => img,
         Err(_) => {println!("画像は読み込めませんでした{}",path);
            return;
         }
      };
      // サイズを取得
      let (width,height) = img.dimensions();
      let size = [width as usize,height as usize];
      
      // RGBA8 フォーマットに変換
      let rgba_image = img.to_rgba8();
      // egui::ColorImage を作成
      let color_image = 
         egui::ColorImage::from_rgba_unmultiplied(size,&rgba_image);

      // RetainedImageに変換
      self.tex_handle = Some(ctx.load_texture("image", color_image, egui::TextureOptions::default()));
   }
}

//--------------------------------------------------------------

/// 入力からファイルパスを取得する
///
/// return: ファイルパスのリスト
fn input_to_pathes() -> Vec<PathBuf>{
   if atty::isnt(atty::Stream::Stdin) {
      let file_path_str = std::io::stdin().lock().lines().map(|line| {
         let line = line.expect("Failed to read line");
         PathBuf::from(line)
      }).filter(|path|path.exists()).collect();
      file_path_str
   } else {
      let matches = clap::Command::new("img")
         .version("0.1.0")
         .author("mo")
         .about("simple img viewer")
         .arg(clap::Arg::new("input_img_path").required(true))
         .get_matches();

      // コマンドライン引数のパース
      let file_path_str:Vec<PathBuf> = matches.get_one::<String>("input_img_path")
         .expect("ファイルパスがありません")
         .split(',')
         .map(|s| PathBuf::from(s.to_string()))
         .filter(|path| path.exists())
         .collect();
      file_path_str
   }
}
