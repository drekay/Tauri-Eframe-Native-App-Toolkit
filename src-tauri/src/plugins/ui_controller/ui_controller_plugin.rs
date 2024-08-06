use eframe::egui;
use std::sync::{Arc, Mutex};
use crossbeam_channel::Receiver;
use crate::plugins::about_window::{AboutWindowState, Message, WindowInfo, DraggedWindow};
use image::GenericImageView;

pub struct UiController {
    state: Arc<Mutex<AboutWindowState>>,
    receiver: Receiver<Message>,
}

impl UiController {
    pub fn new(state: Arc<Mutex<AboutWindowState>>, receiver: Receiver<Message>) -> Self {
        let ui_controller = UiController { state, receiver };
        
        // Load icons
       /* let ctx = egui::Context::default();
        ui_controller.load_texture(&ctx, "collapse_icon", include_bytes!("../../../../assets/collapse_icon.png"));
        ui_controller.load_texture(&ctx, "expand_icon", include_bytes!("../../../../assets/expand_icon.png"));
        */
        ui_controller
    }

    fn load_texture(&self, ctx: &egui::Context, name: &str, image_data: &[u8]) {
        let image = image::load_from_memory(image_data).unwrap().to_rgba8();
        let size = [image.width() as _, image.height() as _];
        let image_buffer = image.into_raw();
        let color_image = egui::ColorImage::from_rgba_unmultiplied(size, &image_buffer);
        ctx.load_texture(name, color_image, Default::default());
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
               /* Message::CloseWindow(window_id) => {
                    self.close_window(&mut state, window_id);
                }*/
            }
        }
    
        // Add the menu bar
        egui::TopBottomPanel::top("ui_controller_top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
                ui.menu_button("About", |ui| {
                    if ui.button("Add About Window").clicked() {
                        messages_to_send.push(Message::AddWindow);
                    }
                });
            });
        });
    
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                let mut state = self.state.lock().unwrap();
                let windows = state.windows.clone();
                let dragged_window = state.dragged_window.clone();
                drop(state);
    
                let mut drag_started = false;
                let mut drag_ended = false;
    
                for (index, window) in windows.iter().enumerate() {
                    let mut is_open = true;
                    let window_id = egui::Id::new(format!("about_window_{}", window.id));
                    let mut window_ui = egui::Window::new(&window.title)
                        .id(window_id)
                        .resizable(false)
                        .collapsible(false)
                        .default_size(window.size)
                        .open(&mut is_open);
    
                    if let Some(ref dragged) = dragged_window {
                        if dragged.index == index {
                            window_ui = window_ui.current_pos(egui::pos2(dragged.current_pos.0, dragged.current_pos.1));
                        }
                    }
    
                    let response = window_ui.show(ctx, |ui| {
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
                    });
    
                    if let Some(response) = response {
                        if response.response.drag_started() {
                            messages_to_send.push(Message::DragWindowStart(window.id, response.response.rect.left_top().into()));
                            drag_started = true;
                        }
    
                        if response.response.dragged() {
                            if let Some(pos) = response.response.interact_pointer_pos() {
                                messages_to_send.push(Message::DragWindowMove((pos.x, pos.y)));
                            }
                        }
    
                        if response.response.drag_released() {
                            messages_to_send.push(Message::DragWindowEnd);
                            drag_ended = true;
                        }
    
                       /* if !is_open {
                            messages_to_send.push(Message::CloseWindow(window.id));
                        }*/
                    }
                }
    
                if drag_started || drag_ended || dragged_window.is_some() {
                    ctx.request_repaint();
                }
            });
        });
    
        // Send all collected messages
        for message in messages_to_send.clone() {
            let _ = self.state.lock().unwrap().sender.send(message);
        }
    
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

    fn close_window(&self, state: &mut AboutWindowState, window_id: usize) {
        state.windows.retain(|w| w.id != window_id);
    }
}
