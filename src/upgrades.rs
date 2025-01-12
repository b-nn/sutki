use crate::MyEguiApp;

pub struct Upgrade {
    pub text: String,
    pub description: String,
    pub price: f64,
    pub price_mult: f64,
    pub max: i64,
    pub count: i64,
    pub effect: fn(&mut MyEguiApp, i64),
    pub tier: usize,
}

pub fn get_upgrades() -> Vec<Upgrade> {
    vec![
        Upgrade {
            text: "Early Bird".to_owned(),
            description:
                "Gives a boost to cats depending on how early in the month they are"
                    .to_owned(),
            price: 500.0,
            max: 1,
            price_mult: 1.0,
            count: 0,
            effect: |x, _y| {
                for i in 0..x.cat_multipliers.len() {
                    x.cat_multipliers[i] += 1.036_f64.powi(31 - i as i32);
                }
                x.upgrades[1].price = 1500.0;
            },
            tier: 0
        },
        Upgrade {
            text: "Late Bird".to_owned(),
            description:
                "Gives a boost to cats depending on how late in the month they are"
                    .to_owned(),
            price: 500.0,
            price_mult: 1.0,
            max: 1,
            count: 0,
            effect: |x, _y| {
                for i in 0..x.cat_multipliers.len() {
                    x.cat_multipliers[i] += 1.036_f64.powi(i as i32);
                }
                x.upgrades[0].price = 1500.0;
            },
            tier: 0
        },
        Upgrade {
            text: "Faster Spin".to_owned(),
            description: "Makes the 'Extra Effective' boost cycle through 50% faster"
                .to_owned(),
            price: 200.0,
            price_mult: 1.4,
            max: 20,
            count: 0,
            effect: |x, y| {
                x.day_offset += x.dt * (2_f64.powi(y as i32) - 1.0);
            },
            tier: 0
        },
        Upgrade {
            text: "..Wider Spin?".to_owned(),
            description:
                "Makes an additional cat get the 'Extra Effective' boost at the same time"
                    .to_owned(),
            price: 1000.0,
            price_mult: 1.3,
            max: 25,
            count: 0,
            effect: |x, y| {
                x.day_width = y;
            },
            tier: 0
        },
        Upgrade {
            text: "Cat Synergy".to_owned(),
            description: "Buying cats increases the multiplier of all other cats"
                .to_owned(),
            price: 10000.0,
            price_mult: 1.0,
            max: 1,
            count: 0,
            effect: |x, _y| {
                if x.asleep { return; }
                for i in 0..x.cat_multipliers.len() {
                    x.cat_multipliers[i] *=
                        x.cats.iter().enumerate().map(|(x, y)| if x == i  {
                            0.0
                        } else{*y * 0.01}).sum::<f64>() + 1.0;
                }
            },
            tier: 0
        },
        Upgrade {
            text: "Like Hot Cakes".to_owned(),
            description: "Gives a temporary boost to cats when they get the 'Extra Effective' boost which falls off over time"
                .to_owned(),
            price: 10000.0,
            price_mult: 1.0,
            max: 1,
            count: 0,
            effect: |x, _y| {
                if x.asleep { return; }
                for i in 0..x.cat_multipliers.len() {
                    if x.cat_times[i] < 0.0 || x.asleep { continue; }
                    x.cat_multipliers[i] *= 1.2f64.powf(5.0 - x.cat_times[i]) + 1.0;
                }
            },
            tier: 0
        },
        Upgrade {
            text: "Sleep".to_owned(),
            description: "Disables the 'Extra Effective' boost and the previous 2 upgrades, but makes cats much cheaper"
                .to_owned(),
            price: 15000.0,
            price_mult: 1.0,
            max: 1,
            count: 0,
            effect: |x, _y| {
                x.asleep = true;
            },
            tier: 0
        },
        Upgrade {
            text: "Appetizing aroma".to_owned(),
            description: "Increases the production of all cats based on how many strawberries you have"
                .to_owned(),
            price: 5.0,
            price_mult: 1.0,
            max: 1,
            count: 0,
            effect: |x, _y| {
                for i in 0..x.cats.len() {
                    x.cat_multipliers[i] *= (x.currencies[1] / 10.0) + 1.0;
                }
            },
            tier: 1
        },
    ]
}
