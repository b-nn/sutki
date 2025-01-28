use crate::within_day_range;
use crate::Game;
use chrono::{self, Duration, NaiveTime};
use egui::debug_text::print;
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

fn format(input: f64) -> String {
    match notation_format{
        Notations::Scientific=>Scientific(input),
        Notations::Standard=>Standard(input),
        Notations::Engineering=>Engineering(input),
        Notations::None=>None(input),
        Notations::Binary=>Binary(input),
        Notations::Hex=>Hex(input),
        Notations::Logarithm=>Logarithm(input),
        Notations::Emoji=>Emoji(input),
        Notations::Blind=>Blind(input),
        Notations::Morse=>Morse(input),
        Notations::Zalgo=>Zalgo(input),
        Notations::Leaf=>Leaf(input),
        Notations::Reverse=>Reverse(input),
        Notations::Celeste=>Celeste(input),
        Notations::Heart=>Heart(input),
        _=>Error(input),
    }
}

fn Error(input: f64) {
    print!("{}","you fucked up");
    Scientific(input.to_string());
}

fn Scientific(input: f64) {
    format!("{:.2e}", input.to_string());
}

fn Standard(input: f64) {
    let abbreviation1 = ["","K","M","B","T","Qd","Qn","Sx","Sp","Oc","No"]; // only used once, use abbreviations 2 and 3 for everything above 1 No
    let abbreviation2 = ["","U","D","T","Qa","Qn","Sx","Sp","Oc","No"];
    let abbreviation3 = ["","De","Vg","Tg","Qd","Qn","Se","Sg","Og","Ng","Ce","Dn","Tc","Qe","Qu","Sc","Si","Oe","Ne"];
    let numwithoutdecimal = input.trunc()
    let numbertodisplay = "";

    let j = 0;
    for i in numwithoutdecimal.to_string().chars() {
        if j >= 2  {
            return;
        }
        numbertodisplay = numbertodisplay + insert(i);
        j = j + 1;
    } // oh for fucks sake give me substr like a normal programming language

    if (numwithoutdecimal.to_string().chars() <= 3) { // below 1K, dont abbreviate at all
        format!("{}",numbertodisplay);
    }

    let indexofabbreviation: f32 = if numwithoutdecimal == 0 {
        1 // Handle the special case for 0
    } else {
        (numwithoutdecimal.abs().to_string().len() / 3).floor()
    }; // how many triplets of zeroes are in it

    let abtwotouse = indexofabbreviation % 11;

    if indexofabbreviation < 11 { // below 1 Dc, use abbreviations 1
        format!("{}",numbertodisplay + abbreviation1[indexofabbreviation]);
    } else {
        amountofabthree = indexofabbreviation/11;
        format!("{}", numbertodisplay + abbreviation2[indexofabbreviation % 11 + 1] + abbrevation3[(indexofabbreviation/11).floor()]);
    }

}

fn Engineering(input: f64) {
    let exponent = (input.log10().floor() as i32 / 3) * 3;
    let normalized_base = input / 10f64.powi(exponent);
    // Format the engineering notation
    format!("{:.3} x 10^{}", normalized_base, exponent);
}

fn None(input: f64) {
    format!("{}",input);
}

fn Binary(input: f64) {
    format!("{}",input);
}

fn Hex(input: f64) {
    format!("{}",input);
}

fn Logarithm(input: f64) {
    format!("e{}",input.log10());
}

fn Leaf(input: f64) {
    let abbreviations = ["", "k", "m", "b", "t", "a", "A", "c", "C", "d", "D", "e", "E", "f", "F", "g", "G", "h", "H", "i", "I", "j", "J", "n", "N", "o", "O", "p", "P", "q", "Q", "r", "R", "s", "S", "u", "U", "v", "V", "w", "W", "x", "X", "y", "Y", "z", "Z"];
    let numwithoutdecimal = input.trunc()
    let numbertodisplay = "";

    let j = 0;
    for i in numwithoutdecimal.to_string().chars() {
        if j >= 2  {
            return;
        }
        numbertodisplay.push(i);
        j = j + 1;
    }

    if (num > (10.0**(abbrevations.len()-1)*3)) {
        format!("{}", format!("{:.2e}",num - 10.0**((abbrevations.len()-1)*3) + "Z"));
    }

    if (numwithoutdecimal.to_string().chars() <= 3) { // below 1k, dont abbreviate at all
        format!("{}",numbertodisplay);
    }

    let indexofabbreviation: f32 = ((numwithoutdecimal.checked_ilog10().unwrap_or(0) + 1) / 3).floor(); // how many triplets of zeroes are in it
    format!("{}", numbertodisplay + abbreviations[indexofabbreviation]);
    
}

