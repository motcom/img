use std::io::BufRead ;
use std::path::PathBuf;
use atty;
use clap;
use eframe::{self};
use egui::{self, ColorImage};
use image;

pub struct MyViwer {
   // テクスチャハンドル
   tex_handle_lst: Vec<egui::ColorImage>,
   cur_index:   isize,
}

impl MyViwer {
   pub fn exe(&mut self) {
      eframe::run_native(
         "my_app",
         eframe::NativeOptions::default(),
         Box::new(|_| Ok(Box::new(MyViwer::new()))),
      ).expect("Failed to run eframe");
   }

   // コンストラクタ
   pub fn new() -> Self {
      let mut tex_handle_lst = Vec::<egui::ColorImage>::new();
      let pathes = MyViwer::input_to_pathes();
      println!("{:?}",pathes);

      for path in pathes {
         tex_handle_lst.push(MyViwer::path_to_tex_handle(path));
      }

      Self {
        tex_handle_lst,
        cur_index:      0,
      }
   }
   // パスからテクスチャハンドルへ変更する
   fn path_to_tex_handle(path:PathBuf) -> ColorImage{
            let img =image::open(path).expect(&format!("{}このパスからimageモジュールから画像を開けません","path"));
            let img = img.to_rgba8();
            let (width,height) = img.dimensions();
            let pixels = img.into_raw();
            egui::ColorImage::from_rgba_unmultiplied([width as usize,height as usize], &pixels)
   }

   fn next(&mut self) {
      self.cur_index += 1;
      if self.cur_index >= self.tex_handle_lst.len() as isize{
         self.cur_index = 0;
      }
   }
   fn back(&mut self) {
      self.cur_index -= 1;
      if self.cur_index <= 0 {
         self.cur_index = (self.tex_handle_lst.len() -1) as isize;
      }
   }

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
}

// イベントループ ---------------------------------
impl eframe::App for MyViwer {
   fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

      // Input
      ctx.input(|i|{
         // next
         if i.key_pressed(egui::Key::L){
            self.next();
         }

         // back 
         if i.key_pressed(egui::Key::H) {
            self.back();
         }

      });

      // Render
      egui::CentralPanel::default().show(ctx, |ui| { 
         if self.tex_handle_lst.len() != 0{
            let color_img = self.tex_handle_lst.get(self.cur_index as usize)
               .expect("リストから画像を取得できませんでした"); 

            // 画像の表示
            let tex_handle 
               = ctx.load_texture("tst", color_img.clone(), egui::TextureOptions::default());
            ui.image(&tex_handle);
         } 
      });

      // ループ遅延
      ctx.request_repaint_after(std::time::Duration::from_millis(33)); 
   }
}

