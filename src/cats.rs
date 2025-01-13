use crate::get_upgrades;
use crate::within_day_range;
use crate::Game;
use chrono::{self, Duration, NaiveTime};
use egui::{RichText, Ui};

pub fn update(mut app: &mut Game, ui: &mut Ui) {
    ui.heading("sutki [W.I.P. name]");
    ui.label(format!(
        "You currently have {}$ (+{:.2}$/s)",
        app.currencies[0].round(),
        app.cps
    ));
    if app.unlocked_tiers[1] {
        ui.label(format!("You have {:.2} strawberries.", app.currencies[1]));
    }
    let tomorrow_midnight = (app.date + Duration::days(1))
        .with_time(NaiveTime::from_hms_opt(0, 0, 0).unwrap())
        .unwrap();

    ui.label(format!(
        "{:.2} seconds until tomorrow.",
        (tomorrow_midnight - app.date).num_seconds() as f64
            / 2_f64.powi(app.upgrades[2].count as i32),
    ));

    egui::ScrollArea::vertical().show(ui, |ui| {
        ui.set_min_width(330.0);

        // cat_handler(app, ui);

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
