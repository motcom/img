use clap;
use eframe;
use image;

fn main() {
    // コマンドライン引数のパース
    let matches = clap::App::new("img")
        .version("0.1.0")
        .author("mot")
        .about("simple img viewer")
        .arg(
            clap::Arg::with_name("input_img_path")
                .help("Input image file")
                .required(true)
                .index(1),
        )
        .get_matches();

    // コマンドライン引数から画像ファイルのパスを取得
    let input_img_path = if let Some(path) = matches.value_of("input_img_path") {
        path
    } else {
        eprintln!("input_img_path is required");
        exit(1);
    };

    // eframを使ってウィンドウを作成
    eframe::run_native(
        "my_app",
        eframe::NativeOptions::default(),
        Box::new(|_| Ok(Box::new(MyApp::new()))),
    )
    .expect("Failed to run eframe");
}

struct MyApp {
    texId: egui::TextureId,
    tex_size: (usize, usize),
}

impl MyApp {
    fn new() -> Self {
        Self
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add(egui::Checkbox::new(&mut self.checked, "Check me"));
        });
    }
}
