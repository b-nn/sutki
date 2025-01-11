use chrono::{self, DateTime, Datelike, Duration, Local, NaiveTime, Utc};
use eframe::egui;
use egui::RichText;
use log::log;

// fn main() {
//     let native_options = eframe::NativeOptions::default();
//     let _ = eframe::run_native(
//         "My egui App",
//         native_options,
//         Box::new(|cc| Ok(Box::new(MyEguiApp::new(cc)))),
//     );
// }

pub mod upgrades;
use upgrades::{get_upgrades, Upgrade};

pub trait Update {
    fn update(&self);
}

pub struct MyEguiApp {
    time: DateTime<Local>,
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

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct SaveStruct {
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
    upgrades: Vec<(String, i64)>,
}

impl Default for SaveStruct {
    fn default() -> Self {
        Self {
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
            upgrades: vec![],
        }
    }
}

impl Default for MyEguiApp {
    fn default() -> Self {
        Self {
            time: Local::now(),
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
            upgrades: upgrades::get_upgrades(),
        }
    }
}

impl MyEguiApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.

        if let Some(storage) = cc.storage {
            let t: SaveStruct = eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();

            let default_upgrades = get_upgrades();

            let mut final_upgrades = vec![];
            for i in default_upgrades {
                let mut upgrade = i;
                for j in &t.upgrades {
                    if &j.0 == &upgrade.text {
                        upgrade.count = j.1;
                        upgrade.price = upgrade.price * upgrade.price_mult.powi(j.1 as i32);
                        break;
                    }
                }
                final_upgrades.push(upgrade);
            }
            MyEguiApp {
                time: Local::now(),
                dt: t.dt,
                cats: t.cats,
                cat_multipliers: t.cat_multipliers,
                day_offset: t.day_offset,
                day_width: t.day_width,
                cat_prices: t.cat_prices,
                cat_price_multipliers: t.cat_price_multipliers,
                cat_times: t.cat_times,
                currency: t.currency,
                colors: t.colors,
                upgrades: final_upgrades,
            }
        } else {
            Default::default()
        }
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
                    egui::Button::new(format!("Hire another cat {:.2}$", app.cat_prices[i])),
                )
                .on_hover_text(format!(
                    "x{} to self, x5 to all other unbought cats",
                    app.cat_price_multipliers[i],
                ))
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
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        let t = SaveStruct {
            dt: self.dt,
            cats: self.cats,
            cat_multipliers: self.cat_multipliers,
            day_offset: self.day_offset,
            day_width: self.day_width,
            cat_prices: self.cat_prices,
            cat_price_multipliers: self.cat_price_multipliers,
            cat_times: self.cat_times,
            currency: self.currency,
            colors: self.colors,
            upgrades: self
                .upgrades
                .iter()
                .map(|x| (x.text.to_owned(), x.count))
                .collect(),
        };
        eframe::set_value(storage, eframe::APP_KEY, &t);
        println!("saved!");
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_theme_preference_buttons(ui);
            });
        });

        let date = Utc::now() + Duration::seconds(self.day_offset as i64);
        let (cps, day) = update(self, date);
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("sutki [W.I.P. name]");
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

            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.set_min_width(330.0);

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
            });

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
        self.dt = (Local::now() - self.time).num_microseconds().unwrap() as f64 * 1e-6;
        self.time = Local::now();
        ctx.request_repaint();
    }
}
