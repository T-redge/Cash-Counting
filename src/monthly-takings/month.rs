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
}
