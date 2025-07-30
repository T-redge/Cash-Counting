use eframe::{
    CreationContext,
    egui::{self, FontId, Key, RichText, TextEdit, Ui},
};
pub mod numpad;
use numpad::*;

#[unsafe(no_mangle)]
fn android_main(app: winit::platform::android::activity::AndroidApp) {
    let options = eframe::NativeOptions {
        android_app: Some(app),
        ..Default::default()
    };
    eframe::run_native(
        "BankCalc",
        options,
        Box::new(|cc| Ok(Box::new(CashCalculator::new(cc)))),
    )
    .unwrap()
}

pub struct CashCalculator {
    note_100: String,
    num_of_100: u32,
    n100_sel: bool,
    note_50: String,
    num_of_50: u32,
    n50_sel: bool,
    note_20: String,
    num_of_20: u32,
    n20_sel: bool,
    note_10: String,
    num_of_10: u32,
    n10_sel: bool,
    note_5: String,
    num_of_5: u32,
    n5_sel: bool,
    coin_2: String,
    num_of_2: u32,
    c2_sel: bool,
    coin_1: String,
    num_of_1: u32,
    c1_sel: bool,
    coin_50: String,
    num_of_c50: f32,
    c50_sel: bool,
    coin_20: String,
    num_of_c20: f32,
    c20_sel: bool,
    coin_10: String,
    num_of_c10: f32,
    c10_sel: bool,
    coin_5: String,
    num_of_c5: f32,
    c5_sel: bool,

    numpad: Keypad,
}

impl CashCalculator {
    pub fn new(_cc: &CreationContext<'_>) -> Self {
        Self {
            note_100: "0".to_string(),
            num_of_100: 0,
            n100_sel: false,
            note_50: "0".to_owned(),
            num_of_50: 0,
            n50_sel: false,
            note_20: "0".to_owned(),
            num_of_20: 0,
            n20_sel: false,
            note_10: "0".to_owned(),
            num_of_10: 0,
            n10_sel: false,
            note_5: "0".to_owned(),
            num_of_5: 0,
            n5_sel: false,
            coin_2: "0".to_owned(),
            num_of_2: 0,
            c2_sel: false,
            coin_1: "0".to_owned(),
            num_of_1: 0,
            c1_sel: false,
            coin_50: "0".to_owned(),
            num_of_c50: 0.0,
            c50_sel: false,
            coin_20: "0".to_owned(),
            num_of_c20: 0.0,
            c20_sel: false,
            coin_10: "0".to_owned(),
            num_of_c10: 0.0,
            c10_sel: false,
            coin_5: "0".to_owned(),
            num_of_c5: 0.0,
            c5_sel: false,

            numpad: Keypad::new(),
        }
    }
}

impl eframe::App for CashCalculator {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("status bar space").show(ctx, |ui| {
            ui.set_height(32.0);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.columns_const(|[ui_1, ui_2, ui_3]| {
                    create_label(ui_1, "Value");
                    create_label(ui_2, "Amount");
                    create_label(ui_3, "Total");
                });
                ui.end_row();
                text_line_note_100(ui, self);
                ui.end_row();
                text_line_note_50(ui, self);
                ui.end_row();
                text_line_note_20(ui, self);
                ui.end_row();
                text_line_note_10(ui, self);
                ui.end_row();
                text_line_note_5(ui, self);
                ui.end_row();
                text_line_coin_2(ui, self);
                ui.end_row();
                text_line_coin_1(ui, self);
                ui.end_row();
                text_line_coin_50(ui, self);
                ui.end_row();
                text_line_coin_20(ui, self);
                ui.end_row();
                text_line_coin_10(ui, self);
                ui.end_row();
                text_line_coin_5(ui, self);
                ui.end_row();
                get_notes_totals(ui, self);
                ui.end_row();
                get_coins_totals(ui, self);
                ui.end_row();
                get_final_totals(ui, self);
                ui.end_row();
                let size_x = ctx.screen_rect().width();
                let size_y = ctx.screen_rect().height();
                ui.label("x: ".to_owned() + &(size_x.to_string()));
                ui.label("y: ".to_owned() + &(size_y.to_string()));
                self.numpad.show(ctx);
            });
        });
    }
    fn raw_input_hook(&mut self, ctx: &egui::Context, raw_input: &mut egui::RawInput) {
        self.numpad.bump_events(ctx, raw_input);
    }
}

