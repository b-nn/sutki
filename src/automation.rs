use crate::Game;
use egui::Ui;

#[derive(serde::Deserialize, serde::Serialize, Debug, PartialEq, Clone)]
pub enum AutomationMode {
    MostMoney,
    MostStrawberries,
    Efficiency,
}

fn get_strawberries(money: f64, cats: [f64; 31], app: &mut Game) -> f64 {
    (cats.iter().sum::<f64>() / 30.0 - 1.0)
        * if app.challenges[2].count != 0 {
            if money.ln() < 1.0 {
                1.0
            } else {
                money.ln()
            }
        } else {
            1.0
        }
}

fn update_base(app: &mut Game, cats: [f64; 31]) -> f64 {
    app.cat_multipliers = [1.0; 31];
    let mut cps = 0.0;
    for i in 0..app.cat_multipliers.len() {
        app.cat_multipliers[i] += 1.036_f64.powi(31 - i as i32);
        app.cat_multipliers[i] += 1.036_f64.powi(i as i32);
        app.cat_multipliers[i] *= 1.5f64.powi(app.cat_strawberries[i] as i32);
        if app.current_challenge.id == 3 {
            app.cat_multipliers[i] *= 0.9f64.powf(app.cats[i]);
        }
        if app.challenges[3].count > 1 {
            app.cat_multipliers[i] *= (app.challenges[3].count as f64 * 0.05_f64 + 1.0_f64)
                .powf((app.cats[i] / 10.0).floor());
        }
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

fn get_money_gain(app: &mut Game) {
    let base = update_base(app, app.cats);
    for i in 0..app.money_gain_per_cat.len() {
        app.money_gain_per_cat[i] = update_base(
            app,
            app.cats
                .iter()
                .enumerate()
                .map(|(x, y)| y + if x == i { 1.0 } else { 0.0 })
                .collect::<Vec<f64>>()
                .try_into()
                .unwrap(),
        ) - base;
    }
}

fn buy_cat(app: &mut Game, index: usize) {
    app.currencies[0] -= app.cat_prices[index];
    if app.cats[index] == 0.0 {
        for j in 0..app.cat_prices.len() {
            if index != j && app.cats[j] == 0.0 {
                app.cat_price_5_multiplier[j] += 1.0;
            }
        }
    }
    app.cats[index] += 1.0;
}

fn update_cat_prices(app: &mut Game) {
    app.cat_prices = [1.0; 31];
    for i in 0..app.cats.len() {
        app.cat_prices[i] = if app.asleep {
            1.45_f64.powf(app.cats[i]) * 2.1_f64.powi(app.cat_price_5_multiplier[i] as i32)
        } else {
            1.5_f64.powf(app.cats[i]) * 5_f64.powi(app.cat_price_5_multiplier[i] as i32)
        };
    }
}

pub fn buy_best_cat(app: &mut Game) -> bool {
    let mut index = 0;
    get_money_gain(app);
    update_cat_prices(app);
    match app.automation_mode {
        AutomationMode::MostMoney => {
            let mut max = 0.0;
            for i in 0..app.money_gain_per_cat.len() {
                if (app.money_gain_per_cat[i]) > max {
                    max = app.money_gain_per_cat[i];
                    index = i;
                }
            }
            if app.cat_prices[index] < app.currencies[0] && max > 0.0 {
                buy_cat(app, index);
                return true;
            }
        }
        AutomationMode::Efficiency => {
            let percent = app
                .money_gain_per_cat
                .iter()
                .enumerate()
                .map(|(i, x)| x / app.cat_prices[i])
                .collect::<Vec<_>>();
            let mut max = 0.0;
            for i in 0..percent.len() {
                if percent[i] > max {
                    max = percent[i];
                    index = i;
                }
            }
            if app.cat_prices[index] <= app.currencies[0] && app.money_gain_per_cat[index] > 0.0 {
                buy_cat(app, index);
                return true;
            }
        }
        AutomationMode::MostStrawberries => {
            let base = get_strawberries(app.currencies[0], app.cats, app);
            let mut gains = [0.0; 31];
            for i in 0..gains.len() {
                gains[i] = get_strawberries(
                    app.currencies[0] - app.cat_prices[i],
                    app.cats
                        .iter()
                        .enumerate()
                        .map(|(x, y)| y + if x == i { 1.0 } else { 0.0 })
                        .collect::<Vec<f64>>()
                        .try_into()
                        .unwrap(),
                    app,
                ) - base;
            }
            let mut max = 0.0;
            let mut max_price = f64::MAX;
            for i in 0..gains.len() {
                if (gains[i]) > max || (gains[i] == max && max_price > app.cat_prices[i]) {
                    max = gains[i];
                    index = i;
                    max_price = app.cat_prices[i];
                }
            }
            if app.cat_prices[index] < app.currencies[0] && max > 0.0 {
                buy_cat(app, index);
                return true;
            }
        }
    }
    false
}

pub fn update(app: &mut Game, ui: &mut Ui) {
    // ui.label("no tab (for now)");
    // buy_best_cat(app);
    if !app.automation_unlocked {
        ui.heading("You don't have this unlocked yet, please come back later!");
        return;
    }
    let t = &mut app.automation_enabled;
    ui.toggle_value(
        t,
        if *t {
            "Disable Automation"
        } else {
            "Enable Automation"
        },
    );
    let lower_bound = (0.1 - (app.upgrades[11].count as f64 * 0.01)).max(0.001);
    ui.add(
        egui::Slider::new(&mut app.automation_interval, lower_bound..=1.0).text(
            if 0.1 - (app.upgrades[11].count as f64 * 0.01) == 0.0 {
                "Interval [MAX]"
            } else {
                "Interval"
            },
        ),
    );
    egui::ComboBox::from_label("Automation Mode")
        .selected_text(format!("{:?}", app.automation_mode))
        .show_ui(ui, |ui| {
            ui.selectable_value(
                &mut app.automation_mode,
                AutomationMode::MostMoney,
                "Most Money",
            );
            ui.selectable_value(
                &mut app.automation_mode,
                AutomationMode::MostStrawberries,
                "Most Strawberries",
            );
            ui.selectable_value(
                &mut app.automation_mode,
                AutomationMode::Efficiency,
                "Efficiency",
            );
        });
}
