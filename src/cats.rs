use crate::formatnum;
use crate::prestigetwo_placeholder;
use crate::within_day_range;
use crate::Game;
use chrono::{self, Duration, NaiveTime};
use egui::Image;
use egui::Rect;
use egui::Stroke;
use egui::{RichText, Ui};

// enum CatInfo {
//     Multiplier,
//     Efficiency,
//     Cps,
// }

// pub struct Cats {
//     info_type: CatInfo,
// }


fn make_daygif(i: usize) -> egui::Image<'static> {
    return match i {
        1 => Image::new(egui::include_image!("../assets/day-1.gif")),
        2 => Image::new(egui::include_image!("../assets/day-2.gif")),
        3 => Image::new(egui::include_image!("../assets/day-3.gif")),
        4 => Image::new(egui::include_image!("../assets/day-4.gif")),
        5 => Image::new(egui::include_image!("../assets/day-5.gif")),
        6 => Image::new(egui::include_image!("../assets/day-6.gif")),
        7 => Image::new(egui::include_image!("../assets/day-7.gif")),
        8 => Image::new(egui::include_image!("../assets/day-8.gif")),
        9 => Image::new(egui::include_image!("../assets/day-9.gif")),
        10 => Image::new(egui::include_image!("../assets/day-10.gif")),
        11 => Image::new(egui::include_image!("../assets/day-11.gif")),
        12 => Image::new(egui::include_image!("../assets/day-12.gif")),
        13 => Image::new(egui::include_image!("../assets/day-13.gif")),
        14 => Image::new(egui::include_image!("../assets/day-14.gif")),
        15 => Image::new(egui::include_image!("../assets/day-15.gif")),
        16 => Image::new(egui::include_image!("../assets/day-16.gif")),
        17 => Image::new(egui::include_image!("../assets/day-17.gif")),
        18 => Image::new(egui::include_image!("../assets/day-18.gif")),
        19 => Image::new(egui::include_image!("../assets/day-19.gif")),
        20 => Image::new(egui::include_image!("../assets/day-20.gif")),
        21 => Image::new(egui::include_image!("../assets/day-21.gif")),
        22 => Image::new(egui::include_image!("../assets/day-22.gif")),
        23 => Image::new(egui::include_image!("../assets/day-23.gif")),
        24 => Image::new(egui::include_image!("../assets/day-24.gif")),
        25 => Image::new(egui::include_image!("../assets/day-25.gif")),
        26 => Image::new(egui::include_image!("../assets/day-26.gif")),
        27 => Image::new(egui::include_image!("../assets/day-27.gif")),
        28 => Image::new(egui::include_image!("../assets/day-28.gif")),
        29 => Image::new(egui::include_image!("../assets/day-29.gif")),
        30 => Image::new(egui::include_image!("../assets/day-30.gif")),
        31 => Image::new(egui::include_image!("../assets/day-31.gif")),
        _ => Image::new(egui::include_image!("../assets/what.gif")),
    }; // fuckin egui forcing me to do this shit
}

