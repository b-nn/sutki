use crate::get_upgrades;
use crate::Game;
use egui::Ui;

pub fn update(app: &mut Game, ui: &mut Ui) {
    if ui
        .add_enabled(
            app.cats.iter().sum::<f64>() >= 60.0,
            egui::Button::new(format!(
                "Prestige for {:.2} strawberries",
                app.cats.iter().sum::<f64>() / 30.0 - 1.0
            )),
        )
        .clicked()
    {
        app.currencies[1] += app.cats.iter().sum::<f64>() / 30.0 - 1.0;
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
