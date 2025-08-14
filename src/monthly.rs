pub mod day;
pub mod month;
pub mod week;
use crate::{CCursor, CCursorRange};
use eframe::egui::{self, Button, Color32, RichText, Ui};
use std::collections::BTreeMap;
use {
    day::{Day, *},
    month::{Month, *},
};

pub struct MonthlyCalc {
    day: DayCalc,
    month: MonthCalc,
    year: u16,
    year_string: String,
    year_highlighted: bool,
    date: u8,
    date_select: bool,
    date_full: String,
    takings: String,
    takings_highlighted: bool,
    daily_data: BTreeMap<String, (Day, f32)>,
    week_totals: f32,
    weekly_totals: BTreeMap<String, f32>,
    start_week: String,
    end_week: String,
    week_dates: Vec<String>,
    week_number: usize,
    w_one: bool,
    w_two: bool,
    w_three: bool,
    w_four: bool,
}
impl MonthlyCalc {
    pub fn new() -> Self {
        Self {
            day: DayCalc::new(),
            month: MonthCalc::new(),
            year: 0,
            year_string: String::from("0"),
            year_highlighted: false,
            date: 1,
            date_select: false,
            date_full: String::new(),
            takings: "0".to_owned(),
            takings_highlighted: false,
            daily_data: BTreeMap::new(),
            week_totals: 0.0,
            weekly_totals: BTreeMap::new(),
            start_week: String::new(),
            end_week: String::new(),
            week_dates: Vec::new(),
            week_number: 0,
            w_one: false,
            w_two: false,
            w_three: false,
            w_four: false,
        }
    }
    pub fn show(&mut self, ui: &mut Ui) {
        if self.date == 1 {
            if !self.date_select {
                self.select_date(ui);
            }
        }
        if self.date_select {
            self.show_data_entry(ui);
        }
        if self.date > 1 {
            self.show_montly_record(ui);
        }
    }
    fn select_date(&mut self, ui: &mut egui::Ui) {
        ui.columns_const(|[ui_1, ui_2, ui_3]| {
            ui_1.vertical_centered(|ui_1| {
                ui_1.label(
                    RichText::new("Day")
                        .underline()
                        .color(Color32::WHITE)
                        .monospace(),
                );
                //   self.select_day(ui_1);
            });
            ui_2.vertical_centered(|ui_2| {
                ui_2.label(
                    RichText::new("Month")
                        .underline()
                        .color(Color32::WHITE)
                        .monospace(),
                );
                self.select_month(ui_2);
            });
            ui_3.vertical_centered(|ui_3| {
                ui_3.label(
                    RichText::new("Year")
                        .underline()
                        .color(Color32::WHITE)
                        .monospace(),
                );
                self.select_year(ui_3);
            });
        });
        ui.vertical_centered_justified(|ui| {
            if ui
                .add(
                    Button::new(
                        RichText::new("Enter Date")
                            .color(Color32::WHITE)
                            .monospace(),
                    )
                    .min_size(egui::Vec2 { x: 0.0, y: 25.0 }),
                )
                .clicked()
            {
                self.date_select = true;
            }
        });
    }
    fn select_year(&mut self, ui: &mut Ui) {
        let mut year = egui::TextEdit::singleline(&mut self.year_string)
            .text_color(Color32::WHITE)
            .show(ui);
        if !self.year_highlighted {
            year.state.cursor.set_char_range(Some(CCursorRange::two(
                CCursor::new(0),
                CCursor::new(self.year_string.len()),
            )));
            year.state.store(ui.ctx(), year.response.id);
            self.year_highlighted = true;
        }
        if year.response.gained_focus() {
            self.year_highlighted = false;
        }
        self.year = self.year_string.parse::<u16>().unwrap();
    }
    fn select_month(&mut self, ui: &mut Ui) {
        egui::ComboBox::from_id_salt("ID_Month")
            // .selected_text(format!("{:?}", self.month))
            .show_ui(ui, |ui| {
                // ui.selectable_value(
                //    &mut self.month,
                //    Month::January,
                //    Month::January.return_month_name(),
                // );
            });
    }
    fn show_date(&mut self, ui: &mut Ui) {
        let date = (
            self.date.to_string(),
            //self.month.return_month_num().to_string(),
            self.year.to_string(),
        );
        if self.date < 10 {
            self.date_full =
                String::from("0".to_string() + &date.0 + "/" + &date.1 + "/" + &date.2);
        } else {
            self.date_full = String::from(date.0 + "/" + &date.1 + "/" + &date.2);
        }
        ui.label(
            RichText::new(&self.date_full)
                .color(Color32::WHITE)
                .monospace(),
        );
    }
    fn show_data_entry(&mut self, ui: &mut egui::Ui) {
        ui.columns_const(|[ui_1, ui_2, ui_3]| {
            ui_1.vertical_centered(|ui_1| {
                ui_1.label(
                    RichText::new("Day")
                        .underline()
                        .color(Color32::WHITE)
                        .monospace(),
                );
                self.show_day(ui_1);
            });
            ui_2.vertical_centered(|ui_2| {
                ui_2.label(
                    RichText::new("Date")
                        .underline()
                        .color(Color32::WHITE)
                        .monospace(),
                );
                self.show_date(ui_2);
            });
            ui_3.vertical_centered(|ui_3| {
                ui_3.label(
                    RichText::new("Takings")
                        .underline()
                        .color(Color32::WHITE)
                        .monospace(),
                );
                self.enter_takings(ui_3);
            });
        });
        ui.vertical_centered(|ui| {
            let finalise = ui.add_sized(
                [400.0, 25.0],
                Button::new(
                    RichText::new("Enter Data")
                        .color(Color32::WHITE)
                        .monospace(),
                ),
            );
            if finalise.clicked() {
                self.save_data();
            }
        });
    }
    fn next_day(&mut self) {
        //let tomorrow = match self.day {
        //    Day::None => Day::None,
        //    Day::Monday => {
        //        self.start_week = self.date_full.clone();
        //        Day::Tuesday
        //    }
        //    Day::Tuesday => Day::Wednesday,
        //    Day::Wednesday => Day::Thursday,
        //    Day::Thursday => Day::Friday,
        //    Day::Friday => Day::Saturday,
        //    Day::Saturday => Day::Sunday,
        //    Day::Sunday => {
        //        self.end_week = self.date_full.clone();
        //        let week_dates = String::from(self.start_week.clone() + "-" + &self.end_week);
        //        self.week_dates.push(week_dates.clone());
        //         self.weekly_totals
        //            .insert(self.week_dates[0].clone(), self.week_totals);
        //        self.week_totals = 0.0;
        //        Day::Monday
        //    }
        //};
        // self.day = tomorrow;
        // self.date += 1;
        // self.takings = "0".to_string();
    }
    fn save_data(&mut self) {
        let day_takings = (self.day, self.takings.parse::<f32>().unwrap());
        self.daily_data.insert(self.date_full.clone(), day_takings);
        self.takings_highlighted = false;
        self.get_weekly_total(&self.date_full.clone());
        self.next_day();
    }
    fn show_daily_record(&mut self, ui: &mut egui::Ui, day_key: &str) {
        let day_info = self.daily_data.get(day_key).unwrap();
        let day_day = RichText::new(day_info.0.return_day())
            .color(Color32::WHITE)
            .monospace();
        let day_date = RichText::new(day_key).color(Color32::WHITE).monospace();
        let day_takings = RichText::new(day_info.1.to_string())
            .color(Color32::WHITE)
            .monospace();

        ui.columns_const(|[ui_1, ui_2, ui_3]| {
            ui_1.vertical_centered(|ui_1| {
                ui_1.label(day_day);
            });
            ui_2.vertical_centered(|ui_2| {
                ui_2.label(day_date);
            });
            ui_3.vertical_centered(|ui_3| {
                ui_3.label(day_takings);
            });
        });
    }
    fn get_weekly_total(&mut self, daily_info: &str) {
        let total = self.daily_data.get(daily_info).unwrap();
        self.week_totals += total.1;
    }
    fn show_weekly_total(&mut self, ui: &mut egui::Ui, week_num: usize) {
        ui.columns_const(|[_ui_1, ui_2, ui_3]| {
            ui_2.vertical_centered(|ui_2| {
                ui_2.label(
                    RichText::new("Week totals")
                        .color(Color32::WHITE)
                        .monospace(),
                );
            });
            ui_3.vertical_centered(|ui| {
                if self.week_dates.len() > 0 {
                    let week = self.weekly_totals.contains_key(&self.week_dates[week_num]);
                    if week {
                        let w = self.weekly_totals.get(&self.week_dates[week_num]).unwrap();
                        ui.label(
                            RichText::new(w.to_string())
                                .color(Color32::WHITE)
                                .monospace(),
                        );
                    }
                }
            });
        });
    }
    fn weekly(&mut self, ui: &mut egui::Ui, key: usize) {
        let data = self.daily_data.clone();
        for daily_info in data.keys() {
            self.show_daily_record(ui, &daily_info);
            if *daily_info == self.end_week {
                self.show_weekly_total(ui, key);
                break;
            }
        }
    }
    fn show_montly_record(&mut self, ui: &mut egui::Ui) {
        ui.columns_const(|[ui_1, ui_2, ui_3]| {
            ui_1.vertical_centered(|ui_1| {
                ui_1.label(
                    RichText::new("Day")
                        .underline()
                        .color(Color32::WHITE)
                        .monospace(),
                );
            });
            ui_2.vertical_centered(|ui_2| {
                ui_2.label(
                    RichText::new("Date")
                        .underline()
                        .color(Color32::WHITE)
                        .monospace(),
                );
            });
            ui_3.vertical_centered(|ui_3| {
                ui_3.label(
                    RichText::new("Takings")
                        .underline()
                        .color(Color32::WHITE)
                        .monospace(),
                );
            });
        });
        if self.w_one {
            self.weekly(ui, 0);
        }
        if self.w_two {
            self.weekly(ui, 1);
        }
        if self.w_three {
            self.weekly(ui, 2);
        }
        if self.w_four {
            self.weekly(ui, 3);
        }
    }
}
