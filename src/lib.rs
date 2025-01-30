use std::collections::HashMap;
use chrono::{self, DateTime, Datelike, Duration, Local, Timelike, Utc};
use eframe::egui;
use egui::FontDefinitions;
use log::{log, Level};

mod upgrades;
mod challenges;
use challenges::{get_challenges, Challenge};
use upgrades::{get_upgrades, Upgrade};

mod cats;
mod automation;
mod prestige;
mod settings;
mod prestigetwo_placeholder;

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

#[derive(serde::Deserialize, serde::Serialize, Debug, Eq, PartialEq, Clone, Copy, Hash)]
pub enum Notations {
    Scientific,
    HybridScientific,
    Standard,
    Engineering,
    None,
    Binary,
    Hex,
    Logarithm,
    Leaf,
    Emoji,
    Morse,
    Celeste,
    Heart,
    Reverse,
    Blind,
}


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
    currencies: [f64; 3],
   //colors: [egui::Color32; 1],
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
                [true, false, false, true, false, false],
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
            currency_symbols: ['$', '🍓','🥇'],
            cat_price_5_multiplier: [0.0; 31],
            asleep: false,
            cps: 0.0,
            date: Utc::now(),
            state: Tab::Cats,
            modules: [ // Cats, Upgrades, Settings, Prestige, Challenges, Automation
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
            load_game(t)
        } else {
            Default::default()
        }
    }
}

