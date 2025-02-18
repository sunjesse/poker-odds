use eframe::egui;
use poker_odds_backend::solve;

fn main() -> eframe::Result {
    env_logger::init();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([720.0, 480.0]),
        ..Default::default()
    };
    eframe::run_native(
        "NLH Poker Equity Calculator",
        options,
        Box::new(|_| Ok(Box::<MyApp>::default())),
    )
}

struct MyApp {
    nplayers: usize,
    board: String,
    equity: Option<f32>,
    hands: Vec<String>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            nplayers: 2,
            board: "".to_string(),
            equity: None,
            hands: Vec::from(["".to_string(), "".to_string()]),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("NLH Poker Equity Calculator");
            ui.add(egui::Slider::new(&mut self.nplayers, 2..=10).text("# players:"));

            while self.hands.len() < self.nplayers {
                self.hands.push("".to_string());
            }
            while self.hands.len() > self.nplayers {
                self.hands.pop();
            }

            for i in 0..self.nplayers {
                ui.horizontal(|ui| {
                    let label = if i == 0 {
                        "Your Hand: "
                    } else {
                        "Opponent Hand: "
                    };
                    let name_label = ui.label(label);
                    ui.text_edit_singleline(&mut self.hands[i])
                        .labelled_by(name_label.id);
                });
            }

            ui.horizontal(|ui| {
                let name_label = ui.label("Board: ");
                ui.text_edit_singleline(&mut self.board)
                    .labelled_by(name_label.id);
            });

            if ui.button("Solve").clicked() {
                self.equity = Some(solve(self.hands.clone(), self.board.clone()));
            }
            if let Some(equity) = self.equity {
                ui.label(format!("Your hand's equity is: {:?}", equity));
            }
        });
    }
}
