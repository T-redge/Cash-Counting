use crate::cash::{CashType::*, *};
use eframe::egui::{self, RichText, Ui};
pub struct BankingCalc {
    note_100: CashWidget,
    note_50: CashWidget,
    note_20: CashWidget,
    note_10: CashWidget,
    note_5: CashWidget,
    coin_2: CashWidget,
    coin_1: CashWidget,
    coin_50: CashWidget,
    coin_20: CashWidget,
    coin_10: CashWidget,
    coin_5: CashWidget,
}

impl BankingCalc {
    pub fn new() -> Self {
        Self {
            note_100: CashWidget::new(Note100),
            note_50: CashWidget::new(Note50),
            note_20: CashWidget::new(Note20),
            note_10: CashWidget::new(Note10),
            note_5: CashWidget::new(Note5),
            coin_2: CashWidget::new(Coin2),
            coin_1: CashWidget::new(Coin1),
            coin_50: CashWidget::new(Coin50),
            coin_20: CashWidget::new(Coin20),
            coin_10: CashWidget::new(Coin10),
            coin_5: CashWidget::new(Coin5),
        }
    }
    fn add_coin_total_currency(&self) -> f32 {
        let two = self.coin_2.return_total_currency();
        let one = self.coin_1.return_total_currency();
        let fifty = self.coin_50.return_total_currency();
        let twenty = self.coin_20.return_total_currency();
        let ten = self.coin_10.return_total_currency();
        let five = self.coin_5.return_total_currency();

        two + one + fifty + twenty + ten + five
    }
    fn add_coin_total_dollar(&self) -> f32 {
        let two = self.coin_2.get_total_currency_dollar();
        let one = self.coin_1.get_total_currency_dollar();
        let fifty = self.coin_50.get_total_currency_dollar();
        let twenty = self.coin_20.get_total_currency_dollar();
        let ten = self.coin_10.get_total_currency_dollar();
        let five = self.coin_5.get_total_currency_dollar();

        two + one + fifty + twenty + ten + five
    }
    fn coin_total_currency_label(&self, ui: &mut Ui) {
        ui.label(
            RichText::new(format!("{:.0}", self.add_coin_total_currency()))
                .monospace()
                .size(17.5)
                .color(egui::Color32::WHITE),
        );
    }
    fn coin_total_dollar_label(&self, ui: &mut Ui) {
        ui.label(
            RichText::new(format!(" = ${:.2}", self.add_coin_total_dollar()))
                .monospace()
                .size(17.5)
                .color(egui::Color32::WHITE),
        );
    }
    fn get_coin_totals(&self, ui: &mut Ui) {
        ui.columns_const(|[ui_1, ui_2, ui_3]| {
            create_label(ui_1, "Coins");
            self.coin_total_currency_label(ui_2);
            self.coin_total_dollar_label(ui_3);
        });
    }

    fn add_note_total_currency(&self) -> f32 {
        let hundred = self.note_100.return_total_currency();
        let fifty = self.note_50.return_total_currency();
        let twenty = self.note_20.return_total_currency();
        let ten = self.note_10.return_total_currency();
        let five = self.note_5.return_total_currency();

        hundred + fifty + twenty + ten + five
    }
    fn add_note_total_dollar(&self) -> f32 {
        let hundred = self.note_100.get_total_currency_dollar();
        let fifty = self.note_50.get_total_currency_dollar();
        let twenty = self.note_20.get_total_currency_dollar();
        let ten = self.note_10.get_total_currency_dollar();
        let five = self.note_5.get_total_currency_dollar();

        hundred + fifty + twenty + ten + five
    }
    fn note_total_currency_label(&self, ui: &mut Ui) {
        ui.label(
            RichText::new(format!("{:.0}", self.add_note_total_currency()))
                .monospace()
                .size(17.5)
                .color(egui::Color32::WHITE),
        );
    }
    fn note_total_dollar_label(&self, ui: &mut Ui) {
        ui.label(
            RichText::new(format!(" = ${:.2}", self.add_note_total_dollar()))
                .monospace()
                .size(17.5)
                .color(egui::Color32::WHITE),
        );
    }
    fn get_note_totals(&self, ui: &mut Ui) {
        ui.columns_const(|[ui_1, ui_2, ui_3]| {
            create_label(ui_1, "Notes");
            self.note_total_currency_label(ui_2);
            self.note_total_dollar_label(ui_3);
        });
    }

    fn add_total_currency(&self) -> f32 {
        self.add_coin_total_currency() + self.add_note_total_currency()
    }
    fn add_total_dollar(&self) -> f32 {
        self.add_coin_total_dollar() + self.add_note_total_dollar()
    }
    fn total_currency_label(&self, ui: &mut Ui) {
        ui.label(
            RichText::new(format!("{:.0}", self.add_total_currency()))
                .monospace()
                .size(17.5)
                .color(egui::Color32::WHITE),
        );
    }
    fn total_dollar_label(&self, ui: &mut Ui) {
        ui.label(
            RichText::new(format!(" = ${:.2}", self.add_total_dollar()))
                .monospace()
                .size(17.5)
                .color(egui::Color32::WHITE),
        );
    }
    fn get_final_totals(&self, ui: &mut Ui) {
        ui.columns_const(|[ui_1, ui_2, ui_3]| {
            create_label(ui_1, "Total");
            self.total_currency_label(ui_2);
            self.total_dollar_label(ui_3);
        });
    }

    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.horizontal_wrapped(|ui| {
            ui.columns_const(|[ui_1, ui_2, ui_3]| {
                create_label(ui_1, "Value");
                create_label(ui_2, "Amount");
                create_label(ui_3, "Total");
            });
            ui.end_row();
            self.note_100.show(ui);
            ui.end_row();
            self.note_50.show(ui);
            ui.end_row();
            self.note_20.show(ui);
            ui.end_row();
            self.note_10.show(ui);
            ui.end_row();
            self.note_5.show(ui);
            ui.end_row();
            self.coin_2.show(ui);
            ui.end_row();
            self.coin_1.show(ui);
            ui.end_row();
            self.coin_50.show(ui);
            ui.end_row();
            self.coin_20.show(ui);
            ui.end_row();
            self.coin_10.show(ui);
            ui.end_row();
            self.coin_5.show(ui);
            ui.end_row();
            self.get_note_totals(ui);
            ui.end_row();
            self.get_coin_totals(ui);
            ui.end_row();
            self.get_final_totals(ui);
            ui.end_row();
        });
    }
}
fn create_label(ui: &mut Ui, text: &str) {
    ui.label(
        RichText::new(text.to_string())
            .monospace()
            .size(17.5)
            .color(egui::Color32::WHITE),
    );
}
