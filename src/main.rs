use std::time::Instant;

use chrono::{self, format, DateTime, Datelike, Utc};
use eframe::egui;
use egui::RichText;

fn main() {
    let native_options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "My egui App",
        native_options,
        Box::new(|cc| Ok(Box::new(MyEguiApp::new(cc)))),
    );
}

pub trait Update {
    fn update(&self);
}

struct Upgrade {
    bought: bool,
    text: String,
    description: String,
    price: f64,
    effect: fn(&mut MyEguiApp),
}

struct MyEguiApp {
    time: Instant,
    dt: f64,
    cats: [f64; 31],
    cat_multipliers: [f64; 31],
    //  NOTE: Resets every frame, make sure to update every frame even when implementing static multipliers
    cat_prices: [f64; 31],
    cat_price_multipliers: [f64; 31],
    currency: f64,
    colors: [egui::Color32; 1],
    upgrades: Vec<Box<Upgrade>>,
}

impl MyEguiApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        //

        let temp = MyEguiApp {
            time: Instant::now(),
            dt: 0.0,
            cats: [0.0; 31],
            cat_multipliers: [1.0; 31],
            cat_prices: [1.0; 31],
            cat_price_multipliers: [1.5; 31],
            currency: 1.0,
            colors: [egui::Color32::from_hex("#FF784F").unwrap()],
            upgrades: vec![Box::new(Upgrade {
                bought: false,
                text: "Early Bird".to_owned(),
                description: "Gives a boost to cats depending on how early in the month they are"
                    .to_owned(),
                price: 100.0,
                effect: |x| {
                    for i in 0..x.cat_multipliers.len() {
                        x.cat_multipliers[i] -= (i as f64 / 31.0) - 1.0;
                    }
                },
            })],
        };
        temp
    }
}

fn update(app: &mut MyEguiApp, date: DateTime<Utc>) -> (f64, usize) {
    app.cat_multipliers = [1.0; 31];
    let day = date.day0() as usize;
    let mut cps = 0.0;
    for i in 0..app.cats.len() {
        if day == i {
            app.cat_multipliers[i] *= 1.5;
        }
    }
    for i in 0..app.upgrades.len() {
        if app.upgrades[i].bought {
            (app.upgrades[i].effect)(app);
        }
    }

    cps += app
        .cats
        .iter()
        .zip(app.cat_multipliers.iter())
        .map(|(x, y)| x * y)
        .sum::<f64>();
    app.currency += cps * app.dt;
    (cps, day)
}

fn cat_handler(app: &mut MyEguiApp, ui: &mut egui::Ui, day: usize) {
    for i in 0..app.cats.len() {
        ui.horizontal(|ui| {
            if i == day {
                ui.label(
                    RichText::new(format!(
                        "You have {} 'Day {}' cats [{:.2}]",
                        app.cats[i],
                        i + 1,
                        app.cat_multipliers[i]
                    ))
                    .color(app.colors[0]),
                );
            } else {
                ui.label(format!(
                    "You have {} 'Day {}' cats [{:.2}]",
                    app.cats[i],
                    i + 1,
                    app.cat_multipliers[i]
                ));
            }
            if ui
                .add_enabled(
                    app.cat_prices[i] <= app.currency,
                    egui::Button::new(format!("Hire another cat {:.2}", app.cat_prices[i])),
                )
                .clicked()
            {
                app.currency -= app.cat_prices[i];
                if app.cats[i] == 0.0 {
                    for j in 0..app.cat_prices.len() {
                        if i != j && app.cats[j] == 0.0 {
                            app.cat_prices[j] *= 10.0;
                        }
                    }
                }
                app.cats[i] += 1.0;
                app.cat_prices[i] *= app.cat_price_multipliers[i];
            }
        });
    }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let date = Utc::now();
        let (cps, day) = update(self, date);
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("clidle [W.I.P. name]");
            ui.label(format!(
                "You currently have {}$ (+{:.2}$/s)",
                self.currency.round(),
                cps
            ));

            cat_handler(self, ui, day);

            for i in 0..self.upgrades.len() {
                let price = self.upgrades[i].price;
                if ui
                    .add_enabled(
                        price <= self.currency && !self.upgrades[i].bought,
                        egui::Button::new(format!("{} {}$", self.upgrades[i].text, price)),
                    )
                    .on_hover_text(&self.upgrades[i].description)
                    .on_disabled_hover_text(format!(
                        "[{}s] {}",
                        ((price - self.currency) / cps).ceil(),
                        self.upgrades[i].description
                    ))
                    .clicked()
                {
                    self.currency -= self.upgrades[i].price;
                    self.upgrades[i].bought = true;
                }
            }

            // ui.hyperlink("https://github.com/emilk/egui");
            // ui.text_edit_singleline(&mut self.label);
            // if ui.button("Click me").clicked() {}
            // ui.add(egui::Slider::new(&mut self.fps, 0.0..=240.0).prefix("Desired FPS: "));
            // ui.label(format!("Current FPS: {}", (1.0 / self.dt).round()));
            // ui.label(format!("count: {}", self.count));

            // ui.checkbox(&mut false, "Checkbox");

            // ui.horizontal(|ui| {
            //     ui.radio_value(&mut self.num, Enum::First, "First");
            //     ui.radio_value(&mut self.num, Enum::Second, "Second");
            //     ui.radio_value(&mut self.num, Enum::Third, "Third");
            // });

            // ui.separator();

            // ui.collapsing("Click to see what is hidden!", |ui| {
            //     ui.label("Not much, as it turns out");
            // });
        });
        self.dt = self.time.elapsed().as_micros() as f64 * 1e-6;
        self.time = Instant::now();
        ctx.request_repaint();
    }
}
