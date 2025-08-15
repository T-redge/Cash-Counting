use std::collections::BTreeMap;
pub struct WeekCalc {
    weekly_data: BTreeMap<String, (String, f32)>,
}
impl WeekCalc {
    pub fn new() -> Self {
        Self {
            weekly_data: BTreeMap::new(),
        }
    }
}
