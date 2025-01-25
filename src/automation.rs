use crate::Game;
use egui::Ui;

#[derive(serde::Deserialize, serde::Serialize, Debug, PartialEq, Clone)]
pub enum AutomationMode {
    MostMoney,
    MostStrawberries,
}

pub fn update(app: &mut Game, ui: &mut Ui) {
    let t = &mut app.automation_enabled;
    ui.toggle_value(
        t,
        if *t {
            "Disable Automation"
        } else {
            "Enable Automation"
        },
    );
    ui.add(egui::Slider::new(&mut app.automation_interval, 0.01..=1.0).text("My value"));
    egui::ComboBox::from_label("Automation Mode")
        .selected_text(format!("{:?}", app.automation_mode))
        .show_ui(ui, |ui| {
            ui.selectable_value(
                &mut app.automation_mode,
                AutomationMode::MostMoney,
                "Cheapest Cat",
            );
            ui.selectable_value(
                &mut app.automation_mode,
                AutomationMode::MostStrawberries,
                "Cheapest Cat",
            );
        });
}
