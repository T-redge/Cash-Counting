pub mod date_picker;
pub mod takings_input;
use date_picker::{Day, *};
use eframe::egui;

use crate::monthly_takings::takings_input::TakingsField;

#[derive(Debug, Clone)]
pub struct Date {
    d: u8,
    m: u8,
    y: u16,
}
impl Date {
    fn default() -> Self {
        Self { d: 0, m: 0, y: 0 }
    }
    fn new(d: u8, m: u8, y: u16) -> Self {
        Self { d, m, y }
    }
    pub fn print_date(&self) -> String {
        let day = self.d;
        let month = self.m;
        let year = self.y;

        let date =
            String::from(day.to_string() + "/" + &month.to_string() + "/" + &year.to_string());
        date
    }
}
#[derive(Debug, Clone)]
enum Data {
    Day(Day),
    Date(Date),
    Takings(f32),
}
impl Data {
    fn return_day(&self) -> Option<Day> {
        match &self {
            Data::Day(day) => Some(*day),
            _ => None,
        }
    }
    fn return_date(&self) -> Option<Date> {
        match self {
            Data::Date(date) => Some(date.clone()),
            _ => None,
        }
    }
    fn return_takings(&self) -> Option<f32> {
        match self {
            Data::Takings(takings) => Some(*takings),
            _ => None,
        }
    }
}

pub struct MonthlyTakings {
    day_date: u8,
    date_picker: DatePicker,
    full_date: Date,
    takings_field: TakingsField,
    daily_data: Vec<Data>,
    weekly_data: Vec<Vec<Data>>,
}
impl MonthlyTakings {
    pub fn new() -> Self {
        Self {
            day_date: 1,
            date_picker: DatePicker::new(),
            full_date: Date::default(),
            takings_field: TakingsField::new(),
            daily_data: vec![],
            weekly_data: init_weekly_data(),
        }
    }
    pub fn show(&mut self, ui: &mut egui::Ui) {
        if !self.date_picker.return_date_picked() {
            self.date_picker.show(ui);
        } else {
            self.full_date = Date::new(
                self.day_date,
                self.date_picker.return_month().return_month_num(),
                self.date_picker.return_year(),
            );
            self.takings_field
                .show(ui, self.date_picker.return_day(), self.full_date.clone());
            self.headers(ui);
        }
    }
    fn headers(&self, ui: &mut egui::Ui) {
        ui.columns_const(|[ui_1, ui_2, ui_3]| {
            ui_1.vertical_centered(|ui_1| {
                ui_1.label(create_label("Day").underline());
            });
            ui_2.vertical_centered(|ui_2| {
                ui_2.label(create_label("Date").underline());
            });
            ui_3.vertical_centered(|ui_3| {
                ui_3.label(create_label("Takings").underline());
            });
        });
    }
}
fn create_label(text: &str) -> egui::RichText {
    let txt = egui::RichText::new(text)
        .color(egui::Color32::WHITE)
        .monospace();
    txt
}
fn init_daily_data() -> Vec<Data> {
    let default_data: Vec<Data> = vec![
        Data::Day(Day::None),
        Data::Date(Date::default()),
        Data::Takings(0.0),
    ];
    default_data
}
fn init_weekly_data() -> Vec<Vec<Data>> {
    let default_daily = init_daily_data();
    let mut default_weekly: Vec<Vec<Data>> = vec![];
    default_weekly.push(default_daily);
    default_weekly
}
fn input_daily_data(day: Day, date: Date, takings: f32) -> Vec<Data> {
    let new_data = vec![Data::Day(day), Data::Date(date), Data::Takings(takings)];
    new_data
}
