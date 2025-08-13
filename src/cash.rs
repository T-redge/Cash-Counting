use crate::{CCursor, CCursorRange};
use eframe::egui::{Color32, FontId, RichText, TextEdit, Ui};

pub enum CashType {
    Note100,
    Note50,
    Note20,
    Note10,
    Note5,
    Coin2,
    Coin1,
    Coin50,
    Coin20,
    Coin10,
    Coin5,
}
impl CashType {
    fn cash_label(&self, ui: &mut Ui) {
        let text = match self {
            Self::Note100 => "$100",
            Self::Note50 => "$50",
            Self::Note20 => "$20",
            Self::Note10 => "$10",
            Self::Note5 => "$5",
            Self::Coin2 => "$2",
            Self::Coin1 => "$1",
            Self::Coin50 => "$0.50",
            Self::Coin20 => "$0.20",
            Self::Coin10 => "$0.10",
            Self::Coin5 => "$0.05",
        };
        ui.label(
            RichText::new(text)
                .monospace()
                .size(17.5)
                .color(Color32::WHITE),
        );
    }
    fn cash_number(&self) -> f32 {
        let num = match self {
            Self::Note100 => 100.0,
            Self::Note50 => 50.0,
            Self::Note20 => 20.0,
            Self::Note10 => 10.0,
            Self::Note5 => 5.0,
            Self::Coin2 => 2.0,
            Self::Coin1 => 1.0,
            Self::Coin50 => 0.50,
            Self::Coin20 => 0.20,
            Self::Coin10 => 0.10,
            Self::Coin5 => 0.05,
        };
        num
    }
}
pub struct CashWidget {
    pub cash_type: CashType,
    pub cash_text: String,
    pub cash_total_currency: f32,
    pub cash_focused: bool,
    pub cash_highlighted: bool,
}
impl CashWidget {
    pub fn new(cash_type: CashType) -> Self {
        Self {
            cash_type,
            cash_text: "0".to_owned(),
            cash_total_currency: 0.0,
            cash_focused: false,
            cash_highlighted: false,
        }
    }
    pub fn show(&mut self, ui: &mut Ui) {
        ui.columns_const(|[ui_1, ui_2, ui_3]| {
            self.cash_type.cash_label(ui_1);
            let font_id = FontId::proportional(17.5);
            let mut cash = TextEdit::singleline(&mut self.cash_text)
                .text_color(Color32::WHITE)
                .font(font_id)
                .show(ui_2);

            if !self.cash_highlighted {
                cash.state.cursor.set_char_range(Some(CCursorRange::two(
                    CCursor::new(0),
                    CCursor::new(self.cash_text.len()),
                )));
                cash.state.store(ui_2.ctx(), cash.response.id);
                self.cash_highlighted = true;
            }

            if cash.response.lost_focus() {
                self.cash_focused = true;
            }
            if cash.response.gained_focus() {
                self.cash_highlighted = false;
            }
            if self.cash_focused {
                self.cash_total_currency = self.get_total_currency_number();
                self.cash_focused = false;
            }

            self.create_total_dollar_label(ui_3);
        });
    }
    fn create_total_dollar_label(&mut self, ui: &mut Ui) {
        ui.label(
            RichText::new(
                " = $".to_string()
                    + &(format!("{:.2}", self.get_total_currency_dollar()).to_string()),
            )
            .monospace()
            .size(17.5)
            .color(Color32::WHITE),
        );
    }
    pub fn get_total_currency_dollar(&self) -> f32 {
        self.cash_total_currency * self.cash_type.cash_number()
    }
    fn get_total_currency_number(&self) -> f32 {
        let cash_total = if self.cash_text.trim().parse::<f32>().is_err() {
            0.0
        } else {
            self.cash_text.trim().parse::<f32>().unwrap()
        };
        cash_total
    }
    pub fn return_total_currency(&self) -> f32 {
        self.cash_total_currency
    }
}
