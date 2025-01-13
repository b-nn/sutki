use chrono::{self, DateTime, Datelike, Duration, Local, NaiveTime, Utc};
use eframe::egui;
use egui::{debug_text::print, RichText};
use log::{log, Level};

// fn main() {
//     let native_options = eframe::NativeOptions::default();
//     let _ = eframe::run_native(
//         "My egui App",
//         native_options,
//         Box::new(|cc| Ok(Box::new(Game::new(cc)))),
//     );
// }

pub mod upgrades;
use upgrades::{get_upgrades, Upgrade};
pub mod cats;

pub trait Update {
    fn update(&self);
}

#[derive(PartialEq)]
enum Tab {
    Cats,
    Upgrades,
    Settings,
}

const TABS: [(&str, Tab); 3] = [
    ("Cats", Tab::Cats),
    ("Upgrades", Tab::Upgrades),
    ("Settings", Tab::Settings),
];

pub struct Game {
    real_time: DateTime<Local>,
    dt: f64,
    cats: [f64; 31],
    cat_multipliers: [f64; 31],
    day_offset: f64,
    day_width: u32,
    day: u32,
    date: DateTime<Utc>,
    //  NOTE: Resets every frame, make sure to update every frame even when implementing static multipliers
    cat_prices: [f64; 31],
    cat_price_multipliers: [f64; 31],
    cat_price_5_multiplier: [f64; 31],
    cat_times: [f64; 31],
    currencies: [f64; 2],
    colors: [egui::Color32; 1],
    upgrades: Vec<Upgrade>,
    cat_strawberries: [i64; 31],
    cat_strawberry_prices: [i64; 31],
    unlocked_tiers: [bool; 2],
    status: String,
    status_time: DateTime<Local>,
    currency_symbols: [char; 2],
    asleep: bool,
    cps: f64,
    state: Tab,
}

fn change_status(level: Level, message: &str, status: &mut String, time: &mut DateTime<Local>) {
    *status = message.to_owned();
    *time = Local::now();
    log!(level, "{}", message);
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct SaveStruct {
    cats: [f64; 31],
    day_offset: f64,
    day_width: u32,
    //  NOTE: Resets every frame, make sure to update every frame even when implementing static multipliers
    cat_prices: [f64; 31],
    cat_times: [f64; 31],
    currencies: [f64; 2],
    upgrades: Vec<(String, i64, i64)>,
    cat_strawberries: [i64; 31],
    cat_strawberry_prices: [i64; 31],
    unlocked_tiers: [bool; 2],
    cat_price_5_multiplier: [f64; 31],
}

impl Default for SaveStruct {
    fn default() -> Self {
        Self {
            cats: [0.0; 31],
            cat_prices: [1.0; 31],
            day_offset: 1.0,
            day_width: 0,
            currencies: [1.0, 0.0],
            cat_times: [0.0; 31],
            upgrades: vec![],
            cat_strawberries: [0; 31],
            cat_strawberry_prices: [1; 31],
            unlocked_tiers: [true, false],
            cat_price_5_multiplier: [0.0; 31],
        }
    }
}

impl Default for Game {
    fn default() -> Self {
        Self {
            real_time: Local::now(),
            dt: 0.0,
            cats: [0.0; 31],
            cat_multipliers: [1.0; 31],
            cat_prices: [1.0; 31],
            cat_price_multipliers: [1.5; 31],
            day_offset: 1.0,
            day_width: 0,
            day: Local::now().day0(),
            currencies: [1.0, 0.0],
            cat_times: [0.0; 31],
            colors: [egui::Color32::from_hex("#FF784F").unwrap()],
            upgrades: upgrades::get_upgrades(),
            cat_strawberries: [0; 31],
            cat_strawberry_prices: [1; 31],
            unlocked_tiers: [true, false],
            status: "Opened game".to_owned(),
            status_time: Local::now(),
            currency_symbols: ['$', '🍓'],
            cat_price_5_multiplier: [0.0; 31],
            asleep: false,
            cps: 0.0,
            date: Utc::now(),
            state: Tab::Cats,
        }
    }
}

impl Game {
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
                    if &j.0 != &upgrade.text {
                        continue;
                    }
                    if &j.2 != &upgrade.max {
                        continue;
                    }
                    upgrade.count = j.1;
                    upgrade.price = upgrade.price * upgrade.price_mult.powi(j.1 as i32);
                    break;
                }
                final_upgrades.push(upgrade);
            }
            Game {
                cats: t.cats,
                day_offset: t.day_offset,
                day_width: t.day_width,
                cat_prices: t.cat_prices,
                cat_times: t.cat_times,
                currencies: t.currencies,
                upgrades: final_upgrades,
                cat_strawberries: t.cat_strawberries,
                cat_strawberry_prices: t.cat_strawberry_prices,
                unlocked_tiers: t.unlocked_tiers,
                status: "Opened game".to_owned(),
                status_time: Local::now(),
                cat_price_5_multiplier: t.cat_price_5_multiplier,
                ..Default::default()
            }
        } else {
            Default::default()
        }
    }
}

