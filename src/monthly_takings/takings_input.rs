use eframe::egui::{
    self,
    text::{CCursor, CCursorRange},
};

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
            text: "0".to_owned(),
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
                self.takings_textbox(ui_3);
            });
        });
        if ui
            .add_sized(
                [400.0, 25.0],
                egui::Button::new(create_label("Confirm Takings")),
            )
            .clicked()
        {
            self.takings = self.text.trim().parse::<f32>().unwrap();
            self.text = "0".to_owned();
        }
    }
    fn takings_textbox(&mut self, ui: &mut egui::Ui) {
        let mut text = egui::TextEdit::singleline(&mut self.text)
            .text_color(egui::Color32::WHITE)
            .background_color(egui::Color32::BLACK)
            .show(ui);
        if !self.text_highlighted {
            text.state.cursor.set_char_range(Some(CCursorRange::two(
                CCursor::new(0),
                CCursor::new(self.text.len()),
            )));
            text.state.store(ui.ctx(), text.response.id);
            self.text_highlighted = true;
        }

        if text.response.gained_focus() {
            self.text_highlighted = false;
        }
    }
}
