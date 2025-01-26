use crate::change_status;
use crate::load_game;
use crate::TABS;
use crate::SaveStruct;
use crate::Game;
use crate::MODULES;
use egui::Ui;

pub fn update(app: &mut Game, ui: &mut Ui) {
    ui.horizontal(|ui| {
        let mut index = 0;
        for j in TABS.iter().enumerate() {
            index += 1;
            ui.vertical(|ui| {
                ui.label(j.1.0);
                for i in MODULES.iter().enumerate() {
                    ui.add_enabled(i.0 != 2 || 
                        app.modules.iter().map(|x| x[2] as i32).sum::<i32>() > 1 || 
                        !app.modules[i.0][j.0], 
                        |ui: &mut Ui| {
                        ui.checkbox(&mut app.modules[j.0][i.0], *i.1)
                    }).on_disabled_hover_text("You cannot remove the settings module from all tabs, add it to another tab first.");
                }
            });
            if index != TABS.len() {ui.separator();};
        }
    });

    if ui.button("Export save to clipboard").clicked() {
        let t = SaveStruct {
            cats: app.cats,
            day_offset: 0.0,
            day_width: app.day_width,
            cat_prices: app.cat_prices,
            cat_times: app.cat_times,
            currencies: app.currencies,
            upgrades: app
                .upgrades
                .iter()
                .map(|x| (x.text.to_owned(), x.count, x.max))
                .collect(),
            cat_strawberries: app.cat_strawberries,
            cat_strawberry_prices: app.cat_strawberry_prices,
            unlocked_tiers: app.unlocked_tiers,
            cat_price_5_multiplier: app.cat_price_5_multiplier,
            modules: app.modules,
            challenges: app
                .challenges
                .iter()
                .map(|x| (x.description.to_owned(), x.count, x.max))
                .collect(),
            current_challenge: app.current_challenge.id,
            in_challenge: app.in_challenge,
            automation_interval: app.automation_interval,
            automation_enabled: app.automation_enabled,
            automation_mode: app.automation_mode.clone(),
        };

        match ron::to_string(&t) {
            Ok(text) => {
                ui.output_mut(|x| x.copied_text = text);
            }
            Err(t) => {
                change_status(log::Level::Error, "Failed to export save, copied error to clipboard.", &mut app.status, &mut app.status_time);
                ui.output_mut(|x| x.copied_text = format!("Error while exporting save: {}", t));
            }
        }
    }

    if ui.button("Import save from text field").clicked() {
        if app.settings_text_field.is_empty() {
            app.settings_text_field = "Please put in your save file!".to_string();
        }
        match ron::from_str::<SaveStruct>(&app.settings_text_field) {
            Ok(t) => {
                *app = load_game(t);
            }
            Err(t) => {
                change_status(log::Level::Error, "Failed to import save, copied error to clipboard.", &mut app.status, &mut app.status_time);
                ui.output_mut(|text| {
                    text.copied_text = format!("Error while importing save: {}", t);
                });
            }
        };
    }

    ui.add(egui::TextEdit::singleline(&mut app.settings_text_field));


    // let t = egui::Grid::new("settings_id");
    // let t = t.spacing(Vec2::new(30.0, 0.0));
    // t.show(ui, |ui| {
    //     for i in TABS {
    //         ui.label(i.0);
    //     }
    //     ui.end_row();

    //     for i in MODULES.iter().enumerate() {
    //         for j in TABS.iter().enumerate() {
    //             ui.add_enabled(i.0 != 2 || 
    //                 app.modules.iter().map(|x| x[2] as i32).sum::<i32>() > 1 || 
    //                 !app.modules[i.0][j.0], 
    //                 |ui: &mut Ui| {
    //                 ui.checkbox(&mut app.modules[j.0][i.0], *i.1)
    //             }).on_disabled_hover_text("You cannot remove the settings module from all tabs, add it to another tab first.");
    //         }
    //         ui.end_row();
    //     }
    // });
}
