use crate::*;
use eframe::egui;
use std::collections::BTreeMap;
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Week {
    One,
    Two,
    Three,
    Four,
    Five,
}
impl Week {
    pub fn return_week_num(&self) -> String {
        format!("Week {:?}", self)
    }
}
pub struct WeekCalc {
    week: BTreeMap<Week, f32>,
}
impl WeekCalc {
    pub fn new() -> Self {
        Self {
            week: init_week_map(),
        }
    }
    pub fn weekly_totals(&mut self, ui: &mut egui::Ui, curr_wk: Week) {
        ui.columns_const(|[ui_1, ui_2, ui_3]| {
            ui_1.vertical_centered(|ui_1| {
                ui_1.label(create_label(&curr_wk.return_week_num()).underline());
            });
            ui_2.vertical_centered(|ui_2| {
                ui_2.label(create_label("Totals").underline());
            });
            ui_3.vertical_centered(|ui_3| {
                ui_3.label(
                    create_label(self.week.get(&curr_wk).unwrap().to_string().as_str()).underline(),
                );
            });
        });
    }
}

fn init_week_map() -> BTreeMap<Week, f32> {
    let mut init = BTreeMap::new();
    init.insert(Week::One, 0.0);
    init.insert(Week::Two, 0.0);
    init.insert(Week::Three, 0.0);
    init.insert(Week::Four, 0.0);
    init.insert(Week::Five, 0.0);
    init
}
