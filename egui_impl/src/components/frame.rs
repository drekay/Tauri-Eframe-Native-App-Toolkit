//egui_impl/src/components/frame.rs
use eframe::egui;

const FRAME_HEIGHT: f32 = 100.0;

pub struct Frame {
    pub title: String,
    pub content: String,
    pub is_minimized: bool,
    frame_color: egui::Color32,
    title_bar_color: egui::Color32,
    pub is_being_dragged: bool,
}

impl Frame {
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_string(),
            content: "Window content goes here".to_string(),
            is_minimized: false,
            frame_color: egui::Color32::from_gray(240),
            title_bar_color: egui::Color32::from_gray(220),
            is_being_dragged: false,
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui, index: usize) -> (egui::Response, bool, bool, bool, usize, egui::Vec2) {
        let mut is_closed = false;
        let mut drag_started = false;
        let mut drag_released = false;
        let mut drag_delta = egui::Vec2::ZERO;

        let frame_response = egui::Frame::none()
           .fill(self.frame_color)
           .stroke(egui::Stroke::new(1.0, egui::Color32::BLACK))
           .rounding(egui::Rounding::same(5.0))
           /* .shadow(egui::epaint::Shadow {
                color: egui::Color32::from_black_alpha(60),
                offset: egui::vec2(2.0, 2.0),
                blur: 5.0,
                spread: 0.0,
            })*/
           .show(ui, |ui| {
                let available_width = ui.available_width();
                let (rect, response) = ui.allocate_exact_size(egui::vec2(available_width, FRAME_HEIGHT), egui::Sense::drag());
                
                if response.drag_started() {
                    drag_started = true;
                }
                if response.drag_released() {
                    drag_released = true;
                }
                self.is_being_dragged = response.dragged();
                if self.is_being_dragged {
                    println!("Frame {} is being dragged", index);
                }
                drag_delta = response.drag_delta();

                ui.allocate_ui_at_rect(rect, |ui| {
                    ui.vertical(|ui| {
                        // Title bar
                        ui.horizontal(|ui| {
                            egui::Frame::none()
                               .fill(self.title_bar_color)
                               .show(ui, |ui| {
                                    ui.set_min_width(ui.available_width());
                                    ui.horizontal(|ui| {
                                        ui.label(&self.title);
                                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                            if ui.button("❌").clicked() {
                                                is_closed = true;
                                                println!("clicked1");
                                            }
                                            if ui.button(if self.is_minimized { "🗖" } else { "🗕" }).clicked() {
                                                self.is_minimized = !self.is_minimized;
                                                println!("clicked2");
                                            }
                                        });
                                    });
                                });
                        });

                        // Content area
                        if !self.is_minimized {
                            egui::Frame::none()
                               .fill(self.frame_color)
                               .show(ui, |ui| {
                                    ui.set_min_width(ui.available_width());
                                    ui.label(&self.content);
                                    // Add more widgets here in the future
                                });
                        }
                    });
                });

                response
            });

        (frame_response.response, is_closed, drag_started, drag_released, index, drag_delta)
    }
}