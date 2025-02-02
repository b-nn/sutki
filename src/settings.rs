use std::collections::HashMap;

use crate::{change_status, load_game, save_game};
use crate::{TABS, SaveStruct, Game, MODULES};
use egui::Ui;
use crate::Notations;

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

    // Create a HashMap to map Notations to their display strings
    let mut notation_map: HashMap<Notations, &str> = HashMap::new();

    // Populate the HashMap with Notation variants and their corresponding strings
    notation_map.insert(Notations::Scientific, "Scientific");
    notation_map.insert(Notations::HybridScientific, "Hybrid Scientific");
    notation_map.insert(Notations::Standard, "Standard");
    notation_map.insert(Notations::Engineering, "Engineering");
    notation_map.insert(Notations::None, "None");
    notation_map.insert(Notations::Binary, "Binary");
    notation_map.insert(Notations::Hex, "Hex");
    notation_map.insert(Notations::Logarithm, "Logarithm");
    notation_map.insert(Notations::Leaf, "Leaf");
    notation_map.insert(Notations::Emoji, "ｅｍｏｊｉ");
    notation_map.insert(Notations::Morse, "-- --- .-. ... .");
    notation_map.insert(Notations::Celeste, "Celeste");
    notation_map.insert(Notations::Heart, "Heart");
    notation_map.insert(Notations::Reverse, "Reverse");
    notation_map.insert(Notations::Blind, "");
    
    ui.label("Notation:"); //before you ask, no i cant make a for loop becuase that makes the menu unusable (cycles forever)
    egui::ComboBox::from_label("Select one!")
        .selected_text(format!("{}", notation_map.get(&app.notation_format).unwrap()).trim_matches('"'))
        .show_ui(ui, |ui| {
            ui.selectable_value(&mut app.notation_format, Notations::Scientific
                , format!("{}",notation_map.get(&Notations::Scientific).unwrap()));
            ui.selectable_value(&mut app.notation_format, Notations::HybridScientific
                , format!("{}",notation_map.get(&Notations::HybridScientific).unwrap()));
            ui.selectable_value(&mut app.notation_format, Notations::Standard
                , format!("{}",notation_map.get(&Notations::Standard).unwrap()));
            ui.selectable_value(&mut app.notation_format, Notations::Engineering
                , format!("{}",notation_map.get(&Notations::Engineering).unwrap()));
            ui.selectable_value(&mut app.notation_format, Notations::None
                , format!("{}",notation_map.get(&Notations::None).unwrap()));
            ui.selectable_value(&mut app.notation_format, Notations::Binary
                , format!("{}",notation_map.get(&Notations::Binary).unwrap()));
            ui.selectable_value(&mut app.notation_format, Notations::Hex
                , format!("{}",notation_map.get(&Notations::Hex).unwrap()));
            ui.selectable_value(&mut app.notation_format, Notations::Logarithm
                , format!("{}",notation_map.get(&Notations::Logarithm).unwrap()));
            ui.selectable_value(&mut app.notation_format, Notations::Leaf
                , format!("{}",notation_map.get(&Notations::Leaf).unwrap()));
            ui.selectable_value(&mut app.notation_format, Notations::Emoji
                , format!("{}",notation_map.get(&Notations::Emoji).unwrap()));
            ui.selectable_value(&mut app.notation_format, Notations::Morse
                , format!("{}",notation_map.get(&Notations::Morse).unwrap()));
            ui.selectable_value(&mut app.notation_format, Notations::Celeste
                , format!("{}",notation_map.get(&Notations::Celeste).unwrap()));
            ui.selectable_value(&mut app.notation_format, Notations::Heart
                , format!("{}",notation_map.get(&Notations::Heart).unwrap()));
            ui.selectable_value(&mut app.notation_format, Notations::Reverse
                , format!("{}",notation_map.get(&Notations::Reverse).unwrap()));
            ui.selectable_value(&mut app.notation_format, Notations::Blind
                , format!("{}",notation_map.get(&Notations::Blind).unwrap()));
        }
    );

    ui.add(egui::Checkbox::new(&mut app.uwumode, "uwutext mode"));

    if ui.button("Export save to clipboard").clicked() {
        let t = save_game(app);

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

    if ui.button("Increase UI Scale").clicked() {
        app.zoom += 0.1;
    }
    if ui.button("Decrease UI Scale").clicked() {
        app.zoom -= 0.1;
    }

    ui.horizontal(|ui| {
        ui.label("Current UI Scale: ");
        ui.add(egui::DragValue::new(&mut app.zoom).speed(0.001));
        app.zoom = app.zoom.clamp(0.4, 2.0);
    });
    

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
