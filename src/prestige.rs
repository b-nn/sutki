use crate::get_upgrades;
use crate::Game;
use egui::Ui;

fn get_strawberry_amount(app: &mut Game) -> f64 {
    (app.cats.iter().sum::<f64>() / 30.0 - 1.0)
        * if app.challenges[2].count != 0 {
            if app.currencies[0].log10() < 1.0 {
                1.5
            } else {
                1.5_f64.powf(app.currencies[0].log10())
            }
        } else {
            1.0
        }
}

pub fn update(app: &mut Game, ui: &mut Ui) {
    let strawberries = get_strawberry_amount(app);
    if ui
        .add_enabled(
            app.cats.iter().sum::<f64>() >= 60.0,
            egui::Button::new(format!("Prestige for {:.2} strawberries", strawberries)),
        ).on_hover_text("Gives strawberries based off how many cats you have".to_owned())
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
