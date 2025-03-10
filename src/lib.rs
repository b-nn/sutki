use chrono::{self, DateTime, Datelike, Duration, Local, Timelike, Utc};
use eframe::egui;
use egui::{FontDefinitions, Style};
use log::{log, Level};

mod challenges;
mod format;
mod upgrades;
use challenges::{get_challenges, Challenge};
use upgrades::{get_upgrades, Upgrade};
mod automation;
mod cats;
mod prestige;
mod settings;
use format::Notations;

pub trait Module {
    fn update(&self, app: &mut Game, ui: &mut egui::Ui);
}

#[derive(PartialEq)]
pub enum Tab {
    Cats,
    Upgrades,
    Settings,
    Challenges,
    Automation,
}

pub const TABS: [(&str, Tab); 5] = [
    ("Cats", Tab::Cats),
    ("Upgrades", Tab::Upgrades),
    ("Settings", Tab::Settings),
    ("Challenges", Tab::Challenges),
    ("Automation", Tab::Automation),
];

pub const MODULES: [&str; 6] = [
    "Cats",
    "Upgrades",
    "Settings",
    "Prestige",
    "Challenges",
    "Automation",
];

pub const COLORS: [egui::Color32; 2] = [
    egui::Color32::from_rgb(255, 120, 79),
    egui::Color32::from_rgb(11, 20, 22),
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
    cats_visible: [bool; 31],
    currencies: [f64; 3],
    upgrades: Vec<Upgrade>,
    cat_strawberries: [i64; 31],
    cat_strawberry_prices: [i64; 31],
    unlocked_tiers: [bool; 3],
    status: String,
    status_time: DateTime<Local>,
    currency_symbols: [char; 3],
    asleep: bool,
    cps: f64,
    state: Tab,
    modules: [[bool; MODULES.len()]; TABS.len()],
    in_challenge: bool,
    current_challenge: Challenge,
    challenges: Vec<Challenge>,
    automation_interval: f64,
    automation_enabled: bool,
    automation_mode: automation::AutomationMode,
    automation_delay: f64,
    money_gain_per_cat: [f64; 31],
    automation_unlocked: bool,
    settings_text_field: String,
    titles: Vec<String>,
    title_delay: f64,
    title_index: usize,
    zoom: f32,
    notation_format: Notations,
    uwumode: bool,
}

fn change_status(
    level: Level,
    message: &str,
    status_reference: &mut String,
    time_reference: &mut DateTime<Local>,
) {
    *status_reference = message.to_owned();
    *time_reference = Local::now();
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
    currencies: [f64; 3],
    upgrades: Vec<(String, i64, i64)>,
    cat_strawberries: [i64; 31],
    cat_strawberry_prices: [i64; 31],
    unlocked_tiers: [bool; 3],
    cat_price_5_multiplier: [f64; 31],
    modules: [[bool; MODULES.len()]; TABS.len()],
    challenges: Vec<(String, i64, i64)>,
    current_challenge: usize,
    in_challenge: bool,
    automation_interval: f64,
    automation_enabled: bool,
    automation_mode: automation::AutomationMode,
    zoom: f32,
    notation_format: Notations,
    uwumode: bool,
    cats_visible: [bool; 31],
}

