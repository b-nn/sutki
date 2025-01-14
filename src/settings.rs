use crate::TABS;
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