fn Emoji(input: f64) {
    let emojicodes = Hashmap.new();
    emojicodes.insert("1","🐦‍🔥");
    emojicodes.insert("2","🍓");
    emojicodes.insert("3","🔱");
    emojicodes.insert("4","💅");
    emojicodes.insert("5","🏳️‍⚧️");
    emojicodes.insert("6","🎲");
    emojicodes.insert("7","🎰");
    emojicodes.insert("8","🎡");
    emojicodes.insert("9","🫨");
    emojicodes.insert("0","🕸️");
    let emojistring = "";
    for i in input.to_string().chars() {
        emojistring = &(emojistring + emojicodes.get(i));
    }
    format!("{}",emojistring);
}
fn Morse(input: f64) {
    let morsecodes = Hashmap.new();
    morsecodes.insert("1",".----");
    morsecodes.insert("2","..---");
    morsecodes.insert("3","...--");
    morsecodes.insert("4","....-");
    morsecodes.insert("5",".....");
    morsecodes.insert("6","-....");
    morsecodes.insert("7","--...");
    morsecodes.insert("8","---..");
    morsecodes.insert("9","----.");
    morsecodes.insert("0","-----");
    let morsestring = "";
    for i in input.to_string().chars() {
        morsestring.push(morsecodes.get(i); + "/");
    }
    format!("{}",morsestring);
}

fn Celeste(input: f64) {
    let celestecodes = Hashmap.new();
    celestecodes.insert("1",":maddyhug:");
    celestecodes.insert("2",":baddyhug:");
    celestecodes.insert("3",":lanihug:");
    celestecodes.insert("4",":radgranny:");
    celestecodes.insert("5",":theoretical:");
    celestecodes.insert("6",":reaperline:");
    celestecodes.insert("7",":fullclear:");
    celestecodes.insert("8",":CrystalHeart:");
    celestecodes.insert("9",":birb:");
    celestecodes.insert("0",":catbus:");
    let celestestring = "";
    for i in input.to_string().chars() {
        celestestring.push(celestecodes.get(i));
    }
    format!("{}",celestestring);
}

fn Heart(input: f64) {
    let heartcodes = Hashmap.new();
    heartcodes.insert("1","❤");
    heartcodes.insert("2","🧡");
    heartcodes.insert("3","💛");
    heartcodes.insert("4","💚");
    heartcodes.insert("5","💙");
    heartcodes.insert("6","💜");
    heartcodes.insert("7","🤎");
    heartcodes.insert("8","🖤");
    heartcodes.insert("9","🤍");
    heartcodes.insert("0","💔");
    let heartstring = "";
    for i in input.to_string().chars() {
        heartstring.push(heartcodes.get(i));
    }
    format!("{}",heartstring);
}

fn Reverse(input: f64) {
    format!("{}",input.to_string()chars().rev().collect::<String>(););
}

fn Blind(_input: f64) {
    format!("{}","");
}


pub fn update(app: &mut Game, ui: &mut Ui, ctx: &egui::Context) {
    
    ui.label(format!(
        "You currently have {}$ (+{}$/s)",
        format(app.currencies[0]),
        format(app.cps)
    ));

    if app.unlocked_tiers[1] {
        ui.label(format!(
            "You have {} strawberries.",
            format(app.currencies[1])
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
            let image = egui::include_image!("../assets/day-5.gif");

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
                                    format(app.cat_multipliers[i])
                                ))
                                .color(app.colors[0]),
                            )
                            .on_hover_text("This cat is Extra Effective!")
                        } else {
                            ui.label(format!(
                                "{} 'Day {}' cats\n[{}]",
                                app.cats[i],
                                i + 1,
                                format(app.cat_multipliers[i])
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
                                        format(app.cat_prices[i])
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