fn create_label(ui: &mut Ui, denom: &str) {
    ui.label(
        RichText::new(denom.to_string())
            .monospace()
            .size(20.0)
            .color(egui::Color32::WHITE),
    );
}
fn create_total_label(ui: &mut Ui, num: u32, denom: u32) {
    ui.label(
        RichText::new(" = $".to_string() + &(num * denom).to_string())
            .monospace()
            .size(20.0)
            .color(egui::Color32::WHITE),
    );
}
fn create_cents_total_label(ui: &mut Ui, num: f32, denom: f32) {
    let total = num * denom;
    ui.label(
        RichText::new(format!(" = ${:.2}", total))
            .monospace()
            .size(20.0)
            .color(egui::Color32::WHITE),
    );
}

fn text_line_note_100(ui: &mut Ui, cash: &mut CashCalculator) {
    ui.columns_const(|[ui_1, ui_2, ui_3]| {
        create_label(ui_1, "$100");
        let font_id = FontId::proportional(20.0);
        let note_hundred = ui_2.add_sized(
            [75.0, 20.0],
            TextEdit::singleline(&mut cash.note_100)
                .text_color(egui::Color32::WHITE)
                .font(font_id),
        );
        if note_hundred.gained_focus() {}
        if note_hundred.lost_focus() {
            cash.n100_sel = true;
        }

        if cash.n100_sel {
            let cash_total = if cash.note_100.trim().parse::<u32>().is_err() {
                0
            } else {
                cash.note_100.trim().parse::<u32>().unwrap()
            };

            cash.num_of_100 = cash_total;
            cash.n100_sel = false;
        }
        create_total_label(ui_3, cash.num_of_100, 100);
    });
}
fn text_line_note_50(ui: &mut Ui, cash: &mut CashCalculator) {
    ui.columns_const(|[ui_1, ui_2, ui_3]| {
        create_label(ui_1, "$50  ");
        let font_id = FontId::proportional(20.0);
        let note_fifty = ui_2.add_sized(
            [75.0, 20.0],
            TextEdit::singleline(&mut cash.note_50)
                .text_color(egui::Color32::WHITE)
                .font(font_id),
        );
        if note_fifty.clicked() {
            note_fifty.request_focus();
        }
        if note_fifty.lost_focus() {
            cash.n50_sel = true;
        }
        if cash.n50_sel {
            let cash_total = if cash.note_50.trim().parse::<u32>().is_err() {
                0
            } else {
                cash.note_50.trim().parse::<u32>().unwrap()
            };
            cash.num_of_50 = cash_total;
            cash.n50_sel = false;
        }
        create_total_label(ui_3, cash.num_of_50, 50);
    });
}
fn text_line_note_20(ui: &mut Ui, cash: &mut CashCalculator) {
    ui.columns_const(|[ui_1, ui_2, ui_3]| {
        create_label(ui_1, "$20");
        let font_id = FontId::proportional(20.0);
        let note_twenty = ui_2.add_sized(
            [75.0, 20.0],
            TextEdit::singleline(&mut cash.note_20)
                .text_color(egui::Color32::WHITE)
                .font(font_id),
        );
        if note_twenty.clicked() {
            note_twenty.request_focus();
        }
        if note_twenty.lost_focus() {
            cash.n20_sel = true;
        }
        if cash.n20_sel {
            let cash_total = if cash.note_20.trim().parse::<u32>().is_err() {
                0
            } else {
                cash.note_20.trim().parse::<u32>().unwrap()
            };
            cash.num_of_20 = cash_total;
            cash.n20_sel = false;
        }
        create_total_label(ui_3, cash.num_of_20, 20);
    });
}
fn text_line_note_10(ui: &mut Ui, cash: &mut CashCalculator) {
    ui.columns_const(|[ui_1, ui_2, ui_3]| {
        create_label(ui_1, "$10");
        let font_id = FontId::proportional(20.0);
        let note_ten = ui_2.add_sized(
            [75.0, 20.0],
            TextEdit::singleline(&mut cash.note_10)
                .text_color(egui::Color32::WHITE)
                .font(font_id),
        );
        if note_ten.clicked() {
            note_ten.request_focus();
        }
        if note_ten.lost_focus() {
            cash.n10_sel = true;
        }
        if cash.n10_sel {
            let cash_total = if cash.note_10.trim().parse::<u32>().is_err() {
                0
            } else {
                cash.note_10.trim().parse::<u32>().unwrap()
            };
            cash.num_of_10 = cash_total;
            cash.n10_sel = false;
        }
        create_total_label(ui_3, cash.num_of_10, 10);
    });
}
fn text_line_note_5(ui: &mut Ui, cash: &mut CashCalculator) {
    ui.columns_const(|[ui_1, ui_2, ui_3]| {
        create_label(ui_1, "$5");
        let font_id = FontId::proportional(20.0);
        let note_five = ui_2.add_sized(
            [75.0, 20.0],
            TextEdit::singleline(&mut cash.note_5)
                .text_color(egui::Color32::WHITE)
                .font(font_id),
        );
        if note_five.clicked() {
            note_five.request_focus();
        }
        if note_five.lost_focus() {
            cash.n5_sel = true;
        }
        if cash.n5_sel {
            let cash_total = if cash.note_5.trim().parse::<u32>().is_err() {
                0
            } else {
                cash.note_5.trim().parse::<u32>().unwrap()
            };

            cash.num_of_5 = cash_total;
            cash.n5_sel = false;
        }
        create_total_label(ui_3, cash.num_of_5, 5);
    });
}
fn text_line_coin_2(ui: &mut Ui, cash: &mut CashCalculator) {
    ui.columns_const(|[ui_1, ui_2, ui_3]| {
        create_label(ui_1, "$2");
        let font_id = FontId::proportional(20.0);
        let coin_two = ui_2.add_sized(
            [75.0, 20.0],
            TextEdit::singleline(&mut cash.coin_2)
                .text_color(egui::Color32::WHITE)
                .font(font_id),
        );
        if coin_two.clicked() {
            coin_two.request_focus();
        }
        if coin_two.lost_focus() {
            cash.c2_sel = true;
        }
        if cash.c2_sel {
            let cash_total = if cash.coin_2.trim().parse::<u32>().is_err() {
                0
            } else {
                cash.coin_2.trim().parse::<u32>().unwrap()
            };
            cash.num_of_2 = cash_total;
            cash.c2_sel = false;
        }
        create_total_label(ui_3, cash.num_of_2, 2);
    });
}
fn text_line_coin_1(ui: &mut Ui, cash: &mut CashCalculator) {
    ui.columns_const(|[ui_1, ui_2, ui_3]| {
        create_label(ui_1, "$1");
        let font_id = FontId::proportional(20.0);
        let coin_one = ui_2.add_sized(
            [75.0, 20.0],
            TextEdit::singleline(&mut cash.coin_1)
                .text_color(egui::Color32::WHITE)
                .font(font_id),
        );
        if coin_one.clicked() {
            coin_one.request_focus();
        }
        if coin_one.lost_focus() {
            cash.c1_sel = true;
        }
        if cash.c1_sel {
            let cash_total = if cash.coin_1.trim().parse::<u32>().is_err() {
                0
            } else {
                cash.coin_1.trim().parse::<u32>().unwrap()
            };

            cash.num_of_1 = cash_total;
            cash.c1_sel = false;
        }
        create_total_label(ui_3, cash.num_of_1, 1);
    });
}
fn text_line_coin_50(ui: &mut Ui, cash: &mut CashCalculator) {
    ui.columns_const(|[ui_1, ui_2, ui_3]| {
        create_label(ui_1, "$0.50");
        let font_id = FontId::proportional(20.0);
        let fifty_cent = ui_2.add_sized(
            [75.0, 20.0],
            TextEdit::singleline(&mut cash.coin_50)
                .text_color(egui::Color32::WHITE)
                .font(font_id),
        );
        if fifty_cent.clicked() {
            fifty_cent.request_focus();
        }
        if fifty_cent.lost_focus() {
            cash.c50_sel = true;
        }

        if cash.c50_sel {
            let cash_total = if cash.coin_50.trim().parse::<f32>().is_err() {
                0.0
            } else {
                cash.coin_50.trim().parse::<f32>().unwrap()
            };
            cash.num_of_c50 = cash_total;
            cash.c50_sel = false;
        }
        create_cents_total_label(ui_3, cash.num_of_c50, 0.50);
    });
}
fn text_line_coin_20(ui: &mut Ui, cash: &mut CashCalculator) {
    ui.columns_const(|[ui_1, ui_2, ui_3]| {
        create_label(ui_1, "$0.20");
        let font_id = FontId::proportional(20.0);
        let twenty_cent = ui_2.add_sized(
            [75.0, 20.0],
            TextEdit::singleline(&mut cash.coin_20)
                .text_color(egui::Color32::WHITE)
                .font(font_id),
        );
        if twenty_cent.clicked() {
            twenty_cent.request_focus();
        }
        if twenty_cent.lost_focus() {
            cash.c20_sel = true;
        }
        if cash.c20_sel {
            let cash_total = if cash.coin_20.trim().parse::<f32>().is_err() {
                0.0
            } else {
                cash.coin_20.trim().parse::<f32>().unwrap()
            };
            cash.num_of_c20 = cash_total;
            cash.c20_sel = false;
        }
        create_cents_total_label(ui_3, cash.num_of_c20, 0.20);
    });
}
fn text_line_coin_10(ui: &mut Ui, cash: &mut CashCalculator) {
    ui.columns_const(|[ui_1, ui_2, ui_3]| {
        create_label(ui_1, "$0.10");
        let font_id = FontId::proportional(20.0);
        let ten_cent = ui_2.add_sized(
            [75.0, 20.0],
            TextEdit::singleline(&mut cash.coin_10)
                .text_color(egui::Color32::WHITE)
                .font(font_id),
        );
        if ten_cent.clicked() {
            ten_cent.request_focus();
        }
        if ten_cent.lost_focus() {
            cash.c10_sel = true;
        }

        if cash.c10_sel {
            let cash_total = if cash.coin_10.trim().parse::<f32>().is_err() {
                0.0
            } else {
                cash.coin_10.trim().parse::<f32>().unwrap()
            };
            cash.num_of_c10 = cash_total;
            cash.c10_sel = false;
        }
        create_cents_total_label(ui_3, cash.num_of_c10, 0.10);
    });
}
fn text_line_coin_5(ui: &mut Ui, cash: &mut CashCalculator) {
    ui.columns_const(|[ui_1, ui_2, ui_3]| {
        create_label(ui_1, "$0.05");
        let font_id = FontId::proportional(20.0);
        let five_cent = ui_2.add_sized(
            [75.0, 20.0],
            TextEdit::singleline(&mut cash.coin_5)
                .text_color(egui::Color32::WHITE)
                .font(font_id),
        );
        if five_cent.clicked() {
            five_cent.request_focus();
        }
        if five_cent.lost_focus() {
            cash.c5_sel = true;
        }
        if cash.c5_sel {
            let cash_total = if cash.coin_5.trim().parse::<f32>().is_err() {
                0.0
            } else {
                cash.coin_5.trim().parse::<f32>().unwrap()
            };
            cash.num_of_c5 = cash_total;
            cash.c5_sel = false;
        }
        create_cents_total_label(ui_3, cash.num_of_c5, 0.05);
    });
}

