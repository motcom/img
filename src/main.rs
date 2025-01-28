use std::io::BufRead ;
use std::path::PathBuf;
use atty;
use clap;
use eframe::{self};
use egui;
use image;


// メイン関数
fn main() {
      debug_exe();
}

fn debug_exe() {
   eframe::run_native(
      "my_app",
      eframe::NativeOptions::default(),
      Box::new(|_| Ok(Box::new(MyApp::new( )))),
   ).expect("Failed to run eframe");
 }


// 画像ビューワー構造体 -----------------------------------
impl eframe::App for MyApp {
   fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
      egui::CentralPanel::default().show(ctx, |ui| {

         // 画像の表示
         if let Some(tex_id) = self.tex_id {
            ui.add(
               egui::Image::new(
               (self.tex_id.expect("tex_not_found"),
                  self.tex_size.expect("size_not_found"))
         ));
            
         }else {
            let path = "../../../sample_images/image-1.jpg";
            let img =image::open(path).expect("画像を開けません");
            self.tex_id = Some(img);
            self.tex_size = Some(
               egui::Vec2::new(
                  img.width() as f32 
                  ,img.height() as f32));

         }

      });
   }
}

struct MyApp {
   tex_id: Option<egui::TextureId>,
   tex_size:Option<egui::Vec2>,
}


impl MyApp {
   fn new() -> Self {
      Self {
         tex_id: None,
         tex_size:None,
      }
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