impl Default for SaveStruct {
    fn default() -> Self {
        Self {
            cats: [0.0; 31],
            cat_prices: [1.0; 31],
            day_offset: 1.0,
            day_width: 0,
            currencies: [1.0, 0.0, 0.0],
            cat_times: [0.0; 31],
            upgrades: vec![],
            cat_strawberries: [0; 31],
            cat_strawberry_prices: [1; 31],
            unlocked_tiers: [true, false, false],
            cat_price_5_multiplier: [0.0; 31],
            modules: [
                [true, true, false, true, false, false],
                [false, true, false, true, false, false],
                [false, false, true, false, false, false],
                [false, false, false, false, true, false],
                [false, false, false, false, false, true],
            ],
            challenges: vec![],
            current_challenge: 1000000,
            in_challenge: false,
            automation_mode: automation::AutomationMode::MostMoney,
            automation_interval: 0.1,
            automation_enabled: false,
            zoom: 1.0,
            notation_format: Notations::HybridScientific,
            uwumode: false,
            cats_visible: [false; 31],
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
            currencies: [1.0, 0.0, 0.0],
            cat_times: [0.0; 31],
            //colors: [egui::Color32::from_hex("#FF784F").unwrap()],
            upgrades: get_upgrades(),
            cat_strawberries: [0; 31],
            cat_strawberry_prices: [1; 31],
            unlocked_tiers: [true, false, false],
            status: "Opened game".to_owned(),
            status_time: Local::now(),
            currency_symbols: ['$', '🍓', '🥇'],
            cat_price_5_multiplier: [0.0; 31],
            asleep: false,
            cps: 0.0,
            date: Utc::now(),
            state: Tab::Cats,
            modules: [
                // Cats, Upgrades, Settings, Prestige, Challenges, Automation
                [true, true, false, true, false, false],
                [false, true, false, true, false, false],
                [false, false, true, false, false, false],
                [false, false, false, false, true, false],
                [false, false, false, false, false, true],
            ],
            in_challenge: false,
            current_challenge: Challenge::default(),
            challenges: get_challenges(),
            automation_mode: automation::AutomationMode::MostMoney,
            automation_interval: 0.1,
            automation_enabled: false,
            money_gain_per_cat: [0.0; 31],
            automation_delay: 0.0,
            automation_unlocked: false,
            settings_text_field: String::new(),
            titles: vec![
                "wow! moving title!!!               ".to_owned(),
                r#""remember kids, it's only homicide if you kill a member of your own species""#
                    .to_owned(),
                "But then I had a very good idea, I used F5".to_owned(),
                "c++ is a very valid reason to cry".to_owned(),
                r#"And god said "Let there be cabbits!""#.to_owned(),
                ":3".to_owned(),
                ">:3".to_owned(),
                r#"":3" MY ASS, GET OUT OF MY HOUSE!!!"#.to_owned(),
                "not like the other catgirls".to_owned(),
                "hor".to_owned(),
                "Now dogassium free!".to_owned(),
                "so THAT just happened".to_owned(),
                "it's a pile of balls".to_owned(),
                "gaslight gatekeep girlboss".to_owned(),
                "i should never come into offtopic ever again i guess".to_owned(),
                "seinfield".to_owned(),
                "Also try- actually no don't".to_owned(),
                "heh.".to_owned(),
                "hop on yoinkz".to_owned(),
                "iktus".to_owned(),
                "sutki // sutki // sutki // sutki // sutki // sutki // sutki // sutki".to_owned(),
                "clidle edition".to_owned(),
                "clordle edition".to_owned(),
                "meow btw. if you even care.".to_owned(),
                "This game contains an undisclosed ad for the gaming site galaxy.click".to_owned(),
                "I like rubbing my belly".to_owned(),
                r#""hey did you know that-" yes john, everyone knows"#.to_owned(),
                "this is the 28th title".to_owned(),
                "Your lips look dry. And don't even try licking them.".to_owned(),
                "this is the 82nd title".to_owned(),
                "titlebar".to_owned(),
            ],
            title_delay: 0.0,
            title_index: (Utc::now().second() % 31) as usize, // should always be titles.count
            zoom: 1.0,
            notation_format: Notations::HybridScientific,
            uwumode: false,
            cats_visible: [false; 31],
        }
    }
}

pub fn save_game(t: &mut Game) -> SaveStruct {
    SaveStruct {
        cats: t.cats,
        day_offset: 0.0,
        day_width: t.day_width,
        cat_prices: t.cat_prices,
        cat_times: t.cat_times,
        currencies: t.currencies,
        upgrades: t
            .upgrades
            .iter()
            .map(|x| (x.text.to_owned(), x.count, x.max))
            .collect(),
        cat_strawberries: t.cat_strawberries,
        cat_strawberry_prices: t.cat_strawberry_prices,
        unlocked_tiers: t.unlocked_tiers,
        cat_price_5_multiplier: t.cat_price_5_multiplier,
        modules: t.modules,
        challenges: t
            .challenges
            .iter()
            .map(|x| (x.description.to_owned(), x.count, x.max))
            .collect(),
        current_challenge: t.current_challenge.id,
        in_challenge: t.in_challenge,
        automation_interval: t.automation_interval,
        automation_enabled: t.automation_enabled,
        automation_mode: t.automation_mode.clone(),
        zoom: t.zoom,
        notation_format: t.notation_format.clone(),
        uwumode: t.uwumode,
        cats_visible: t.cats_visible,
    }
}

