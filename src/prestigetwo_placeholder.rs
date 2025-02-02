use crate::formatnum;
use crate::challenges::Challenge;
use crate::Game;
use rand;
use egui::Ui;

const GOLD_BASE_CHANCE: f64 = 0.001;

fn get_goldberry_chance(app: &mut Game) -> f64 {
    let tier_two_completions: Vec<&Challenge> = app.challenges.iter()
        .filter(|item| item.tier == 2)
        .collect();
    return GOLD_BASE_CHANCE * tier_two_completions.iter().count() as f64
}

pub fn update(app: &mut Game, ui: &mut Ui) {
    if app.unlocked_tiers[2] {
        let goldberrieschance = get_goldberry_chance(app);
        ui.label(format!("Golden Strawberry Chance: {}% ({} Tier 2 Challenges)"
        , formatnum(app, goldberrieschance), formatnum(app, goldberrieschance/GOLD_BASE_CHANCE)))
        .on_hover_text("Random chance of golden strawberries \n based off how many tier 2 challenges you completed".to_owned());
        
         let randval: f32 = rand::random_range(0.0..(1.0/goldberrieschance) as f32); //maybe 1/1000 by default is too lenient
         if randval == 0.0 {
             app.currencies[2] += 1.0; // upgrade to increase this later?
         }
    }

}
