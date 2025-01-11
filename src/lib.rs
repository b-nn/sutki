use std::time::Instant;

use chrono::{self, DateTime, Datelike, Duration, NaiveTime, Utc};
use eframe::egui;
use egui::RichText;

// fn main() {
//     let native_options = eframe::NativeOptions::default();
//     let _ = eframe::run_native(
//         "My egui App",
//         native_options,
//         Box::new(|cc| Ok(Box::new(MyEguiApp::new(cc)))),
//     );
// }

pub trait Update {
    fn update(&self);
}

struct Upgrade {
    text: String,
    description: String,
    price: f64,
    price_mult: f64,
    max: i64,
    count: i64,
    effect: fn(&mut MyEguiApp, i64),
}

pub struct MyEguiApp {
    time: Instant,
    dt: f64,
    cats: [f64; 31],
    cat_multipliers: [f64; 31],
    day_offset: f64,
    day_width: i64,
    //  NOTE: Resets every frame, make sure to update every frame even when implementing static multipliers
    cat_prices: [f64; 31],
    cat_price_multipliers: [f64; 31],
    cat_times: [f64; 31],
    currency: f64,
    colors: [egui::Color32; 1],
    upgrades: Vec<Box<Upgrade>>,
}

impl MyEguiApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
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
            day_offset: 1.0,
            day_width: 0,
            currency: 1.0,
            cat_times: [0.0; 31],
            colors: [egui::Color32::from_hex("#FF784F").unwrap()],
            upgrades: vec![
                Box::new(Upgrade {
                    text: "Early Bird".to_owned(),
                    description:
                        "Gives a boost to cats depending on how early in the month they are"
                            .to_owned(),
                    price: 100.0,
                    max: 1,
                    price_mult: 1.0,
                    count: 0,
                    effect: |x, _y| {
                        for i in 0..x.cat_multipliers.len() {
                            x.cat_multipliers[i] += 1.036_f64.powi(31 - i as i32);
                        }
                        x.upgrades[1].price = 500.0;
                    },
                }),
                Box::new(Upgrade {
                    text: "Late Bird".to_owned(),
                    description:
                        "Gives a boost to cats depending on how late in the month they are"
                            .to_owned(),
                    price: 100.0,
                    price_mult: 1.0,
                    max: 1,
                    count: 0,
                    effect: |x, _y| {
                        for i in 0..x.cat_multipliers.len() {
                            x.cat_multipliers[i] += 1.036_f64.powi(i as i32);
                        }
                        x.upgrades[0].price = 500.0;
                    },
                }),
                Box::new(Upgrade {
                    text: "Faster Spin".to_owned(),
                    description: "Makes the 'Extra Effective' boost cycle through 50% faster"
                        .to_owned(),
                    price: 200.0,
                    price_mult: 1.4,
                    max: 30,
                    count: 0,
                    effect: |x, y| {
                        x.day_offset += x.dt * (2_f64.powi(y as i32) - 1.0);
                    },
                }),
                Box::new(Upgrade {
                    text: "..Wider Spin?".to_owned(),
                    description:
                        "Makes an additional cat get the 'Extra Effective' boost at the same time"
                            .to_owned(),
                    price: 1000.0,
                    price_mult: 1.4,
                    max: 15,
                    count: 0,
                    effect: |x, y| {
                        x.day_width = y;
                    },
                }),
                Box::new(Upgrade {
                    text: "Cat Synergy".to_owned(),
                    description: "Buying cats increases the base production of all other cats"
                        .to_owned(),
                    price: 10000.0,
                    price_mult: 1.4,
                    max: 1,
                    count: 0,
                    effect: |x, _y| {
                        for i in 0..x.cat_multipliers.len() {
                            x.cat_multipliers[i] *=
                                x.cats.iter().enumerate().map(|(x, y)| if x == i  {
                                    0.0
                                } else{*y * 0.01}).sum::<f64>() + 1.0;
                        }
                    },
                }),
                Box::new(Upgrade {
                    text: "Like Hot Cakes".to_owned(),
                    description: "Gives a temporary boost to cats when they get the 'Extra Effective' boost which falls off over time"
                        .to_owned(),
                    price: 10000.0,
                    price_mult: 1.4,
                    max: 1,
                    count: 0,
                    effect: |x, _y| {
                        for i in 0..x.cat_multipliers.len() {
                            if x.cat_times[i] < 0.0 { continue; }
                            x.cat_multipliers[i] *= 1.2f64.powf(5.0 - x.cat_times[i]) + 1.0;
                        }
                    },
                }),
            ],
        };
        temp
    }
}

fn update(app: &mut MyEguiApp, date: DateTime<Utc>) -> (f64, f64) {
    app.cat_multipliers = [1.0; 31];
    let day = date.day0() as f64;
    let mut cps = 0.0;
    for i in 0..app.upgrades.len() {
        if app.upgrades[i].count > 0 {
            (app.upgrades[i].effect)(app, app.upgrades[i].count);
        }
    }
    for i in 0..app.cats.len() {
        if within_day_range(day, app.day_width as f64, i as f64) {
            app.cat_multipliers[i] *= 1.5;
            app.cat_times[i] += app.dt;
        } else {
            app.cat_times[i] = -0.05;
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

fn within_day_range(day: f64, width: f64, i: f64) -> bool {
    if day + width < 31.0 {
        if i >= day && i <= day + width {
            true
        } else {
            false
        }
    } else {
        if (i >= day || i <= (day + width).rem_euclid(31.0)) {
            true
        } else {
            false
        }
    }
}

fn cat_handler(app: &mut MyEguiApp, ui: &mut egui::Ui, day: f64) {
    let mut count = 0;
    for i in 0..app.cats.len() {
        ui.horizontal(|ui| {
            if within_day_range(day, app.day_width as f64, i as f64) {
                count += 1;
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
                            app.cat_prices[j] *= 5.0;
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
        let date = Utc::now() + Duration::seconds(self.day_offset as i64);
        let (cps, day) = update(self, date);
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("clidle [W.I.P. name]");
            ui.label(format!(
                "You currently have {}$ (+{:.2}$/s)",
                self.currency.round(),
                cps
            ));
            let tomorrow_midnight = (date + Duration::days(1))
                .with_time(NaiveTime::from_hms_opt(0, 0, 0).unwrap())
                .unwrap();

            ui.label(format!(
                "{} seconds until tomorrow.",
                (tomorrow_midnight - date).num_seconds(),
            ));

            cat_handler(self, ui, day);

            for i in 0..self.upgrades.len() {
                let price = self.upgrades[i].price;
                if ui
                    .add_enabled(
                        price <= self.currency && self.upgrades[i].count < self.upgrades[i].max,
                        egui::Button::new(format!(
                            "{} {:.2}$ [{}/{}]",
                            self.upgrades[i].text,
                            price,
                            self.upgrades[i].count,
                            self.upgrades[i].max
                        )),
                    )
                    .on_hover_text(&self.upgrades[i].description)
                    .on_disabled_hover_text(format!(
                        "[{}s, x{}] {}",
                        ((price - self.currency) / cps).ceil(),
                        self.upgrades[i].price_mult,
                        self.upgrades[i].description
                    ))
                    .clicked()
                {
                    self.currency -= self.upgrades[i].price;
                    self.upgrades[i].price *= self.upgrades[i].price_mult;
                    self.upgrades[i].count += 1;
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
