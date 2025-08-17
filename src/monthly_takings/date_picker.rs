use crate::monthly_takings::create_label;
use eframe::egui;
#[derive(Debug, Clone, Copy, PartialEq)]
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
    pub fn return_day_name(&self) -> String {
        format!("{:?}", self)
    }
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Month {
    None,
    January,
    Febuary,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December,
}
impl Month {
    fn return_month_name(&self) -> String {
        format!("{:?}", self)
    }
    pub fn return_month_num(&self) -> u8 {
        match self {
            Self::None => 0,
            Self::January => 1,
            Self::Febuary => 2,
            Self::March => 3,
            Self::April => 4,
            Self::May => 5,
            Self::June => 6,
            Self::July => 7,
            Self::August => 8,
            Self::September => 9,
            Self::October => 10,
            Self::November => 11,
            Self::December => 12,
        }
    }
}
pub struct DatePicker {
    day_name: Day,
    month_name: Month,
    month_date: u8,
    year_record: Vec<u16>,
    year_date: u16,

    date_picked: bool,
}
impl DatePicker {
    pub fn new() -> Self {
        Self {
            day_name: Day::None,
            month_name: Month::None,
            month_date: 0,
            year_record: vec![2025],
            year_date: 0,

            date_picked: false,
        }
    }
    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.columns_const(|[ui_1, ui_2, ui_3]| {
            ui_1.vertical_centered_justified(|ui_1| {
                ui_1.label(create_label("Day").underline());
                self.day_picker(ui_1);
            });
            ui_2.vertical_centered_justified(|ui_2| {
                ui_2.label(create_label("Month").underline());
                self.month_picker(ui_2);
            });
            ui_3.vertical_centered_justified(|ui_3| {
                ui_3.label(create_label("Year").underline());
                self.year_picker(ui_3);
            });
        });
        ui.vertical_centered_justified(|ui| {
            if ui
                .add_sized(
                    [390.0, 25.0],
                    egui::Button::new(create_label("Confirm Date").underline()),
                )
                .clicked()
            {
                self.month_date = self.month_name.return_month_num();
                self.date_picked = true;
            }
        });
    }
    fn day_picker(&mut self, ui: &mut egui::Ui) {
        egui::ComboBox::from_id_salt("ID_Day")
            .selected_text(format!("{:?}", self.day_name))
            .width(125.0)
            .show_ui(ui, |ui| {
                ui.selectable_value(
                    &mut self.day_name,
                    Day::Monday,
                    Day::Monday.return_day_name(),
                );
                ui.selectable_value(
                    &mut self.day_name,
                    Day::Tuesday,
                    Day::Tuesday.return_day_name(),
                );
                ui.selectable_value(
                    &mut self.day_name,
                    Day::Wednesday,
                    Day::Wednesday.return_day_name(),
                );
                ui.selectable_value(
                    &mut self.day_name,
                    Day::Thursday,
                    Day::Thursday.return_day_name(),
                );
                ui.selectable_value(
                    &mut self.day_name,
                    Day::Friday,
                    Day::Friday.return_day_name(),
                );
                ui.selectable_value(
                    &mut self.day_name,
                    Day::Saturday,
                    Day::Saturday.return_day_name(),
                );
                ui.selectable_value(
                    &mut self.day_name,
                    Day::Sunday,
                    Day::Sunday.return_day_name(),
                );
            });
    }
    fn month_picker(&mut self, ui: &mut egui::Ui) {
        egui::ComboBox::from_id_salt("ID_Month")
            .selected_text(format!("{:?}", self.month_name))
            .width(125.0)
            .show_ui(ui, |ui| {
                ui.selectable_value(
                    &mut self.month_name,
                    Month::January,
                    Month::January.return_month_name(),
                );
                ui.selectable_value(
                    &mut self.month_name,
                    Month::Febuary,
                    Month::Febuary.return_month_name(),
                );
                ui.selectable_value(
                    &mut self.month_name,
                    Month::March,
                    Month::March.return_month_name(),
                );
                ui.selectable_value(
                    &mut self.month_name,
                    Month::April,
                    Month::April.return_month_name(),
                );
                ui.selectable_value(
                    &mut self.month_name,
                    Month::May,
                    Month::May.return_month_name(),
                );
                ui.selectable_value(
                    &mut self.month_name,
                    Month::June,
                    Month::June.return_month_name(),
                );
                ui.selectable_value(
                    &mut self.month_name,
                    Month::July,
                    Month::July.return_month_name(),
                );
                ui.selectable_value(
                    &mut self.month_name,
                    Month::August,
                    Month::August.return_month_name(),
                );
                ui.selectable_value(
                    &mut self.month_name,
                    Month::September,
                    Month::September.return_month_name(),
                );
                ui.selectable_value(
                    &mut self.month_name,
                    Month::October,
                    Month::October.return_month_name(),
                );
                ui.selectable_value(
                    &mut self.month_name,
                    Month::November,
                    Month::November.return_month_name(),
                );
                ui.selectable_value(
                    &mut self.month_name,
                    Month::December,
                    Month::December.return_month_name(),
                );
            });
    }
    fn year_picker(&mut self, ui: &mut egui::Ui) {
        egui::ComboBox::from_id_salt("ID_Year")
            .width(125.0)
            .selected_text(self.year_date.to_string())
            .show_ui(ui, |ui| {
                for x in &self.year_record {
                    ui.selectable_value(&mut self.year_date, *x, x.to_string());
                }
            });
    }
    pub fn return_day(&self) -> Day {
        self.day_name
    }
    pub fn return_month(&self) -> Month {
        self.month_name
    }
    pub fn return_year(&self) -> u16 {
        self.year_date
    }
    pub fn return_date_picked(&self) -> bool {
        self.date_picked
    }
}
