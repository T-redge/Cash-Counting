pub mod day;
pub mod month;
pub mod week;

use eframe::egui;

use {day::*, month::*, week::*};

pub struct MonthlyTakings {
    day: DayCalc,
    week: WeekCalc,
    month: MonthCalc,
    year: u16,
    date_selected: bool,
}
impl MonthlyTakings {
    pub fn new() -> Self {
        Self {
            day: DayCalc::new(),
            week: WeekCalc::new(),
            month: MonthCalc::new(),
            year: 2025,
            date_selected: false,
        }
    }
    pub fn show(&mut self, ui: &mut egui::Ui) {
        if !self.date_selected {
            self.get_date(ui);
        } else {
            self.show_sheet_header(ui);
            self.week.weekly_totals(ui, Week::One);
            ui.label(self.get_day_label());
        }
    }
    fn show_sheet_header(&mut self, ui: &mut egui::Ui) {
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
    fn get_date(&mut self, ui: &mut egui::Ui) {
        ui.columns_const(|[day, month, year]| {
            day.vertical_centered(|day| {
                day.label(create_label("Day"));
                self.day.select_day(day);
            });
            month.vertical_centered(|month| {
                month.label(create_label("Month"));
                self.month.select_month(month);
            });
            year.vertical_centered(|year| {
                year.label(create_label("Year"));
                year.label(create_label(&self.year.to_string()));
            })
        });
        ui.vertical_centered_justified(|ui| {
            let select_date_btn = ui.add(
                egui::Button::new(create_label("Confirm Date"))
                    .min_size(egui::Vec2 { x: 0.0, y: 25.0 }),
            );
            if select_date_btn.clicked() {
                self.date_selection();
            }
        });
    }
    fn get_day_label(&self) -> String {
        let date_day = self.day.return_day_date().to_string();
        let date_month = self.month.return_month().return_month_num().to_string();
        let date_year = self.year.to_string();

        let date_full = String::from(date_day + "/" + &date_month + "/" + &date_year);
        date_full
    }
    fn date_selection(&mut self) {
        self.date_selected = true;
    }
}

pub fn create_label(text: &str) -> egui::RichText {
    let txt = egui::RichText::new(text)
        .color(egui::Color32::WHITE)
        .monospace();
    txt
}
