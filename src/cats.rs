use crate::format::formatnum;
use crate::within_day_range;
use crate::Game;
use chrono::{self, Duration, NaiveTime};
use egui::{RichText, Ui};

fn make_daygif(i: usize) -> egui::Image<'static> {
    return match i {
        _ => egui::Image::new(egui::include_image!("../assets/pattern.png")),
        // 1 => egui::Image::new(egui::include_image!("../assets/day-1.gif")),
        // 2 => egui::Image::new(egui::include_image!("../assets/day-2.gif")),
        // 3 => egui::Image::new(egui::include_image!("../assets/day-3.gif")),
        // 4 => egui::Image::new(egui::include_image!("../assets/day-4.gif")),
        // 5 => egui::Image::new(egui::include_image!("../assets/day-5.gif")),
        // 6 => egui::Image::new(egui::include_image!("../assets/day-6.gif")),
        // 7 => egui::Image::new(egui::include_image!("../assets/day-7.gif")),
        // 8 => egui::Image::new(egui::include_image!("../assets/day-8.gif")),
        // 9 => egui::Image::new(egui::include_image!("../assets/day-9.gif")),
        // 10 => egui::Image::new(egui::include_image!("../assets/day-10.gif")),
        // 11 => egui::Image::new(egui::include_image!("../assets/day-11.gif")),
        // 12 => egui::Image::new(egui::include_image!("../assets/day-12.gif")),
        // 13 => egui::Image::new(egui::include_image!("../assets/day-13.gif")),
        // 14 => egui::Image::new(egui::include_image!("../assets/day-14.gif")),
        // 15 => egui::Image::new(egui::include_image!("../assets/day-15.gif")),
        // 16 => egui::Image::new(egui::include_image!("../assets/day-16.gif")),
        // 17 => egui::Image::new(egui::include_image!("../assets/day-17.gif")),
        // 18 => egui::Image::new(egui::include_image!("../assets/day-18.gif")),
        // 19 => egui::Image::new(egui::include_image!("../assets/day-19.gif")),
        // 20 => egui::Image::new(egui::include_image!("../assets/day-20.gif")),
        // 21 => egui::Image::new(egui::include_image!("../assets/day-21.gif")),
        // 22 => egui::Image::new(egui::include_image!("../assets/day-22.gif")),
        // 23 => egui::Image::new(egui::include_image!("../assets/day-23.gif")),
        // 24 => egui::Image::new(egui::include_image!("../assets/day-24.gif")),
        // 25 => egui::Image::new(egui::include_image!("../assets/day-25.gif")),
        // 26 => egui::Image::new(egui::include_image!("../assets/day-26.gif")),
        // 27 => egui::Image::new(egui::include_image!("../assets/day-27.gif")),
        // 28 => egui::Image::new(egui::include_image!("../assets/day-28.gif")),
        // 29 => egui::Image::new(egui::include_image!("../assets/day-29.gif")),
        // 30 => egui::Image::new(egui::include_image!("../assets/day-30.gif")),
        // 31 => egui::Image::new(egui::include_image!("../assets/day-31.gif")),
        // _ => egui::Image::new(egui::include_image!("../assets/what.gif")),
    };
}

pub fn update(app: &mut Game, ui: &mut Ui, ctx: &egui::Context) {
    ui.label(format!(
        "You currently have {}$ (+{}$/s)",
        formatnum(&app.notation_format, app.currencies[0]),
        formatnum(&app.notation_format, app.cps)
    ));
    if app.unlocked_tiers[1] {
        ui.label(format!(
            "You have {} strawberries.",
            formatnum(&app.notation_format, app.currencies[1])
        ));
    }
    if app.unlocked_tiers[2] {
        ui.label(format!(
            "You have {} golden strawberries.",
            formatnum(&app.notation_format, app.currencies[2])
        ));
    }

    let tomorrow_midnight = (app.date + Duration::days(1))
        .with_time(NaiveTime::from_hms_opt(0, 0, 0).unwrap())
        .unwrap();

    ui.label(format!(
        "{} seconds until tomorrow.",
        formatnum(
            &app.notation_format,
            (tomorrow_midnight - app.date).num_seconds() as f64
                / 2_f64.powi(app.upgrades[2].count as i32)
        ),
    ));
    egui::ScrollArea::vertical()
        .drag_to_scroll(false)
        .show(ui, |ui| {
            ui.set_min_width(330.0);

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
                        let cell_width = (ctx.screen_rect().width() - 8.0 * 8.0) / 7.0;
                        let cell_height = cell_width / 2.0;
                        let mut available_height = cell_height;

                        // ui.add(make_daygif(i).max_height(height).max_width(width));
                        // I'll try getting the gifs to work later, though gifs may look ugly idk
                        // ui.add(
                        //     make_daygif(i)
                        //         .maintain_aspect_ratio(false)
                        //         .fit_to_exact_size([cell_width, cell_height].into()),
                        // );

                        let image_layer = egui::LayerId {
                            order: egui::Order::Background,
                            id: "image".into(),
                        };
                        let interact_layer = egui::LayerId {
                            order: egui::Order::Background,
                            id: "interact".into(),
                        };

                        let mut t = ui.new_child(egui::UiBuilder::new().layer_id(image_layer));
                        // t.add(
                        //     make_daygif(i)
                        //         .texture_options(egui::TextureOptions {
                        //             mipmap_mode: Some(egui::TextureFilter::Nearest),
                        //             ..Default::default()
                        //         })
                        //         .maintain_aspect_ratio(false)
                        //         .fit_to_exact_size([cell_width, cell_height].into()),
                        // );

                        ui.scope_builder(egui::UiBuilder::new().layer_id(interact_layer), |ui| {
                            let extra_effective =
                                within_day_range(app.day, app.day_width, i as u32)
                                    && !app.asleep
                                    && app.current_challenge.id != 0;
                            available_height -= ui
                                .label(
                                    RichText::new(format!(
                                        "{} 'Day {}' cats\n[{}]",
                                        formatnum(&app.notation_format, app.cats[i]),
                                        i + 1,
                                        formatnum(&app.notation_format, app.cat_multipliers[i])
                                    ))
                                    .color(
                                        if extra_effective {
                                            egui::Color32::from_hex("#FF784F").unwrap()
                                        } else {
                                            egui::Color32::from_hex("#888888").unwrap()
                                        },
                                    ),
                                )
                                .on_hover_text(if extra_effective {
                                    "This cat is Extra Effective!"
                                } else {
                                    "Self-explanatory."
                                })
                                .rect
                                .height();

                            available_height = available_height.max(10.0);
                            if app.unlocked_tiers[1] {
                                if ui
                                    .add_enabled_ui(
                                        app.currencies[1]
                                            >= app.cat_strawberry_prices[i].pow(2) as f64,
                                        |ui| {
                                            ui.add_sized(
                                                [cell_width, available_height / 2.0],
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
                                available_height -= 3.0;
                                available_height /= 2.0;
                            }

                            available_height = available_height.max(10.0);
                            if ui
                                .add_enabled_ui(app.cat_prices[i] <= app.currencies[0], |ui| {
                                    ui.add_sized(
                                        [cell_width, available_height],
                                        egui::Button::new(format!(
                                            "Hire another cat {}$",
                                            formatnum(&app.notation_format, app.cat_prices[i])
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
                        });
                        ctx.set_sublayer(image_layer, interact_layer);
                    });
                }
            });
        });
}
