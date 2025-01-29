use crate::within_day_range;
use crate::Game;
use chrono::{self, Duration, NaiveTime};
use egui::{RichText, Ui};
use std::collections::HashMap;

enum CatInfo {
    Multiplier,
    Efficiency,
    Cps,
}

pub struct Cats {
    info_type: CatInfo,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, PartialEq, Clone)]
pub enum Notations {
    Scientific,
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

fn format(app: &Game, input: f64) -> String {
    match app.notation_format {
        Notations::Scientific=>scientific(input),
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
    format!("{:.2e}", input)
}

fn standard(input: f64) -> String {
    let abbreviation1 = ["","K","M","B","T","Qd","Qn","Sx","Sp","Oc","No"]; // only used once, use abbreviations 2 and 3 for everything above 1 No
    let abbreviation2 = ["","U","D","T","Qa","Qn","Sx","Sp","Oc","No"];
    let abbreviation3 = ["","De","Vg","Tg","Qd","Qn","Se","Sg","Og","Ng","Ce","Dn","Tc","Qe","Qu","Sc","Si","Oe","Ne"];
    
    if input < 1000.0 { // below 1K, dont abbreviate at all
        return input.to_string();
    }
    
    let truncated_str = input.trunc().to_string();
    let exponent: f64 = input.log10().floor() as f64;
    let closest_exponent = (exponent/3.0).floor() as usize;
    let digits_to_display = (closest_exponent % 3) - 1;
    let mut number_to_display = truncated_str[0..digits_to_display].to_string();

    if closest_exponent < 11 { // below 1 Dc, use abbreviations 1
        number_to_display.push_str(abbreviation1[closest_exponent]);
    } else {
        number_to_display.push_str(abbreviation2[closest_exponent % 11]);
        number_to_display.push_str(abbreviation3[closest_exponent / 11]);
    }
    return format!("{}",number_to_display)
}

fn engineering(input: f64) -> String {
    let exponent = (input.log10().floor() as i32 / 3) * 3;
    let normalized_base = input / 10f64.powi(exponent);
    format!("{:.3}e{}", normalized_base, exponent)
}

fn none(input: f64) -> String {
    return input.to_string();
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
    
    let num_without_decimal = input.trunc();
    let mut number_to_display = String::new(); // Use a mutable String

    // Limit to the first 2 digits
    let digits: String = num_without_decimal.to_string().chars().take(3).collect();
    number_to_display.push_str(&digits); // Append to the display string

    // Check if we need to display in scientific notation
    if input > 10.0f64.powi(abbreviations.len() as i32 * 3 - 1) {
        let exponent = (abbreviations.len() - 1) as f64 * 3.0;
        return format!("{:.2e}Z", input / 10.0f64.powf(exponent));
    }

    // If the number is less than 1k, return the number without abbreviation
    if num_without_decimal.to_string().chars().count() <= 3 {
        return num_without_decimal.to_string();
    }

    // Calculate the index for the abbreviation
    let index_of_abbreviation = (num_without_decimal.log10().floor() / 3.0).min((abbreviations.len() - 1) as f64) as usize;

    // Append the appropriate abbreviation
    number_to_display.push_str(abbreviations[index_of_abbreviation]);

    return number_to_display // Return the constructed number string
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
    for i in input.to_string().chars() {
        if let Some(&emoji) = emojicodes.get(&i) {
            emojistring.push_str(emoji);
        }
    }
    emojistring
}
fn morse(input: f64) -> String {
    let mut morsecodes = HashMap::new();
    morsecodes.insert('1',".----");
    morsecodes.insert('2',"..---");
    morsecodes.insert('3',"...--");
    morsecodes.insert('4',"....-");
    morsecodes.insert('5',".....");
    morsecodes.insert('6',"-....");
    morsecodes.insert('7',"--...");
    morsecodes.insert('8',"---..");
    morsecodes.insert('9',"----.");
    morsecodes.insert('0',"-----");
    morsecodes.insert('.',".");

    let mut morsestring = String::new();
    for i in input.to_string().chars() {
        if let Some(&morse) = morsecodes.get(&i) {
            morsestring.push_str(morse);
        }
    }
    morsestring
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
    for i in input.to_string().chars() {
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
    for i in input.to_string().chars() {
        if let Some(&heart) = heartcodes.get(&i) {
            heartstring.push_str(heart);
        }
    }
    heartstring // Return the constructed heart string
}

fn reverse(input: f64) -> String {
    format!("{}",input.to_string().chars().rev().collect::<String>())
}

fn blind(_input: f64) -> String {
    format!("{}","")
}

pub fn update(app: &mut Game, ui: &mut Ui, ctx: &egui::Context) {
    ui.label(format!(
        "You currently have {}$ (+{}$/s)",
        format(app, app.currencies[0]),
        format(app, app.cps)
    ));
    if app.unlocked_tiers[1] {
        ui.label(format!(
            "You have {} strawberries.",
            format(app, app.currencies[1])
        ));
    }
    let tomorrow_midnight = (app.date + Duration::days(1))
        .with_time(NaiveTime::from_hms_opt(0, 0, 0).unwrap())
        .unwrap();

    ui.label(format!(
        "{:.2} seconds until tomorrow.",
        (tomorrow_midnight - app.date).num_seconds() as f64
            / 2_f64.powi(app.upgrades[2].count as i32),
    ));
    egui::ScrollArea::vertical()
        .drag_to_scroll(false)
        .show(ui, |ui| {
            ui.set_min_width(330.0);

            // cat_handler(app, ui);
            let _image = egui::include_image!("../assets/day-5.gif"); // rust stfu about this

            egui::Grid::new("GRIDDDD").striped(true).show(ui, |ui| {
                ui.label("Monday");
                ui.label("Tuesday");
                ui.label("Wednesday");
                ui.label("Thurday");
                ui.label("Friday");
                ui.label("Saturday");
                ui.label("Sunday");
                ui.end_row();
                for i in 0..app.cats.len() {
                    if i % 7 == 0 && i != 0 {
                        ui.end_row();
                    }
                    ui.vertical(|ui| {
                        let mut size = (ctx.screen_rect().width() - 8.0 * 3.0) / 14.0;
                        size -= if within_day_range(app.day, app.day_width, i as u32)
                            && !app.asleep
                            && app.current_challenge.id != 0
                        {
                            ui.label(
                                RichText::new(format!(
                                    "{} 'Day {}' cats\n[{}]",
                                    app.cats[i],
                                    i + 1,
                                    format(app, app.cat_multipliers[i])
                                ))
                                .color(app.colors[0]),
                            )
                            .on_hover_text("This cat is Extra Effective!")
                        } else {
                            ui.label(format!(
                                "{} 'Day {}' cats\n[{}]",
                                app.cats[i],
                                i + 1,
                                format(app, app.cat_multipliers[i])
                            ))
                        }
                        .rect
                        .height();
                        if size < 10.0 {
                            size = 10.0
                        }

                        if ui
                            .add_enabled_ui(app.cat_prices[i] <= app.currencies[0], |ui| {
                                ui.add_sized(
                                    [
                                        (ctx.screen_rect().width() - 8.0 * 8.0) / 7.0,
                                        size / 2.0 - 3.0,
                                    ],
                                    egui::Button::new(format!(
                                        "Hire another cat {}$",
                                        format(app, app.cat_prices[i])
                                    )),
                                )
                            })
                            .inner
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
                                .add_enabled_ui(
                                    app.currencies[1] >= app.cat_strawberry_prices[i].pow(2) as f64,
                                    |ui| {
                                        ui.add_sized(
                                            [
                                                (ctx.screen_rect().width() - 8.0 * 8.0) / 7.0,
                                                size / 2.0 - 3.0,
                                            ],
                                            egui::Button::new(format!(
                                                "Feed cat {} strawberry",
                                                app.cat_strawberry_prices[i].pow(2)
                                            )),
                                        )
                                        .on_hover_text("Gives a 1.5x boost to this cat")
                                    },
                                )
                                .inner
                                .clicked()
                            {
                                app.currencies[1] -= app.cat_strawberry_prices[i].pow(2) as f64;
                                app.cat_strawberries[i] += 1;
                                app.cat_strawberry_prices[i] += 1;
                            }
                        }
                    });
                }
                // ui.set_min_height((ctx.screen_rect().width() - 8.0 * 8.0) / 7.0);
            });

            // for i in 0..app.cats.len() {
            //     if i == 4 {
            //         ui.vertical(|ui| {
            //             if within_day_range(app.day, app.day_width, i as u32)
            //                 && !app.asleep
            //                 && app.current_challenge.id != 0
            //             {
            //                 ui.label(
            //                     RichText::new(format!(
            //                         "Total: {} [{:.2}]",
            //                         app.cats[i], app.cat_multipliers[i]
            //                     ))
            //                     .color(app.colors[0]),
            //                 )
            //                 .on_hover_text("This cat is Extra Effective!");
            //             } else {
            //                 ui.label(format!(
            //                     "Total: {} [{:.2}]",
            //                     app.cats[i], app.cat_multipliers[i]
            //                 ));
            //             }

            //             let t = egui::Image::new(egui::include_image!("../assets/day-5.gif"))
            //                 .maintain_aspect_ratio(true)
            //                 .max_width(ctx.screen_rect().width() / 5.0);
            //             println!("{:?}", ctx.screen_rect());

            //             let button = egui::ImageButton::new(t);
            //             if ui
            //                 .add_enabled(app.cat_prices[i] <= app.currencies[0], button)
            //                 .on_hover_text(format!(
            //                     "x{} to self, x5 to all other unbought cats",
            //                     app.cat_price_multipliers[i],
            //                 ))
            //                 .clicked()
            //             {
            //                 app.currencies[0] -= app.cat_prices[i];
            //                 if app.cats[i] == 0.0 {
            //                     for j in 0..app.cat_prices.len() {
            //                         if i != j && app.cats[j] == 0.0 {
            //                             app.cat_price_5_multiplier[j] += 1.0;
            //                         }
            //                     }
            //                 }
            //                 app.cats[i] += 1.0;
            //             }
            //             if app.unlocked_tiers[1] {
            //                 if ui
            //                     .add_enabled(
            //                         app.currencies[1] >= app.cat_strawberry_prices[i].pow(2) as f64,
            //                         egui::Button::new(format!(
            //                             "Feed {} strawberry",
            //                             app.cat_strawberry_prices[i].pow(2)
            //                         )),
            //                     )
            //                     .clicked()
            //                 {
            //                     app.currencies[1] -= app.cat_strawberry_prices[i].pow(2) as f64;
            //                     app.cat_strawberries[i] += 1;
            //                     app.cat_strawberry_prices[i] += 1;
            //                 }
            //             }
            //         });
            //         continue;
            //     }

            //     ui.horizontal(|ui| {
            //         if within_day_range(app.day, app.day_width, i as u32)
            //             && !app.asleep
            //             && app.current_challenge.id != 0
            //         {
            //             ui.label(
            //                 RichText::new(format!(
            //                     "You have {} 'Day {}' cats [{:.2}]",
            //                     app.cats[i],
            //                     i + 1,
            //                     app.cat_multipliers[i]
            //                 ))
            //                 .color(app.colors[0]),
            //             )
            //             .on_hover_text("This cat is Extra Effective!");
            //         } else {
            //             ui.label(format!(
            //                 "You have {} 'Day {}' cats [{:.2}]",
            //                 app.cats[i],
            //                 i + 1,
            //                 app.cat_multipliers[i]
            //             ));
            //         }
            //         if i == 4 {
            //             ui.image(egui::include_image!("../assets/day-5.gif"));
            //         }

            //         if ui
            //             .add_enabled(
            //                 app.cat_prices[i] <= app.currencies[0],
            //                 egui::Button::new(format!("Hire another cat {:.2}$", app.cat_prices[i])),
            //             )
            //             .on_hover_text(format!(
            //                 "x{} to self, x5 to all other unbought cats",
            //                 app.cat_price_multipliers[i],
            //             ))
            //             .clicked()
            //         {
            //             app.currencies[0] -= app.cat_prices[i];
            //             if app.cats[i] == 0.0 {
            //                 for j in 0..app.cat_prices.len() {
            //                     if i != j && app.cats[j] == 0.0 {
            //                         app.cat_price_5_multiplier[j] += 1.0;
            //                     }
            //                 }
            //             }
            //             app.cats[i] += 1.0;
            //         }

            //         if app.unlocked_tiers[1] {
            //             if ui
            //                 .add_enabled(
            //                     app.currencies[1] >= app.cat_strawberry_prices[i].pow(2) as f64,
            //                     egui::Button::new(format!(
            //                         "Feed cat {} strawberry",
            //                         app.cat_strawberry_prices[i].pow(2)
            //                     )),
            //                 )
            //                 .clicked()
            //             {
            //                 app.currencies[1] -= app.cat_strawberry_prices[i].pow(2) as f64;
            //                 app.cat_strawberries[i] += 1;
            //                 app.cat_strawberry_prices[i] += 1;
            //             }
            //         }
            //     });
            // }
        });

    // ui.hyperlink("https://github.com/emilk/egui");
    // ui.text_edit_singleline(&mut app.label);
    // if ui.button("Click me").clicked() {}
    // ui.add(egui::Slider::new(&mut app.fps, 0.0..=240.0).prefix("Desired FPS: "));
    // ui.label(format!("Current FPS: {}", (1.0 / app.dt).round()));
    // ui.label(format!("count: {}", app.count));

    // ui.checkbox(&mut false, "Checkbox");

    // ui.horizontal(|ui| {
    //     ui.radio_value(&mut app.num, Enum::First, "First");
    //     ui.radio_value(&mut app.num, Enum::Second, "Second");
    //     ui.radio_value(&mut app.num, Enum::Third, "Third");
    // });

    // ui.separator();

    // ui.collapsing("Click to see what is hidden!", |ui| {
    //     ui.label("Not much, as it turns out");
    // });
}