fn get_notes_totals(ui: &mut Ui, cash: &mut CashCalculator) {
    let num_notes = add_note_num(cash);
    let note_value = add_note_value(cash);
    let mut note_label = String::from("$");
    note_label.push_str(&note_value.to_string());
    ui.columns_const(|[ui_1, ui_2, ui_3]| {
        create_label(ui_1, "Notes");
        create_label(ui_2, &num_notes.to_string());
        create_label(ui_3, &note_label);
    });
}
fn add_note_num(cash: &mut CashCalculator) -> u32 {
    let notes_value =
        cash.num_of_100 + cash.num_of_50 + cash.num_of_20 + cash.num_of_10 + cash.num_of_5;
    notes_value
}
fn add_note_value(cash: &mut CashCalculator) -> u32 {
    let hundred = cash.num_of_100 * 100;
    let fifty = cash.num_of_50 * 50;
    let twenty = cash.num_of_20 * 20;
    let ten = cash.num_of_10 * 10;
    let five = cash.num_of_5 * 5;

    let total = hundred + fifty + twenty + ten + five;
    total
}
fn add_num_coins(cash: &mut CashCalculator) -> f32 {
    let gold = cash.num_of_2 + cash.num_of_1;
    let silver = cash.num_of_c50 + cash.num_of_c20 + cash.num_of_c10 + cash.num_of_c5;
    let total = gold as f32 + silver;
    total
}
fn add_coin_value(cash: &mut CashCalculator) -> f32 {
    let gold = (cash.num_of_2 * 2) + (cash.num_of_1 * 1);
    let silver = (cash.num_of_c50 * 0.50)
        + (cash.num_of_c20 * 0.20)
        + (cash.num_of_c10 * 0.10)
        + (cash.num_of_c5 * 0.05);
    let value = gold as f32 + silver;
    value
}
fn get_coins_totals(ui: &mut Ui, cash: &mut CashCalculator) {
    let num_coins = String::from(add_num_coins(cash).to_string());
    let mut value_coins = String::from("$");
    value_coins.push_str(&format!("{:.2}", add_coin_value(cash)));
    ui.columns_const(|[ui_1, ui_2, ui_3]| {
        create_label(ui_1, "Coins");
        create_label(ui_2, &num_coins);
        create_label(ui_3, &value_coins);
    });
}
fn get_final_totals(ui: &mut Ui, cash: &mut CashCalculator) {
    let total_notes = add_note_num(cash) as f32;
    let value_notes = add_note_value(cash) as f32;

    let total_coins = add_num_coins(cash);
    let value_coins = add_coin_value(cash);

    let total_number = total_notes + total_coins;
    let total_value = format!("{:.2}", value_notes + value_coins);
    let mut value_label = String::from("$");
    value_label.push_str(&total_value.to_string());

    ui.columns_const(|[ui_1, ui_2, ui_3]| {
        create_label(ui_1, "Total");
        create_label(ui_2, &total_number.to_string());
        create_label(ui_3, &value_label);
    });
}
