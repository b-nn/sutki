use crate::get_upgrades;
use crate::Game;
use egui::Ui;
use log::log;

#[derive(Clone, Debug)]
pub struct Challenge {
    pub id: usize,
    pub description: String,
    pub goal: f64,
    pub currency: usize,
    pub count: i64,
    pub max: i64,
    pub effect: fn(&mut Game, i64),
    pub boost: fn(&mut Game, i64),
}

impl Default for Challenge {
    fn default() -> Self {
        Challenge {
            id: 1000000,
            description: "".to_owned(),
            goal: 0.0,
            currency: 0,
            count: 0,
            max: 0,
            effect: |_x, _y| loop {
                log!(log::Level::Error, "Something has gone very wrong, you are never meant to run this, please file a bug report!");
                println!("something has gone very wrong, you are never meant to run this, please report this bug!");
            },
            boost: |_x, _y| loop {
                log!(log::Level::Error, "Something has gone very wrong, you are never meant to run this, please file a bug report!");
                println!("something has gone very wrong, you are never meant to run this");
            },
        }
    }
}

pub fn get_challenges() -> Vec<Challenge> {
    vec![Challenge {
        id: 0,
        description: "Disables the 'Extra Effective!' boost in challenge. \nGives the cat which was most recently boosted an additional boost.".to_owned(),
        count: 0,
        max: 1,
        goal: 75000.0,
        currency: 0,
        effect: |app, _y| {
            app.cat_multipliers = [1.0; 31];
            app.cat_prices = [1.0; 31];
            let mut cps = 0.0;
            for i in 0..app.upgrades.len() {
                if app.upgrades[i].count > 0 {
                    (app.upgrades[i].effect)(app, app.upgrades[i].count);
                }
            }

            for i in 0..app.cats.len() {
                app.cat_prices[i] = if app.asleep {
                    1.45_f64.powf(app.cats[i]) * 2.1_f64.powi(app.cat_price_5_multiplier[i] as i32)
                } else {
                    1.5_f64.powf(app.cats[i]) * 5_f64.powi(app.cat_price_5_multiplier[i] as i32)
                };
                app.cat_multipliers[i] *= 1.5f64.powi(app.cat_strawberries[i] as i32);
            }

            cps += app
                .cats
                .iter()
                .zip(app.cat_multipliers.iter())
                .map(|(x, y)| x * y)
                .sum::<f64>();
            app.currencies[0] += cps * app.dt;
            app.cps = cps;
        },
        boost: |x, y| {
            if !x.asleep {
                x.cat_multipliers[(x.day + x.day_width).rem_euclid(31) as usize] *=
                    3_f64.powi(y as i32);
            }
            x.challenges[0].goal = (x.challenges[0].count + 1) as f64 * 75000.0;
            println!("boost is being applied! {}", 3_f64.powi(y as i32));

        },
    }]
}

pub fn update(app: &mut Game, ui: &mut Ui) {
    if ui
        .add_enabled(
            app.in_challenge,
            egui::Button::new(if !app.in_challenge {
                "You are currently not in any challenge.".to_owned()
            } else {
                format!(
                    "{} {}",
                    if app.currencies[app.current_challenge.currency] > app.current_challenge.goal {
                        "Complete Challenge".to_owned()
                    } else {
                        "Exit Challenge".to_owned()
                    },
                    app.current_challenge.id
                )
            }),
        )
        .clicked()
    {
        if app.currencies[app.current_challenge.currency] > app.current_challenge.goal {
            app.challenges[app.current_challenge.id].count += 1;
        }
        app.in_challenge = false;
        app.current_challenge = Challenge::default();
        println!("oop!");
    }

    for i in 0..app.challenges.len() {
        let challenge = &app.challenges[i];
        if ui
            .button(format!(
                "Challenge #{:02} ({}{}) [{}/{}] \n{}",
                challenge.id,
                challenge.goal,
                app.currency_symbols[challenge.currency],
                challenge.count,
                challenge.max,
                challenge.description,
            ))
            .clicked()
        {
            if challenge.count >= challenge.max {
                continue;
            }
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
            app.in_challenge = true;
            app.current_challenge = challenge.clone();
        };
    }
}
