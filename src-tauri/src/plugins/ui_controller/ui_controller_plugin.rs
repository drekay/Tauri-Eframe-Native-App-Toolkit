use eframe::egui;
use std::sync::{Arc, Mutex};
use crossbeam_channel::Receiver;
use crate::plugins::about_window::{AboutWindowState, Message, WindowInfo, DraggedWindow};

pub struct UiController {
    state: Arc<Mutex<AboutWindowState>>,
    receiver: Receiver<Message>,
}

impl UiController {
    pub fn new(state: Arc<Mutex<AboutWindowState>>, receiver: Receiver<Message>) -> Self {
        UiController { state, receiver }
    }

    pub fn update(&self, ctx: &egui::Context) -> Vec<Message> {
        let mut messages_to_send = Vec::new();

        // Process pending messages
        while let Ok(message) = self.receiver.try_recv() {
            let mut state = self.state.lock().unwrap();
            match message {
                Message::AddWindow => {
                    self.add_window(&mut state);
                }
                Message::CollapseWindow(window_id) => {
                    self.toggle_collapse(&mut state, window_id);
                }
                Message::DragWindowStart(window_id, pos) => {
                    self.start_drag(&mut state, window_id, pos);
                }
                Message::DragWindowMove(pos) => {
                    self.move_drag(&mut state, pos);
                }
                Message::DragWindowEnd => {
                    self.end_drag(&mut state);
                }
            }
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                let state = self.state.lock().unwrap();
                let gap_height = state.gap_height;
                let expanded_height = state.expanded_height;
                let collapsed_height = state.collapsed_height;
                let windows = state.windows.clone();
                drop(state);

                let mut y_offset = 0.0;
                for window in windows.iter() {
                    let window_height = if window.collapsed {
                        collapsed_height
                    } else {
                        expanded_height
                    };

                    let window_rect = egui::Rect::from_min_size(
                        egui::pos2(0.0, y_offset),
                        egui::vec2(ui.available_width(), window_height),
                    );

                    let response = ui.allocate_rect(window_rect, egui::Sense::click_and_drag());

                    ui.put(window_rect, |ui: &mut egui::Ui| {
                        egui::Frame::none()
                            .fill(ui.style().visuals.window_fill)
                            .show(ui, |ui| {
                                ui.horizontal(|ui| {
                                    ui.label(&window.title);
                                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                        let icon = if window.collapsed { "Expand" } else { "Collapse" };
                                        if ui.button(icon).clicked() {
                                            messages_to_send.push(Message::CollapseWindow(window.id));
                                        }
                                    });
                                });
                                if !window.collapsed {
                                    ui.label(&window.content);
                                }
                            })
                            .response
                    });

                    if ui.rect_contains_pointer(window_rect) {
                        if response.drag_started() {
                            messages_to_send.push(Message::DragWindowStart(window.id, (window_rect.left(), window_rect.top())));
                        }

                        if response.dragged() {
                            if let Some(pos) = response.interact_pointer_pos() {
                                messages_to_send.push(Message::DragWindowMove((pos.x, pos.y)));
                            }
                        }

                        if response.drag_released() {
                            messages_to_send.push(Message::DragWindowEnd);
                        }
                    }

                    y_offset += window_height + gap_height;
                }
            });
        });

        messages_to_send
    }

    fn add_window(&self, state: &mut AboutWindowState) {
        let new_id = state.about_counter + 1;
        let new_window = WindowInfo {
            id: new_id,
            title: format!("About{}", new_id),
            content: format!("This is About window #{}.\nThis is a Rust-only app using EGUI!", new_id),
            size: (400.0, 100.0),
            collapsed: false,
        };
        state.about_counter = new_id;
        state.windows.push(new_window);
    }

    fn toggle_collapse(&self, state: &mut AboutWindowState, window_id: usize) {
        if let Some(window) = state.windows.iter_mut().find(|w| w.id == window_id) {
            window.collapsed = !window.collapsed;
        }
    }

    fn start_drag(&self, state: &mut AboutWindowState, window_id: usize, pos: (f32, f32)) {
        let index = state.windows.iter().position(|w| w.id == window_id).unwrap();
        state.dragged_window = Some(DraggedWindow {
            index,
            start_pos: pos,
            current_pos: pos,
        });
    }

    fn move_drag(&self, state: &mut AboutWindowState, pos: (f32, f32)) {
        if let Some(ref mut dragged) = state.dragged_window {
            dragged.current_pos = pos;
        }
    }

    fn end_drag(&self, state: &mut AboutWindowState) {
        if let Some(dragged) = state.dragged_window.take() {
            let mut cumulative_height = 0.0;
            let new_index = state.windows.iter().position(|window| {
                cumulative_height += if window.collapsed {
                    state.collapsed_height + state.gap_height
                } else {
                    state.expanded_height
                };
                dragged.current_pos.1 < 10.0 + cumulative_height
            }).unwrap_or_else(|| state.windows.len().saturating_sub(1));

            if new_index != dragged.index {
                let window = state.windows.remove(dragged.index);
                state.windows.insert(new_index, window);
            }
        }
    }
}
