use eframe::egui;
#[derive(PartialEq, Clone, Copy, Debug)]
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
    pub fn return_month_num(&self) -> u8 {
        match self {
            Month::None => 0,
            Month::January => 1,
            Month::Febuary => 2,
            Month::March => 3,
            Month::April => 4,
            Month::May => 5,
            Month::June => 6,
            Month::July => 7,
            Month::August => 8,
            Month::September => 9,
            Month::October => 10,
            Month::November => 11,
            Month::December => 12,
        }
    }
    pub fn return_month_name(&self) -> String {
        format!("{:?}", self)
    }
}
pub struct MonthCalc {
    month: Month,
}
impl MonthCalc {
    pub fn new() -> Self {
        Self { month: Month::None }
    }
    pub fn select_month(&mut self, ui: &mut egui::Ui) {
        egui::ComboBox::from_id_salt("ID_Month")
            .selected_text(format!("{:?}", self.month))
            .show_ui(ui, |ui| {
                ui.selectable_value(
                    &mut self.month,
                    Month::January,
                    Month::January.return_month_name(),
                );
                ui.selectable_value(
                    &mut self.month,
                    Month::Febuary,
                    Month::Febuary.return_month_name(),
                );
                ui.selectable_value(
                    &mut self.month,
                    Month::March,
                    Month::March.return_month_name(),
                );
                ui.selectable_value(
                    &mut self.month,
                    Month::April,
                    Month::April.return_month_name(),
                );
                ui.selectable_value(&mut self.month, Month::May, Month::May.return_month_name());
                ui.selectable_value(
                    &mut self.month,
                    Month::June,
                    Month::June.return_month_name(),
                );
                ui.selectable_value(
                    &mut self.month,
                    Month::July,
                    Month::July.return_month_name(),
                );
                ui.selectable_value(
                    &mut self.month,
                    Month::August,
                    Month::August.return_month_name(),
                );
                ui.selectable_value(
                    &mut self.month,
                    Month::September,
                    Month::September.return_month_name(),
                );
                ui.selectable_value(
                    &mut self.month,
                    Month::October,
                    Month::October.return_month_name(),
                );
                ui.selectable_value(
                    &mut self.month,
                    Month::November,
                    Month::November.return_month_name(),
                );
                ui.selectable_value(
                    &mut self.month,
                    Month::December,
                    Month::December.return_month_name(),
                );
            });
    }
    pub fn return_month(&self) -> Month {
        self.month
    }
}