fn update(app: &mut Game) -> f64 {
    app.cat_multipliers = [1.0; 31];
    app.cat_prices = [1.0; 31];
    let mut cps = 0.0;
    for i in 0..app.upgrades.len() {
        if app.upgrades[i].count > 0 {
            (app.upgrades[i].effect)(app, app.upgrades[i].count);
        }
    }
    for i in 0..app.cats.len() {
        app.cat_prices[i] = if app.asleep {
            1.45_f64.powf(app.cats[i]) * 2.1_f64.powi(app.cat_price_5_multiplier[i] as i32)
        } else {
            1.5_f64.powf(app.cats[i]) * 5_f64.powi(app.cat_price_5_multiplier[i] as i32)
        };
        if within_day_range(app.day, app.day_width, i as u32) && !app.asleep {
            app.cat_multipliers[i] *= 1.5;
            app.cat_times[i] += app.dt;
        } else {
            app.cat_times[i] = -0.00001;
        }
        app.cat_multipliers[i] *= 1.5f64.powi(app.cat_strawberries[i] as i32);
    }

    cps += app
        .cats
        .iter()
        .zip(app.cat_multipliers.iter())
        .map(|(x, y)| x * y)
        .sum::<f64>();
    app.currencies[0] += cps * app.dt;
    cps
}

fn within_day_range(day: u32, width: u32, i: u32) -> bool {
    if day + width < 31 {
        if i >= day && i <= day + width {
            true
        } else {
            false
        }
    } else {
        if i >= day || i <= (day + width).rem_euclid(31) {
            true
        } else {
            false
        }
    }
}

fn cat_handler(app: &mut Game, ui: &mut egui::Ui) {
    let mut count = 0;
    for i in 0..app.cats.len() {
        ui.horizontal(|ui| {
            if within_day_range(app.day, app.day_width, i as u32) && !app.asleep {
                count += 1;
                ui.label(
                    RichText::new(format!(
                        "You have {} 'Day {}' cats [{:.2}]",
                        app.cats[i],
                        i + 1,
                        app.cat_multipliers[i]
                    ))
                    .color(app.colors[0]),
                )
                .on_hover_text("This cat is Extra Effective!");
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
                    app.cat_prices[i] <= app.currencies[0],
                    egui::Button::new(format!("Hire another cat {:.2}$", app.cat_prices[i])),
                )
                .on_hover_text(format!(
                    "x{} to self, x5 to all other unbought cats",
                    app.cat_price_multipliers[i],
                ))
                .clicked()
            {
                app.currencies[0] -= app.cat_prices[i];
                if app.cats[i] == 0.0 {
                    for j in 0..app.cat_prices.len() {
                        if i != j && app.cats[j] == 0.0 {
                            app.cat_price_5_multiplier[j] += 1.0;
                        }
                    }
                }
                app.cats[i] += 1.0;
            }

            if app.unlocked_tiers[1] {
                if ui
                    .add_enabled(
                        app.currencies[1] >= app.cat_strawberry_prices[i].pow(2) as f64,
                        egui::Button::new(format!(
                            "Feed cat {} strawberry",
                            app.cat_strawberry_prices[i].pow(2)
                        )),
                    )
                    .clicked()
                {
                    app.currencies[1] -= app.cat_strawberry_prices[i].pow(2) as f64;
                    app.cat_strawberries[i] += 1;
                    app.cat_strawberry_prices[i] += 1;
                }
            }
        });
    }
}

impl eframe::App for Game {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        let t = SaveStruct {
            cats: self.cats,
            day_offset: 0.0,
            day_width: self.day_width,
            cat_prices: self.cat_prices,
            cat_times: self.cat_times,
            currencies: self.currencies,
            upgrades: self
                .upgrades
                .iter()
                .map(|x| (x.text.to_owned(), x.count, x.max))
                .collect(),
            cat_strawberries: self.cat_strawberries,
            cat_strawberry_prices: self.cat_strawberry_prices,
            unlocked_tiers: self.unlocked_tiers,
            cat_price_5_multiplier: self.cat_price_5_multiplier,
        };
        eframe::set_value(storage, eframe::APP_KEY, &t);
        change_status(
            Level::Info,
            "Saved!",
            &mut self.status,
            &mut self.status_time,
        );
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                ui.menu_button("File", |ui| {
                    if !is_web {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    }
                    if ui.button("Reset").clicked() {
                        *self = Game::default();
                    }
                    if ui.button("money (for testing purposes)").clicked() {
                        self.currencies[0] += 10000.0;
                    }
                });
                ui.add_space(16.0);

                egui::widgets::global_theme_preference_buttons(ui);

                ui.add_space(16.0);
                for i in TABS {
                    if ui.selectable_label(i.1 == self.state, i.0).clicked() {
                        self.state = i.1;
                    }
                }

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(format!(
                        "{} {}s ago",
                        self.status,
                        (Local::now() - self.status_time).num_seconds(),
                    ));
                });
            });
        });

        self.date = Utc::now() + Duration::seconds(self.day_offset as i64);
        self.day = (Utc::now() + Duration::seconds(self.day_offset as i64)).day0();
        self.cps = update(self);
        let central = egui::CentralPanel::default();
        central.show(ctx, |ui| match self.state {
            Tab::Cats => {
                cats::update(self, ui);
            }
            Tab::Upgrades => upgrades::update(self, ui),
            Tab::Settings => {}
        });

        // cats::update(self, ctx, central);

        self.dt = (Local::now() - self.real_time).num_microseconds().unwrap() as f64 * 1e-6;
        self.real_time = Local::now();
        ctx.request_repaint();
    }
}
