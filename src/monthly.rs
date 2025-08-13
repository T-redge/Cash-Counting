use crate::{CCursor, CCursorRange};
use eframe::egui::{self, Button, Color32, Ui};
#[derive(PartialEq)]
enum Day {
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
    fn return_day(&self) -> String {
        let day = match self {
            Self::None => "Error",
            Self::Monday => "Monday",
            Self::Tuesday => "Tuesday",
            Self::Wednesday => "Wednesday",
            Self::Thursday => "Thursday",
            Self::Friday => "Friday",
            Self::Saturday => "Saturday",
            Self::Sunday => "Sunday",
        };
        day.to_string()
    }
}

pub struct MonthlyCalc {
    day_text: String,
    day: Day,
    date: u8,
    takings: String,
    takings_highlighted: bool,
    data_store: Vec<(String, u8, String)>,
    week_one: bool,
    week_two: bool,
    week_three: bool,
    weekly_store: Vec<f32>,
}
impl MonthlyCalc {
    pub fn new() -> Self {
        Self {
            day_text: String::new(),
            day: Day::None,
            date: 1,
            takings: "0".to_owned(),
            takings_highlighted: false,
            data_store: vec![],
            week_one: false,
            week_two: false,
            week_three: false,
            weekly_store: vec![0.0; 5],
        }
    }
    pub fn show(&mut self, ui: &mut Ui) {
        ui.columns_const(|[ui_1, ui_2, ui_3]| {
            ui_1.label("Day");
            if self.date == 1 {
                self.select_day(ui_1);
            } else {
                self.show_day(ui_1);
            }
            ui_2.label("Date");
            self.show_date(ui_2);
            ui_3.label("Takings");
            self.enter_takings(ui_3);
        });
        ui.vertical_centered(|ui| {
            let finalise = ui.add_sized([400.0, 25.0], Button::new("Enter Data"));
            if finalise.clicked() {
                self.save_data();
            }
        });
        self.monthly_takings_list(ui);
    }
    fn select_day(&mut self, ui: &mut Ui) {
        egui::ComboBox::from_id_salt("DAY")
            .selected_text(format!("{}", self.day_text))
            .show_ui(ui, |ui| {
                if ui
                    .selectable_value(
                        &mut self.day_text,
                        Day::Monday.return_day(),
                        Day::Monday.return_day(),
                    )
                    .clicked()
                {
                    self.day = Day::Monday;
                };
                if ui
                    .selectable_value(
                        &mut self.day_text,
                        Day::Tuesday.return_day(),
                        Day::Tuesday.return_day(),
                    )
                    .clicked()
                {
                    self.day = Day::Tuesday;
                };
                if ui
                    .selectable_value(
                        &mut self.day_text,
                        Day::Wednesday.return_day(),
                        Day::Wednesday.return_day(),
                    )
                    .clicked()
                {
                    self.day = Day::Wednesday;
                };
                if ui
                    .selectable_value(
                        &mut self.day_text,
                        Day::Thursday.return_day(),
                        Day::Thursday.return_day(),
                    )
                    .clicked()
                {
                    self.day = Day::Thursday;
                };
                if ui
                    .selectable_value(
                        &mut self.day_text,
                        Day::Friday.return_day(),
                        Day::Friday.return_day(),
                    )
                    .clicked()
                {
                    self.day = Day::Friday;
                };
                if ui
                    .selectable_value(
                        &mut self.day_text,
                        Day::Saturday.return_day(),
                        Day::Saturday.return_day(),
                    )
                    .clicked()
                {
                    self.day = Day::Saturday;
                };
                if ui
                    .selectable_value(
                        &mut self.day_text,
                        Day::Sunday.return_day(),
                        Day::Sunday.return_day(),
                    )
                    .clicked()
                {
                    self.day = Day::Sunday
                };
            });
    }
    fn show_day(&self, ui: &mut Ui) {
        ui.label(&self.day_text);
    }
    fn show_date(&mut self, ui: &mut Ui) {
        ui.label(&self.date.to_string());
    }
    fn enter_takings(&mut self, ui: &mut Ui) {
        let mut money = egui::TextEdit::singleline(&mut self.takings)
            .text_color(Color32::WHITE)
            .show(ui);
        if !self.takings_highlighted {
            money.state.cursor.set_char_range(Some(CCursorRange::two(
                CCursor::new(0),
                CCursor::new(self.takings.len()),
            )));
            money.state.store(ui.ctx(), money.response.id);
            self.takings_highlighted = true;
        }
        if money.response.gained_focus() {
            self.takings_highlighted = false;
        }
    }
    fn save_data(&mut self) {
        let data = (self.day_text.clone(), self.date, self.takings.clone());
        self.data_store.push(data);
        self.next_day();
        if self.week_one && self.day == Day::Monday {
            self.week_two = true;
        }
        if self.day == Day::Monday {
            self.week_one = true;
        }
    }
    fn next_day(&mut self) {
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
        self.day = tomorrow;
        self.day_text = self.day.return_day();
        self.date += 1;
        self.takings = "0".to_string();
    }
    fn _reset_fields(&mut self) {
        self.day_text = String::new();
        self.date = 1;
        self.takings = "0".to_string();
    }
    fn monthly_takings_list(&mut self, ui: &mut Ui) {
        self.week_list(ui, 0);
        if self.week_one {
            self.week_list(ui, 1);
            if self.week_two {
                self.week_list(ui, 2);
                if self.week_three {
                    self.week_list(ui, 3);
                }
            }
        }
    }
    fn week_list(&mut self, ui: &mut Ui, week_num: usize) {
        if self.data_store.len() > 0 {
            ui.columns_const(|[ui_1, ui_2, ui_3]| {
                for day in 0..(self.data_store.len()) {
                    ui_1.label(&self.data_store[day].0);
                    ui_2.label(&self.data_store[day].1.to_string());
                    ui_3.label(&self.data_store[day].2);
                }
                ui_2.label("Week Total ");
                if self.week_one {
                    ui_3.label(format!("{:.2}", self.weekly_store[week_num]));
                } else {
                    self.weekly_total(ui_3, week_num);
                }
            });
        }
    }
    fn weekly_total(&mut self, ui: &mut egui::Ui, week_num: usize) {
        let mut weekly = 0.0;
        for x in &self.data_store {
            weekly += &x.2.parse::<f32>().unwrap();
        }
        self.weekly_store[week_num] = weekly;
        ui.label(format!("{:.2}", self.weekly_store[week_num]));
    }
}
