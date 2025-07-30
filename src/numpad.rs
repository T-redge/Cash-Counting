use eframe::egui::{self, Button, Ui, Vec2, pos2, vec2};

#[derive(Clone, Debug)]
struct State {
    start_pos: egui::Pos2,
    focus: Option<egui::Id>,
    events: Option<Vec<egui::Event>>,
}

impl State {
    fn new() -> Self {
        Self {
            start_pos: pos2(0.0, 1000.0),
            focus: None,
            events: None,
        }
    }

    fn queue_char(&mut self, c: char) {
        let events = self.events.get_or_insert(vec![]);
        if let Some(key) = egui::Key::from_name(&c.to_string()) {
            events.push(egui::Event::Key {
                key,
                physical_key: Some(key),
                pressed: true,
                repeat: false,
                modifiers: Default::default(),
            });
        }
        events.push(egui::Event::Text(c.to_string()));
    }

    fn queue_key(&mut self, key: egui::Key) {
        let events = self.events.get_or_insert(vec![]);
        events.push(egui::Event::Key {
            key,
            physical_key: Some(key),
            pressed: true,
            repeat: false,
            modifiers: Default::default(),
        });
    }
}
impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}
/// A simple keypad widget.
pub struct Keypad {
    id: egui::Id,
}

impl Keypad {
    pub fn new() -> Self {
        Self {
            id: egui::Id::new("keypad"),
        }
    }

    pub fn bump_events(&self, ctx: &egui::Context, raw_input: &mut egui::RawInput) {
        let events = ctx.memory_mut(|m| {
            m.data
                .get_temp_mut_or_default::<State>(self.id)
                .events
                .take()
        });
        if let Some(mut events) = events {
            events.append(&mut raw_input.events);
            raw_input.events = events;
        }
    }

    fn buttons(ui: &mut Ui, state: &mut State) {
        ui.vertical(|ui| {
            let window_margin = ui.spacing().window_margin;
            let size_1x1 = vec2(128.0, 75.0);
            let _size_1x2 = vec2(32.0, 52.0 + window_margin.topf());
            let _size_2x1 = vec2(64.0 + window_margin.leftf(), 26.0);

            ui.spacing_mut().item_spacing = Vec2::splat(window_margin.leftf());

            ui.horizontal(|ui| {
                if ui.add_sized(size_1x1, Button::new("1")).clicked() {
                    state.queue_char('1');
                }
                if ui.add_sized(size_1x1, Button::new("2")).clicked() {
                    state.queue_char('2');
                }
                if ui.add_sized(size_1x1, Button::new("3")).clicked() {
                    state.queue_char('3');
                }
            });
            ui.horizontal(|ui| {
                if ui.add_sized(size_1x1, Button::new("4")).clicked() {
                    state.queue_char('4');
                }
                if ui.add_sized(size_1x1, Button::new("5")).clicked() {
                    state.queue_char('5');
                }
                if ui.add_sized(size_1x1, Button::new("6")).clicked() {
                    state.queue_char('6');
                }
            });
            ui.horizontal(|ui| {
                if ui.add_sized(size_1x1, Button::new("7")).clicked() {
                    state.queue_char('7');
                }
                if ui.add_sized(size_1x1, Button::new("8")).clicked() {
                    state.queue_char('8');
                }
                if ui.add_sized(size_1x1, Button::new("9")).clicked() {
                    state.queue_char('9');
                }
            });
            ui.horizontal(|ui| {
                if ui.add_sized(size_1x1, Button::new("<-")).clicked() {
                    state.queue_key(egui::Key::Backspace);
                }
                if ui.add_sized(size_1x1, Button::new("0")).clicked() {
                    state.queue_char('0');
                }
                if ui.add_sized(size_1x1, Button::new("Enter")).clicked() {
                    state.queue_key(egui::Key::Tab);
                }
            });
        });
    }

    pub fn show(&self, ctx: &egui::Context) {
        let (focus, mut state) = ctx.memory(|m| {
            (
                m.focused(),
                m.data.get_temp::<State>(self.id).unwrap_or_default(),
            )
        });

        let y = ctx.style().spacing.interact_size.y * 1.25;

        state.start_pos = ctx.input(|i| {
            i.pointer
                .hover_pos()
                .map_or(pos2(0.0, 1000.0), |p| p + vec2(0.0, y))
        });
        if ctx.wants_keyboard_input() && state.focus != focus {
            state.focus = focus;
        }

        egui::Window::new("⌨ Keypad")
            .fixed_pos([0.0, 1000.0])
            .movable(false)
            .resizable(false)
            .collapsible(false)
            .title_bar(false)
            .show(ctx, |ui| Self::buttons(ui, &mut state));

        if let Some(focus) = state.focus {
            ctx.memory_mut(|m| {
                m.request_focus(focus);
            });
        }

        ctx.memory_mut(|m| m.data.insert_temp(self.id, state));
    }
}

impl Default for Keypad {
    fn default() -> Self {
        Self::new()
    }
}