fn update_base(app: &mut Game, cats: [f64; 31]) -> f64 {
    app.cat_multipliers = [1.0; 31];
    app.cat_prices = [1.0; 31];
    let mut cps = 0.0;
    for i in 0..app.cat_multipliers.len() {
        app.cat_multipliers[i] += 1.036_f64.powi(31 - i as i32);
        app.cat_multipliers[i] += 1.036_f64.powi(i as i32);
        app.cat_multipliers[i] *= 1.5f64.powi(app.cat_strawberries[i] as i32);
        if app.asleep {
            continue;
        }
        app.cat_multipliers[i] *= cats
            .iter()
            .enumerate()
            .map(|(x, y)| if x == i { 0.0 } else { *y * 0.01 })
            .sum::<f64>()
            + 1.0;
    }

    cps += cats
        .iter()
        .zip(app.cat_multipliers.iter())
        .map(|(x, y)| x * y)
        .sum::<f64>();
    cps
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

pub fn formatnum(app: &Game, input: f64) -> String {
    match app.notation_format {
        Notations::Scientific=>scientific(input),
        Notations::HybridScientific=>hybrid_scientific(input),
        Notations::Standard=>standard(input),
        Notations::Engineering=>engineering(input),
        Notations::None=>none(input),
        Notations::Binary=>binary(input),
        Notations::Hex=>hex(input),
        Notations::Logarithm=>logarithm(input),
        Notations::Emoji=>emoji(input),
        Notations::Blind=>blind(input),
        Notations::Morse=>morse(input),
        Notations::Leaf=>leaf(input),
        Notations::Reverse=>reverse(input),
        Notations::Celeste=>celeste(input),
        Notations::Heart=>heart(input),
    }
}

fn scientific(input: f64) -> String {
    format!("{:.3e}", input)
}

fn hybrid_scientific(input: f64) -> String {
    if input < 10000.0 {
        format!("{:.3}", input)
    } else {
        format!("{:.3e}", input)
    }
}

fn standard(input: f64) -> String {
    // let abbreviation1 = ["","K","M","B","T","Qd","Qn","Sx","Sp","Oc","No"]; // only used once, use abbreviations 2 and 3 for everything above 1 No
    // let abbreviation2 = ["","U","D","T","Qa","Qn","Sx","Sp","Oc","No"];
    // let abbreviation3 = ["","De","Vg","Tg","Qd","Qn","Se","Sg","Og","Ng","Ce","Dn","Tc","Qe","Qu","Sc","Si","Oe","Ne"];
    
    // // 1dc = 1e33

    // if input < 1000.0 { // below 1K, dont abbreviate at all
    //     return input.to_string();
    // }
    
    // let truncated_str = input.trunc().to_string();
    // let exponent = input.log10().floor();
    // let closest_exponent = (exponent/3.0).floor();
    // let digits_to_display = (exponent % 3.0) as usize;
    // let mut number_to_display = truncated_str.chars().take(digits_to_display).collect::<String>();

    // if exponent < 33.0 { // below 1 Dc, use abbreviations 1
    //     number_to_display.push_str(abbreviation1[closest_exponent as usize]);
    // } else {
    //     number_to_display.push_str(abbreviation2[(closest_exponent % 11.0) as usize]);
    //     number_to_display.push_str(abbreviation3[(closest_exponent / 11.0).floor() as usize]);
    // }
    // format!("{}",number_to_display)
    format!("NOTIMPLEMENTED{:.2e}", input)
}

fn engineering(input: f64) -> String {
    let exponent = (input.log10().floor() as i32 / 3) * 3;
    let normalized_base = input / 10f64.powi(exponent);
    format!("{:.3}e{}", normalized_base, exponent)
}

fn none(input: f64) -> String {
    format!("{:.3}",input)
}

fn binary(input: f64) -> String {
    format!("{:064b}", input.to_bits())
}

fn hex(input: f64) -> String {
    format!("{:016x}", input.to_bits())
}

fn logarithm(input: f64) -> String {
    format!("e{}",input.log10())
}

fn leaf(input: f64) -> String {
    let abbreviations = ["", "k", "m", "b", "t", "a", "A", "c", "C", "d", "D", "e", "E", "f", "F", "g", "G", "h", "H", "i", "I", "j", "J", "n", "N", "o", "O", "p", "P", "q", "Q", "r", "R", "s", "S", "u", "U", "v", "V", "w", "W", "x", "X", "y", "Y", "z", "Z"];

    // too big man
    if input > 10.0_f64.powf(144.0) {
        return format!("{:.3e}Z", input - 10.0_f64.powf(144.0));
    }

    let exponent = input.log10().floor();
    let closest_exponent: usize = (exponent / 3.0).floor() as usize;

    let scaled_number = input / 10_f64.powi((closest_exponent * 3) as i32);
    let formatted_number = format!("{:.3}", scaled_number);

    if exponent < 3.0 { //<1k, dont do anything
        return format!("{:.3}", input);
    }

    format!("{}{}", formatted_number, abbreviations[closest_exponent])
}

fn emoji(input: f64) -> String {
    let mut emojicodes = HashMap::new();
    emojicodes.insert('1',"🐦‍🔥");
    emojicodes.insert('2',"🍓");
    emojicodes.insert('3',"🔱");
    emojicodes.insert('4',"💅");
    emojicodes.insert('5',"🏳️‍⚧️");
    emojicodes.insert('6',"🎲");
    emojicodes.insert('7',"🎰");
    emojicodes.insert('8',"🎡");
    emojicodes.insert('9',"🫨");
    emojicodes.insert('0',"🕸️");
    emojicodes.insert('.',".");

    let mut emojistring = String::new();
    for i in format!("{:.3}",input).chars() {
        if let Some(&emoji) = emojicodes.get(&i) {
            emojistring.push_str(emoji);
        }
    }
    emojistring
}

fn morse(input: f64) -> String {
    let morse_tuples = [
        ("A", ".-"), ("B", "-..."), ("C", "-.-."), ("D", "-.."),
        ("E", "."), ("F", "..-."), ("G", "--."), ("H", "...."),
        ("I", ".."), ("J", ".---"), ("K", "-.-"), ("L", ".-.."),
        ("M", "--"), ("N", "-."), ("O", "---"), ("P", ".--."),
        ("Q", "--.-"), ("R", ".-."), ("S", "..."), ("T", "-"),
        ("U", "..-"), ("V", "...-"), ("W", ".--"), ("X", "-..-"),
        ("Y", "-.--"), ("Z", "--.."),
        ("1", ".----"), ("2", "..---"), ("3", "...--"), ("4", "....-"),
        ("5", "....."), ("6", "-...."), ("7", "--..."), ("8", "---.."),
        ("9", "----."), ("0", "-----")
    ];
    
    let base = morse_tuples.len() as f64;
    let mut morsestring = String::new();
    let mut quotient = input as f64;
    while quotient > 0.0 {
        let remainder = (quotient % base) as usize;
        morsestring.push_str(morse_tuples[remainder].0);
        quotient = (quotient / base).floor();
    }
    
    format!("{}", morsestring.chars().rev().collect::<String>())
}

fn celeste(input: f64) -> String {
    let mut celestecodes = HashMap::new();
    celestecodes.insert('1',":maddyhug:");
    celestecodes.insert('2',":baddyhug:");
    celestecodes.insert('3',":lanihug:");
    celestecodes.insert('4',":radgranny:");
    celestecodes.insert('5',":theoretical:");
    celestecodes.insert('6',":reaperline:");
    celestecodes.insert('7',":fullclear:");
    celestecodes.insert('8',":CrystalHeart:");
    celestecodes.insert('9',":birb:");
    celestecodes.insert('0',":catbus:");
    celestecodes.insert('.', ".");

    let mut celestestring = String::new();
    for i in format!("{:.3}",input).chars() {
        if let Some(&celeste) = celestecodes.get(&i) {
            celestestring.push_str(celeste);
        }
    }
    celestestring
}

fn heart(input: f64) -> String {
    let mut heartcodes = HashMap::new();
    heartcodes.insert('1', "❤");
    heartcodes.insert('2', "🧡");
    heartcodes.insert('3', "💛");
    heartcodes.insert('4', "💚");
    heartcodes.insert('5', "💙");
    heartcodes.insert('6', "💜");
    heartcodes.insert('7', "🤎");
    heartcodes.insert('8', "🖤");
    heartcodes.insert('9', "🤍");
    heartcodes.insert('0', "💔");
    heartcodes.insert('.', ".");

    let mut heartstring = String::new();
    for i in format!("{:.3}",input).chars() {
        if let Some(&heart) = heartcodes.get(&i) {
            heartstring.push_str(heart);
        }
    }
    heartstring // Return the constructed heart string
}

fn reverse(input: f64) -> String {
    format!("{}",format!("{:.3}",input).chars().rev().collect::<String>())
}

fn blind(_input: f64) -> String {
    format!("{}","")
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
                    if ui.button("money (for testing purposes)").clicked() {
                        self.currencies[0] += 10000.0;
                    }
                    if ui.button("more money (for testing purposes)").clicked() {
                        self.currencies[0] += 10000000000000000.0;
                    }
                    if ui.button("strawberries (for testing purposes)").clicked() {
                        self.currencies[1] += 100.0;
                    }
                    if ui.button("more strawberries (for testing purposes)").clicked() {
                        self.currencies[1] += 10000000000000000.0;
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

        if self.automation_enabled {
            if self.automation_delay > self.automation_interval {
                self.automation_delay = 0.0;
                automation::buy_best_cat(self);
            }
        }

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