pub fn update(app: &mut Game, ui: &mut Ui, ctx: &egui::Context) {
    ui.set_min_width(360.0); // yeah this isnt working for me
    ui.set_min_height(180.0);
    ui.label(format!(
        "You currently have {}$ (+{}$/s)",
        formatnum(app, app.currencies[0]),
        formatnum(app, app.cps)
    ));
    if app.unlocked_tiers[1] {
        ui.label(format!(
            "You have {} strawberries.",
            formatnum(app, app.currencies[1])
        ));
    }
    if app.unlocked_tiers[2] {
        ui.label(format!(
            "You have {} golden strawberries.",
            formatnum(app, app.currencies[2])
        ));
        prestigetwo_placeholder::update(app, ui);
    }
    
    let tomorrow_midnight = (app.date + Duration::days(1))
        .with_time(NaiveTime::from_hms_opt(0, 0, 0).unwrap())
        .unwrap();

    ui.label(format!(
        "{} seconds until tomorrow.",
        formatnum(app, (tomorrow_midnight - app.date).num_seconds() as f64
            / 2_f64.powi(app.upgrades[2].count as i32)),
    ));
    egui::ScrollArea::vertical().drag_to_scroll(false).show(ui, |ui| {
            // cat_handler(app, ui);
            egui::Grid::new("GRIDDDD").striped(true).show(ui, |ui| {
                ui.label("Monday"); //blahblah you get what this does
                ui.label("Tuesday");
                ui.label("Wednesday");
                ui.label("Thurday");
                ui.label("Friday");
                ui.label("Saturday");
                ui.label("Sunday");
                ui.end_row();

                let mut x = ctx.screen_rect().width() / 140.0;
                let mut y = 100.0;

                for day in 0..app.cats.len() { // generate the grid of cats
                    let width = (ctx.screen_rect().width().max(65.0) - 8.0 * 8.0) / 7.0;
                    let height =  (ctx.screen_rect().width().max(109.0) - 8.0 * 3.0) / 28.0 - 3.0;
                    if day % 7 == 0 && day != 0 { // next week
                        y += 1.0;
                        ui.end_row();
                    }
                    ui.vertical(|ui| {
                        //let mut size = (ctx.screen_rect().width() - 8.0 * 3.0) / 14.0;
                        // originals:
                        // size = (ctx.screen_rect().width() - 8.0 * 3.0) / 14.0;
                        // height = (ctx.screen_rect().width() - 8.0 * 8.0) / 7.0,
                        // width size / 2.0 - 3.0

                        // size -= if within_day_range(app.day, app.day_width, i as u32) && !app.asleep && app.current_challenge.id != 0
                        // {
                        //     ui.label( // extra effective cat
                        //         RichText::new(format!(
                        //             "{} 'Day {}' cats\n[{}]",
                        //             formatnum(app, app.cats[i]),
                        //             i + 1,
                        //             formatnum(app, app.cat_multipliers[i])
                        //         ))
                        //         .color(egui::Color32::from_hex("#FF784F").unwrap()), // i have no idea why this took app.colors when its literally only used once but 
                        //     )
                        //     .on_hover_text("This cat is Extra Effective!")
                        // } else { // not effective cat
                        //     ui.label(format!(
                        //         "{} 'Day {}' cats\n[{}]",
                        //         formatnum(app, app.cats[i]),
                        //         i + 1,
                        //         formatnum(app, app.cat_multipliers[i])
                        //     ))
                        // }.rect.height().min(10.0); // subtracts the size of the cat cell from size

                        // what the fuck am i doing??

                        let _gif_widget = make_daygif(day + 1)
                        .maintain_aspect_ratio(false).fit_to_exact_size(egui::Vec2::new(width, height * 2.0));

                        let gif_rect = Rect::from_min_max(egui::pos2(x, y), egui::pos2(x + width, y + height * 2.0));
                        ui.painter().rect(gif_rect, 0.0, egui::Color32::from_black_alpha(50),Stroke::new(1.0,egui::Color32::from_black_alpha(50))); // Optional: add a color for visibility

                        let extra_effective = 
                            within_day_range(app.day, app.day_width, day as u32) 
                                && !app.asleep 
                                && app.current_challenge.id != 0;
                        ui.label(
                            RichText::new(format!(
                                "{} 'Day {}' cats\n[{}]",
                                formatnum(app, app.cats[day]),
                                day + 1,
                                formatnum(app, app.cat_multipliers[day])
                            ))
                            .color(if extra_effective {egui::Color32::from_hex("#FF784F").unwrap()} else {egui::Color32::from_hex("#888888").unwrap()}), // i have no idea why this took app.colors when its literally only used once but 
                        )
                        .on_hover_text(if extra_effective {"This cat is Extra Effective!"} else {""});

                        if ui
                            .add_enabled_ui(app.cat_prices[day] <= app.currencies[0], |ui| { // i cant read this.
                                ui.add_sized(
                                    [width,height],
                                    egui::Button::new(format!(
                                        "Hire another cat {}$",
                                        formatnum(app, app.cat_prices[day])
                                    )),
                                )
                            }).inner.on_hover_text(
                                format!("x{} to self, x5 to all other unbought cats",app.cat_price_multipliers[day]))
                                .clicked()
                        {
                            app.currencies[0] -= app.cat_prices[day]; // subtract money
                            if app.cats[day] == 0.0 { // hire new cat
                                for j in 0..app.cat_prices.len() { //gets other cat
                                    if day != j && app.cats[j] == 0.0 {
                                        app.cat_price_5_multiplier[j] += 1.0; // multiplies by 5
                                    }
                                }
                            }
                            app.cats[day] += 1.0; // adds 1 cat
                        }

                        if app.unlocked_tiers[1] { // feed cat strawberries
                            if ui
                                .add_enabled_ui(
                                    app.currencies[1] >= app.cat_strawberry_prices[day].pow(2) as f64,
                                    |ui| {
                                        ui.add_sized(
                                            [width,height],
                                            egui::Button::new(format!(
                                                "Feed cat {} strawberry",
                                                app.cat_strawberry_prices[day].pow(2)
                                            )),
                                        ).on_hover_text("Gives a 1.5x boost to this cat")
                                    },
                                )
                                .inner
                                .clicked()
                            {
                                app.currencies[1] -= app.cat_strawberry_prices[day].pow(2) as f64; // subtract strawberries
                                app.cat_strawberries[day] += 1; // feed
                                app.cat_strawberry_prices[day] += 1; // increase price
                            }
                        }

                        x += width + 10.0;
 
                    }); //ui.vertical

                } // for loop

                // ui.set_min_height((ctx.screen_rect().width() - 8.0 * 8.0) / 7.0);
            }); // grid (too far)

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
