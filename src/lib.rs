pub mod banking;
pub mod cash;
pub mod monthly;
pub mod numpad;
use {
    banking::*,
    eframe::{
        CreationContext,
        egui::{
            self, Button, Color32, Context, Margin,
            text::{CCursor, CCursorRange},
        },
    },
    monthly::*,
    numpad::*,
};
#[derive(PartialEq)]
enum Tab {
    None,
    Banking,
    Monthly,
    Donation,
}
pub struct CashCalculator {
    numpad: Keypad,
    banking_calc: BankingCalc,
    monthly_calc: MonthlyCalc,

    tab: Tab,
}

impl CashCalculator {
    pub fn new(_cc: &CreationContext<'_>) -> Self {
        Self {
            numpad: Keypad::new(),
            banking_calc: BankingCalc::new(),
            monthly_calc: MonthlyCalc::new(),

            tab: Tab::None,
        }
    }
    fn android_bar(&mut self, ctx: &Context) {
        egui::TopBottomPanel::top("status bar space").show(ctx, |ui| {
            ui.set_height(32.0);
        });
    }
    fn menu_tabs(&mut self, ctx: &Context) {
        egui::TopBottomPanel::top("ID_MENUTAB")
            .show_separator_line(false)
            .frame(egui::Frame::default().inner_margin(Margin::ZERO))
            .show(ctx, |ui| {
                ui.spacing_mut().item_spacing.x = 2.0;
                ui.columns_const(|[tab_1, tab_2, tab_3]| match self.tab {
                    Tab::Banking => {
                        if tab_1
                            .add_sized([100.0, 50.0], Button::new("Banking").fill(Color32::BLACK))
                            .clicked()
                        {
                            if self.tab == Tab::Banking {
                                self.tab = Tab::None;
                            } else {
                                self.tab = Tab::Banking;
                            }
                        }
                        if tab_2
                            .add_sized([100.0, 50.0], Button::new("Monthly"))
                            .clicked()
                        {
                            if self.tab == Tab::Monthly {
                                self.tab = Tab::None;
                            } else {
                                self.tab = Tab::Monthly;
                            }
                        }
                        if tab_3
                            .add_sized([100.0, 50.0], Button::new("Donations"))
                            .clicked()
                        {
                            if self.tab == Tab::Donation {
                                self.tab = Tab::None;
                            } else {
                                self.tab = Tab::Donation;
                            }
                        }
                    }
                    Tab::Monthly => {
                        if tab_1
                            .add_sized([100.0, 50.0], Button::new("Banking"))
                            .clicked()
                        {
                            if self.tab == Tab::Banking {
                                self.tab = Tab::None;
                            } else {
                                self.tab = Tab::Banking;
                            }
                        }
                        if tab_2
                            .add_sized([100.0, 50.0], Button::new("Monthly").fill(Color32::BLACK))
                            .clicked()
                        {
                            if self.tab == Tab::Monthly {
                                self.tab = Tab::None;
                            } else {
                                self.tab = Tab::Monthly;
                            }
                        }
                        if tab_3
                            .add_sized([100.0, 50.0], Button::new("Donations"))
                            .clicked()
                        {
                            if self.tab == Tab::Donation {
                                self.tab = Tab::None;
                            } else {
                                self.tab = Tab::Donation;
                            }
                        }
                    }
                    Tab::Donation => {
                        if tab_1
                            .add_sized([100.0, 50.0], Button::new("Banking"))
                            .clicked()
                        {
                            if self.tab == Tab::Banking {
                                self.tab = Tab::None;
                            } else {
                                self.tab = Tab::Banking;
                            }
                        }
                        if tab_2
                            .add_sized([100.0, 50.0], Button::new("Monthly"))
                            .clicked()
                        {
                            if self.tab == Tab::Monthly {
                                self.tab = Tab::None;
                            } else {
                                self.tab = Tab::Monthly;
                            }
                        }
                        if tab_3
                            .add_sized([100.0, 50.0], Button::new("Donations").fill(Color32::BLACK))
                            .clicked()
                        {
                            if self.tab == Tab::Donation {
                                self.tab = Tab::None;
                            } else {
                                self.tab = Tab::Donation;
                            }
                        }
                    }
                    Tab::None => {
                        if tab_1
                            .add_sized([100.0, 50.0], Button::new("Banking"))
                            .clicked()
                        {
                            if self.tab == Tab::Banking {
                                self.tab = Tab::None;
                            } else {
                                self.tab = Tab::Banking;
                            }
                        }
                        if tab_2
                            .add_sized([100.0, 50.0], Button::new("Monthly"))
                            .clicked()
                        {
                            if self.tab == Tab::Monthly {
                                self.tab = Tab::None;
                            } else {
                                self.tab = Tab::Monthly;
                            }
                        }
                        if tab_3
                            .add_sized([100.0, 50.0], Button::new("Donations"))
                            .clicked()
                        {
                            if self.tab == Tab::Donation {
                                self.tab = Tab::None;
                            } else {
                                self.tab = Tab::Donation;
                            }
                        }
                    }
                });
            });
    }
    fn tab_window(&mut self, ctx: &Context) {
        egui::TopBottomPanel::top("ID_APPWINDOW")
            .exact_height(800.0)
            .show(ctx, |ui| match self.tab {
                Tab::None => {}
                Tab::Banking => self.banking_calc.show(ui),
                Tab::Monthly => self.monthly_calc.show(ui),
                Tab::Donation => {}
            });
    }
}

impl eframe::App for CashCalculator {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.android_bar(ctx);
        self.menu_tabs(ctx);
        self.tab_window(ctx);
        self.numpad.show(ctx);
    }
    fn raw_input_hook(&mut self, ctx: &egui::Context, raw_input: &mut egui::RawInput) {
        self.numpad.bump_events(ctx, raw_input);
    }
}
