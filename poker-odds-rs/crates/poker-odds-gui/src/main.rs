use eframe::egui;

fn main() -> eframe::Result {
    env_logger::init();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1080.0, 720.0]),
        ..Default::default()
    };
    eframe::run_native(
        "NLH Poker Equity Calculator",
        options,
        Box::new(|_| {
            Ok(Box::<MyApp>::default())
        }),
    )
}

struct MyApp {
    hero_hand: String,
    nplayers: u32,
    equity: Option<f32>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            hero_hand: "AcAd".to_owned(),
            nplayers: 2,
            equity: None,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("NLH Poker Equity Calculator");
            ui.add(egui::Slider::new(&mut self.nplayers, 2..=10).text("# players:"));
            for i in 0..self.nplayers {
                ui.horizontal(|ui| {
                    let label = if i == 0 { "Your Hand: " } else { "Opponent Hand: " };
                    let name_label = ui.label(label); 
                    ui.text_edit_singleline(&mut self.hero_hand)
                        .labelled_by(name_label.id);
                });
            }
            if ui.button("Increment").clicked() {
                self.nplayers += 1;
            }
            ui.label(format!("Your hand '{}', nplayers {}", self.hero_hand, self.nplayers));
        });
    }
}
