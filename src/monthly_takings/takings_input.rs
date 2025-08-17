use eframe::egui;

use crate::monthly_takings::{Date, create_label, date_picker::Day};

pub struct TakingsField {
    takings: f32,
    text: String,
    text_highlighted: bool,
}
impl TakingsField {
    pub fn new() -> Self {
        Self {
            takings: 0.0,
            text: String::from("0"),
            text_highlighted: false,
        }
    }
    pub fn show(&mut self, ui: &mut egui::Ui, day: Day, date: Date) {
        ui.columns_const(|[ui_1, ui_2, ui_3]| {
            ui_1.vertical_centered(|ui_1| {
                ui_1.label(create_label("Day").underline());
                ui_1.label(create_label(&day.return_day_name()));
            });
            ui_2.vertical_centered(|ui_2| {
                ui_2.label(create_label("Date").underline());
                ui_2.label(create_label(&date.print_date()));
            });
            ui_3.vertical_centered(|ui_3| {
                ui_3.label(create_label("Takings").underline());
            });
        });
        ui.add_sized(
            [400.0, 25.0],
            egui::Button::new(create_label("Confirm Takings")),
        );
    }
}
