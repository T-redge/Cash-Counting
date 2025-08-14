use crate::{CCursor, CCursorRange};
use eframe::egui;
#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Day {
    None,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}
impl Day {
    pub fn return_day(&self) -> String {
        format!("{:?}", self)
    }
    fn return_label(&self, ui: &mut egui::Ui) {
        ui.label(
            egui::RichText::new(self.return_day())
                .color(egui::Color32::WHITE)
                .monospace(),
        );
    }
}

pub struct DayCalc {
    day: Day,
    day_date: u8,
    date_selected: bool,
    day_takings: String,
    highlighted: bool,
}
impl DayCalc {
    pub fn new() -> Self {
        Self {
            day: Day::None,
            day_date: 1,
            date_selected: false,
            day_takings: "0".to_string(),
            highlighted: false,
        }
    }
    pub fn select_day(&mut self, ui: &mut egui::Ui) {
        egui::ComboBox::from_id_salt("DAY")
            .selected_text(format!("{:?}", self.day))
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut self.day, Day::Monday, Day::Monday.return_day());
            });
    }
    pub fn enter_takings(&mut self, ui: &mut egui::Ui) {
        let mut money = egui::TextEdit::singleline(&mut self.day_takings)
            .text_color(egui::Color32::WHITE)
            .show(ui);
        if !self.highlighted {
            money.state.cursor.set_char_range(Some(CCursorRange::two(
                CCursor::new(0),
                CCursor::new(self.day_takings.len()),
            )));
            money.state.store(ui.ctx(), money.response.id);
            self.change_day_selected();
        }
        if money.response.gained_focus() {
            self.change_day_selected();
        }
    }
    pub fn return_day_date(&self) -> u8 {
        self.day_date
    }
    pub fn return_day_takings(&self) -> f32 {
        let dollar = self.day_takings.parse::<f32>().unwrap();
        dollar
    }
    pub fn increment_day_date(&mut self) {
        self.day_date += 1;
    }
    pub fn reset_day_takings(&mut self) {
        self.day_takings = "0".to_string();
    }
    pub fn change_day_selected(&mut self) {
        match self.date_selected {
            true => self.date_selected = false,
            false => self.date_selected = true,
        }
    }
    pub fn increment_day(&mut self) {
        let tomorrow = match self.day {
            Day::None => Day::None,
            Day::Monday => Day::Tuesday,
            Day::Tuesday => Day::Wednesday,
            Day::Wednesday => Day::Thursday,
            Day::Thursday => Day::Friday,
            Day::Friday => Day::Saturday,
            Day::Saturday => Day::Sunday,
            Day::Sunday => Day::Monday,
        };
        self.increment_day_date();
        self.reset_day_takings();
        self.day = tomorrow;
    }
}
