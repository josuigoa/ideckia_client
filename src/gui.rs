use egui::{Color32, Grid, Response, Ui};
use serde_json::Value;
use std::{thread, time::Duration};
use ws::Sender;

use crate::model::{ideckia_layout::IdeckiaLayout, item_state::ItemState};

static mut SENDER: Option<Sender> = None;
static mut LAYOUT: Option<IdeckiaLayout> = None;

pub struct GUI {
    delayed: bool,
    callback_delay_sec: u64,
}

impl GUI {
    pub fn new(_: &eframe::CreationContext<'_>) -> Self {
        GUI {
            delayed: false,
            callback_delay_sec: 0,
        }
    }

    pub fn button(ui: &mut Ui, item: &ItemState) -> Response {
        Self::sized_button(ui, item, 120., 120.)
    }

    pub fn sized_button(ui: &mut Ui, item: &ItemState, width: f32, height: f32) -> Response {
        let text = &item.text;
        let len = text.len();
        let s = if len <= 1 || text == "null" {
            ""
        } else {
            &text.as_str()[1..len - 1]
        };

        ui.add_sized(
            [width, height],
            egui::Button::new(s).wrap(true).stroke(egui::Stroke::new(
                2.,
                Color32::from_rgb(item.bg_color.r, item.bg_color.g, item.bg_color.b),
            )),
        )
    }

    pub fn set_sender(out: &Sender) {
        unsafe { SENDER = Some(out.to_owned()) };
    }

    pub fn set_layout(layout_data: &Value) {
        unsafe { LAYOUT = Some(IdeckiaLayout::new(layout_data)) };
    }

    pub fn send_click(&self, item_id: i64) {
        if self.delayed {
            thread::sleep(Duration::from_secs(self.callback_delay_sec));
        }
        unsafe {
            match &SENDER {
                Some(sender) => {
                    let json = format!(
                        "{{ \"type\": \"click\", \"whoami\": \"client\", \"itemId\": {} }}",
                        item_id
                    );
                    sender
                        .send(json)
                        .expect("Error sending item.click to server");
                }
                None => {}
            }
        }
    }
}

impl eframe::App for GUI {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::right("fixed_items").show(ctx, |ui| unsafe {
            let button_size = ui.available_width() - 10.;
            match &LAYOUT {
                Some(layout) => {
                    if layout.fixed_items.len() > 0 {
                        egui::ScrollArea::vertical().show(ui, |ui| {
                            for fixed_item in layout.fixed_items.iter() {
                                if GUI::sized_button(ui, &fixed_item, button_size, button_size)
                                    .clicked()
                                {
                                    self.send_click(fixed_item.id);
                                }
                            }
                        });
                    }
                }
                None => {}
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::warn_if_debug_build(ui);

            ui.horizontal(|ui| {
                ui.checkbox(&mut self.delayed, "Delay calls");
                ui.add_enabled(
                    self.delayed,
                    egui::Slider::new(&mut self.callback_delay_sec, 1..=10).text("seconds"),
                );
            });

            let width = ui.available_width() - 20.;
            let height = ui.available_height() - 10.;

            Grid::new("layout").show(ui, |ui| unsafe {
                match &LAYOUT {
                    Some(layout) => {
                        let width = width / (layout.columns as f32);
                        let height = height / (layout.rows as f32);
                        let mut index = 0;

                        for item in layout.items.iter() {
                            if GUI::sized_button(ui, &item, width, height).clicked() {
                                self.send_click(item.id);
                            }

                            index += 1;

                            if index % layout.columns == 0 {
                                ui.end_row();
                            }
                        }
                    }
                    None => {}
                }
            });
        });
    }

    fn save(&mut self, _storage: &mut dyn eframe::Storage) {}

    fn on_close_event(&mut self) -> bool {
        true
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {}

    fn auto_save_interval(&self) -> std::time::Duration {
        std::time::Duration::from_secs(30)
    }

    fn max_size_points(&self) -> egui::Vec2 {
        egui::Vec2::INFINITY
    }

    fn clear_color(&self, _visuals: &egui::Visuals) -> egui::Rgba {
        // NOTE: a bright gray makes the shadows of the windows look weird.
        // We use a bit of transparency so that if the user switches on the
        // `transparent()` option they get immediate results.
        egui::Color32::from_rgba_unmultiplied(12, 12, 12, 180).into()

        // _visuals.window_fill() would also be a natural choice
    }

    fn persist_native_window(&self) -> bool {
        true
    }

    fn persist_egui_memory(&self) -> bool {
        true
    }

    fn warm_up_enabled(&self) -> bool {
        false
    }

    fn post_rendering(&mut self, _window_size_px: [u32; 2], _frame: &eframe::Frame) {}
}
