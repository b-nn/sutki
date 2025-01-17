use chrono::{self, DateTime, Datelike, Duration, Local, Utc};
use eframe::egui;
use egui::FontDefinitions;
use log::{log, Level};

// fn main() {
//     let native_options = eframe::NativeOptions::default();
//     let _ = eframe::run_native(
//         "My egui App",
//         native_options,
//         Box::new(|cc| Ok(Box::new(Game::new(cc)))),
//     );
// }

mod upgrades;
use upgrades::{get_upgrades, Upgrade};
mod cats;
mod challenges;
use challenges::{get_challenges, Challenge};
mod prestige;
mod settings;

pub trait Update {
    fn update(&self);
}

#[derive(PartialEq)]
pub enum Tab {
    Cats,
    Upgrades,
    Settings,
    Challenges,
}

pub const TABS: [(&str, Tab); 4] = [
    ("Cats", Tab::Cats),
    ("Upgrades", Tab::Upgrades),
    ("Settings", Tab::Settings),
    ("Challenges", Tab::Challenges),
];

pub const MODULES: [&str; 5] = ["Cats", "Upgrades", "Settings", "Prestige", "Challenges"];

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
    modules: [[bool; MODULES.len()]; TABS.len()],
    in_challenge: bool,
    current_challenge: Challenge,
    challenges: Vec<Challenge>,
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
    modules: [[bool; MODULES.len()]; TABS.len()],
    challenges: Vec<(String, i64, i64)>,
    current_challenge: usize,
    in_challenge: bool,
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
            modules: [
                [true, false, false, true, false],
                [false, true, false, true, false],
                [false, false, true, false, false],
                [false, false, false, false, true],
            ],
            challenges: vec![],
            current_challenge: 1000000,
            in_challenge: false,
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
            modules: [
                [true, false, false, true, false],
                [false, true, false, true, false],
                [false, false, true, false, false],
                [false, false, false, false, true],
            ],
            in_challenge: false,
            current_challenge: Challenge::default(),
            challenges: get_challenges(),
        }
    }
}

impl Game {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.

        let mut fonts = FontDefinitions::default();
        fonts.font_data.insert(
            "Jetbrains".to_owned(),
            std::sync::Arc::new(
                // .ttf and .otf supported
                egui::FontData::from_static(include_bytes!(
                    "../assets/JetBrainsMono-VariableFont_wght.ttf"
                )),
            ),
        );
        fonts
            .families
            .get_mut(&egui::FontFamily::Proportional)
            .unwrap()
            .insert(0, "Jetbrains".to_owned());

        cc.egui_ctx.set_fonts(fonts);

        if let Some(storage) = cc.storage {
            let t: SaveStruct = eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();

            let default_upgrades = get_upgrades();
            let default_challenges = get_challenges();

            let mut final_upgrades = vec![];
            for mut i in default_upgrades {
                for j in &t.upgrades {
                    if &j.0 != &i.text {
                        continue;
                    }
                    if &j.2 != &i.max {
                        continue;
                    }
                    i.count = j.1;
                    i.price = i.price * i.price_mult.powi(j.1 as i32);
                    break;
                }
                final_upgrades.push(i);
            }
            let mut final_challenges = vec![];
            for mut i in default_challenges {
                for j in &t.challenges {
                    if &j.0 != &i.description {
                        continue;
                    }
                    if &j.1 != &i.max {
                        continue;
                    }
                    i.count = j.2;
                    break;
                }
                final_challenges.push(i);
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
                modules: t.modules,
                challenges: final_challenges,
                current_challenge: if t.current_challenge == 1000000 {
                    Challenge::default()
                } else {
                    get_challenges()[t.current_challenge].clone()
                },
                in_challenge: t.in_challenge,
                ..Default::default()
            }
        } else {
            Default::default()
        }
    }
}

fn update(app: &mut Game) {
    app.cat_multipliers = [1.0; 31];
    app.cat_prices = [1.0; 31];
    let mut cps = 0.0;
    for i in 0..app.upgrades.len() {
        if app.upgrades[i].count > 0 {
            (app.upgrades[i].effect)(app, app.upgrades[i].count);
        }
    }
    for i in 0..app.challenges.len() {
        if app.challenges[i].count > 0 {
            (app.challenges[i].boost)(app, app.challenges[i].count);
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
    app.cps = cps;
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

fn render(list: [bool; 5], app: &mut Game, ui: &mut egui::Ui) {
    if list[0] {
        cats::update(app, ui);
    }
    if list[1] {
        upgrades::update(app, ui);
    }
    if list[2] {
        settings::update(app, ui);
    }
    if list[3] {
        prestige::update(app, ui);
    }
    if list[4] {
        challenges::update(app, ui);
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
            modules: self.modules,
            challenges: self
                .challenges
                .iter()
                .map(|x| (x.description.to_owned(), x.count, x.max))
                .collect(),
            current_challenge: self.current_challenge.id,
            in_challenge: self.in_challenge,
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

        if !self.in_challenge {
            update(self);
        } else {
            (self.current_challenge.effect)(self, self.current_challenge.count);
        }

        let central = egui::CentralPanel::default();
        central.show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| match self.state {
                Tab::Cats => render(self.modules[0], self, ui),
                Tab::Upgrades => render(self.modules[1], self, ui),
                Tab::Settings => render(self.modules[2], self, ui),
                Tab::Challenges => render(self.modules[3], self, ui),
            })
        });

        self.dt = (Local::now() - self.real_time).num_microseconds().unwrap() as f64 * 1e-6;
        self.real_time = Local::now();
        ctx.request_repaint();
    }
}
