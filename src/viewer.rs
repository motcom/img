use std::io::BufRead ;
use std::path::PathBuf;
use std::path::Path;
use atty;
use clap;
use eframe::{self};
use egui::ViewportBuilder;
use egui::{self, ColorImage};
use image;

// Data 構造----------------------------------------------------------------------------------
struct LabeledColor {
   _path:PathBuf,
   size:(usize,usize),
   color:egui::ColorImage
}

pub struct MyViwer {
   // テクスチャハンドル
   tex_handle_lst: Vec<LabeledColor>,
   cur_index:   usize,
}
// Data 構造----------------------------------------------------------------------------------

impl MyViwer {
   // 実行イベントループをする前の初期設定もここで行う
   pub fn exe(&mut self) {
      let max_size = self.get_maxsize();

      let native_opt = eframe::NativeOptions {
          viewport:ViewportBuilder::default().with_inner_size([max_size.0 as f32,max_size.1 as f32]),
         ..Default::default()
      };

      // イベントループの生成
      eframe::run_native(
         "img",
         native_opt,
         Box::new(|_| Ok(Box::new(MyViwer::new()))),
      ).expect("Failed to run eframe");
   }

   fn get_maxsize(&mut self)->(usize,usize) {
      let max_width = self.tex_handle_lst.iter().map(|x| x.size.0).max().unwrap_or(0);
      let max_height = self.tex_handle_lst.iter().map(|y| y.size.1).max().unwrap_or(0);
      (max_width,max_height)
   }

   // コンストラクタ
   pub fn new() -> Self {
      let mut tex_handle_lst = Vec::<LabeledColor>::new();
      let pathes = MyViwer::input_to_pathes();
      let image_extensions = vec![
        "avif", "bmp", "dds", "farbfeld", "gif", "hdr", "ico","jpg",
        "jpeg", "exr", "png", "pnm", "qoi", "tga", "tiff", "webp"
      ];

      // tex_handle_lstの構築
      for path in pathes {
         // extension が画像じゃなければ無視
         match path.extension().and_then(|s| s.to_str())
         {
            Some(p) => {if !image_extensions.contains(&p) {
                  continue;
               }
            },
            None=>{
               continue}
         }
         // パスから画像データを取得
         let col_img_opt = MyViwer::path_to_tex_handle(&path);

         // リストに入れる
         if let Some(col_img) = col_img_opt {
            tex_handle_lst.push(LabeledColor{
               _path: path ,
               size:(col_img.width(),col_img.height()),
               color:col_img,
            });
         }
      }

      Self {
         tex_handle_lst,
        cur_index:   0,
      }
   }


   // Event -------------------------------------------------------
   // 次へ
   fn next(&mut self) {
      self.cur_index += 1;
      if self.cur_index >= self.tex_handle_lst.len(){
         self.cur_index = 0;
      }
   }

   // 後ろへ
   fn back(&mut self) {
      if self.tex_handle_lst.len() == 0{
         println!("image list 0");
         return; 
      }

      if self.cur_index == 0 {
         self.cur_index = self.tex_handle_lst.len() -1;
      }else{
         self.cur_index -= 1;
      }
   }

   // utility ---------------パスからテクスチャハンドルへ変更する
   fn path_to_tex_handle(path:&PathBuf) -> Option<ColorImage>{
            let img =image::open(path);
            match img
         {
         Ok(img) => {
            let img = img.to_rgba8();
            let (width,height) = img.dimensions();
            let pixels = img.into_raw();
            Some(egui::ColorImage::from_rgba_unmultiplied([width as usize,height as usize], &pixels))
         }
         Err(_) => {None}
      }
   }
   
   // path をinputする
   fn input_to_pathes() -> Vec<PathBuf>{
      if atty::isnt(atty::Stream::Stdin) {

         let file_path_str = std::io::stdin().lock().lines().map(|line| {
            let line = line.expect("Failed to read line");
            PathBuf::from(line)
         }).filter(|path|path.exists()).collect();
         file_path_str

         } else {

         // コマンドライン引数の設定
         let matches = clap::Command::new("img")
            .version("0.1.0")
            .author("mo")
            .about("simple img viewer")
            .arg(clap::Arg::new("input_img_path").required(true))
            .get_matches();
         
         // ディレクトリかどうか
         if let Some(path_str) = matches.get_one::<String>("input_img_path"){
            let path = Path::new(path_str);
            let mut file_path_str:Vec<PathBuf> = Vec::new();
            if path.is_dir() && path.exists(){
               for p in path.read_dir().expect("Failed to change directory.") {
                  if let Ok(p) = p {
                     file_path_str.push(p.path());
                  }
               }
            }
            return file_path_str;
         }

         // コマンドライン引数のパース
         let file_path_str:Vec<PathBuf> = matches.get_one::<String>("input_img_path")
            .expect("file path not found")
            .split(',')
            .map(|s| PathBuf::from(s.to_string()))
            .filter(|path| path.exists())
            .collect();
         file_path_str
      }
   }
}

// event loop -----------------------------------------------------------------------------------------------------------
impl eframe::App for MyViwer {
   fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
      // Input
      ctx.input(|i|{
         // next
         if i.key_pressed(egui::Key::L){
            println!("next");
            self.next();
         }

         // back 
         if i.key_pressed(egui::Key::H) {
            println!("back");
            self.back();
         }

      });

      // Render
      egui::CentralPanel::default().show(ctx, |ui| { 
         if self.tex_handle_lst.len() != 0{
            let color_img = self.tex_handle_lst.get(self.cur_index as usize)
               .expect("Unable to get images from the list."); 

            // 画像の表示
            let tex_handle 
               = ctx.load_texture("tst", color_img.color.clone(), egui::TextureOptions::default());
            ui.image(&tex_handle);
         } 
      });

      // ループ遅延
      ctx.request_repaint_after(std::time::Duration::from_millis(33)); 
   }
}