pub fn load_game(t: SaveStruct) -> Game {
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
            if &j.2 != &i.max {
                continue;
            }
            i.count = j.1;
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
        cats_visible: t.cats_visible,
        ..Default::default()
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

        // cc.egui_ctx.set_style_of(
        //     egui::Theme::Dark,
        //     Style {
        //         visuals: egui::Visuals {
        //             dark_mode: true,
        //             override_text_color: None,
        //             widgets: egui::style::Widgets {
        //                 noninteractive: egui::style::WidgetVisuals {
        //                     bg_fill: COLORS[1],
        //                     weak_bg_fill: COLORS[1],
        //                     bg_stroke: egui::Stroke {
        //                         color: COLORS[1],
        //                         ..Default::default()
        //                     },
        //                     fg_stroke: egui::Stroke {
        //                         color: COLORS[0],
        //                         ..Default::default()
        //                     },
        //                     expansion: 0.0,
        //                     rounding: egui::Rounding::same(2.0),
        //                 },
        //                 ..Default::default()
        //             },
        //             selection: (),
        //             hyperlink_color: (),
        //             faint_bg_color: (),
        //             extreme_bg_color: (),
        //             code_bg_color: (),
        //             warn_fg_color: (),
        //             error_fg_color: (),
        //             window_rounding: (),
        //             window_shadow: (),
        //             window_fill: (),
        //             window_stroke: (),
        //             window_highlight_topmost: (),
        //             menu_rounding: (),
        //             panel_fill: (),
        //             popup_shadow: (),
        //             resize_corner_size: (),
        //             text_cursor: (),
        //             clip_rect_margin: (),
        //             button_frame: (),
        //             collapsing_header_frame: (),
        //             indent_has_left_vline: (),
        //             striped: (),
        //             slider_trailing_fill: (),
        //             handle_shape: (),
        //             interact_cursor: (),
        //             image_loading_spinners: (),
        //             numeric_color_space: (),
        //         },
        //         ..Default::default()
        //     },
        // );
        cc.egui_ctx.set_fonts(fonts);

        if let Some(storage) = cc.storage {
            let t: SaveStruct = eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
            load_game(t)
        } else {
            Default::default()
        }
    }
}

// #[cfg(not(target_arch = "wasm32"))]
// fn set_title(ctx: &egui::Context, t: String) {
//     ctx.send_viewport_cmd(egui::ViewportCommand::Title(t));
// }
//
// #[cfg(target_arch = "wasm32")]
// fn set_title(ctx: &egui::Context, t: String) {
//     use eframe::wasm_bindgen::JsCast as _;
//     wasm_bindgen_futures::spawn_local(async {});
//
//     let document = web_sys::window()
//         .expect("No window")
//         .document()
//         .expect("No document");
//
//     document.title = t;
// }

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

fn render(list: [bool; 6], app: &mut Game, ui: &mut egui::Ui, ctx: &egui::Context) {
    if list[0] {
        cats::update(app, ui, ctx);
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
    if list[5] {
        automation::update(app, ui);
    }
}

impl eframe::App for Game {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        let t = save_game(self);

        eframe::set_value(storage, eframe::APP_KEY, &t);
        change_status(
            Level::Info,
            "Saved!",
            &mut self.status,
            &mut self.status_time,
        );
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.zoom = ctx.zoom_factor();
        let mut t = "sutki // ".to_owned();
        t.push_str(match self.title_index {
            0 => {
                if self.title_delay > 0.1 {
                    let mut b: Vec<char> = self.titles[0].chars().collect();
                    b.rotate_left(1);
                    self.titles[0] = b.iter().collect();
                    self.title_delay -= 0.1;
                }
                &self.titles[self.title_index][15..]
            }
            9 => {
                if self.real_time.timestamp_micros() % 15 == 0 {
                    "hor"
                } else {
                    ""
                }
            }
            _ => &self.titles[self.title_index],
        });

        ctx.send_viewport_cmd(egui::ViewportCommand::Title(t));

        // change title here

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
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
                    // if ui.button("money (for testing purposes)").clicked() {
                    //     self.currencies[0] += 10000.0;
                    // }
                    // if ui.button("more money (for testing purposes)").clicked() {
                    //     self.currencies[0] += 1e15;
                    // }
                    // if ui.button("strawberries (for testing purposes)").clicked() {
                    //     self.currencies[1] += 100.0;
                    // }
                    // if ui
                    //     .button("more strawberries (for testing purposes)")
                    //     .clicked()
                    // {
                    //     self.currencies[1] += 1e9;
                    // }
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

        if self.automation_enabled {
            let mut keep_looping = true;
            while self.automation_delay > self.automation_interval && keep_looping {
                self.automation_delay -= self.automation_interval;
                keep_looping = automation::buy_best_cat(self);
            }
        }
        self.automation_delay %= self.automation_interval;

        if !self.in_challenge {
            update(self);
        } else {
            (self.current_challenge.effect)(self, self.current_challenge.count);
        }

        let central = egui::CentralPanel::default();
        central.show(ctx, |ui| {
            egui::ScrollArea::vertical()
                .drag_to_scroll(false)
                .show(ui, |ui| match self.state {
                    Tab::Cats => render(self.modules[0], self, ui, ctx),
                    Tab::Upgrades => render(self.modules[1], self, ui, ctx),
                    Tab::Settings => render(self.modules[2], self, ui, ctx),
                    Tab::Challenges => render(self.modules[3], self, ui, ctx),
                    Tab::Automation => render(self.modules[4], self, ui, ctx),
                })
        });

        self.dt = (Local::now() - self.real_time).num_microseconds().unwrap() as f64 * 1e-6;
        self.real_time = Local::now();
        self.automation_delay += self.dt;
        self.title_delay += self.dt;
        ctx.set_zoom_factor(self.zoom);
        ctx.request_repaint();
    }
}
