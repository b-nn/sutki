use crate::Game;
use crate::{format::formatnum, get_upgrades};
use egui::Ui;

fn get_strawberry_amount(app: &mut Game) -> f64 {
    (app.cats.iter().sum::<f64>() / 30.0 - 1.0)
        * if app.challenges[2].count != 0 {
            if app.currencies[0].ln() < 1.0 {
                1.0
            } else {
                app.currencies[0].ln()
            }
        } else {
            1.0
        }
    // there's no reason to put a .max() here, letting it be negative just gives the player an
    // indicator for how long they have until they can prestige for strawberries
}

pub fn update(app: &mut Game, ui: &mut Ui) {
    let strawberries = get_strawberry_amount(app);
    if ui
        .add_enabled(
            app.cats.iter().sum::<f64>() >= 60.0,
            egui::Button::new(format!(
                "Prestige for {} strawberries",
                formatnum(&app.notation_format, strawberries)
            )),
        )
        .on_hover_text("Gives strawberries based off how many cats you have".to_owned())
        .clicked()
    {
        app.currencies[1] += strawberries;
        app.cat_prices = [1.0; 31];
        app.cats = [0.0; 31];
        for i in 0..app.upgrades.len() {
            if app.upgrades[i].tier < 1 {
                let mut t = get_upgrades();
                app.upgrades[i] = t.remove(i);
            }
        }
        app.currencies[0] = 1.0;
        app.day_width = 0;
        app.unlocked_tiers[1] = true;
        app.day_offset = 0.0;
        app.asleep = false;
        app.cat_price_5_multiplier = [0.0; 31];
    }
}
