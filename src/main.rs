use eframe::{self, NativeOptions, egui::ViewportBuilder};
use retail_doc::CashCalculator;

fn main() {
    let options = NativeOptions {
        viewport: ViewportBuilder::default().with_inner_size([412.0, 915.0]),
        ..Default::default()
    };
    eframe::run_native(
        "KLAM",
        options,
        Box::new(|cc| Ok(Box::new(CashCalculator::new(cc)))),
    )
    .unwrap();
}
