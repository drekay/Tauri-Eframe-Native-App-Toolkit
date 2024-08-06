use eframe::egui;
use std::sync::{Arc, Mutex};
use crossbeam_channel::Receiver;
use crate::plugins::about_window::{AboutWindowState, Message, WindowInfo, DraggedWindow};

pub struct UiController {
    state: Arc<Mutex<AboutWindowState>>,
    receiver: Receiver<Message>,
    central_panel_rect: egui::Rect,
    expand_icon: egui::TextureHandle,
    collapse_icon: egui::TextureHandle,
}

impl UiController {
    pub fn new(ctx: &egui::Context, state: Arc<Mutex<AboutWindowState>>, receiver: Receiver<Message>) -> Self {
        let expand_icon = Self::load_image_from_bytes(ctx, include_bytes!("../../../../assets/expand.png"), "expand_icon");
        let collapse_icon = Self::load_image_from_bytes(ctx, include_bytes!("../../../../assets/collapse.png"), "collapse_icon");
        
        UiController {
            state,
            receiver,
            central_panel_rect: egui::Rect::NOTHING,
            expand_icon,
            collapse_icon,
        }
    }

    fn load_image_from_bytes(ctx: &egui::Context, bytes: &[u8], name: &str) -> egui::TextureHandle {
        let image = image::load_from_memory(bytes).expect("Failed to load image");
        let size = [image.width() as _, image.height() as _];
        let image_buffer = image.to_rgba8();
        let pixels = image_buffer.as_flat_samples();
        let color_image = egui::ColorImage::from_rgba_unmultiplied(size, pixels.as_slice());
        ctx.load_texture(
            name,
            color_image,
            egui::TextureOptions::default(),
        )
    }

    pub fn update(&mut self, ctx: &egui::Context) -> Vec<Message> {
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
                    let pos2 = egui::pos2(pos.0, pos.1);
                    self.start_drag(&mut state, window_id, pos2);
                }
                Message::DragWindowMove(pos) => {
                    let pos2 = egui::pos2(pos.0, pos.1);
                    self.move_drag(&mut state, pos2);
                }
                Message::DragWindowEnd => {
                    self.end_drag(&mut state);
                }
               Message::CloseWindow(window_id) => {
                    self.close_window(&mut state, window_id);
                } 
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

        self.central_panel_rect = egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                let state = self.state.lock().unwrap();
                let windows = state.windows.clone();
                let grid = state.grid.clone();
                let dragged_window = state.dragged_window.clone();
                let expanded_height = state.expanded_height;
                let collapsed_height = state.collapsed_height;
                let gap_height = state.gap_height;
                drop(state);

                let mut drag_started = false;
                let mut drag_ended = false;

                for (grid_index, &window_index) in grid.iter().enumerate() {
                    let window = &windows[window_index];
                    let window_height = if window.collapsed { collapsed_height } else { expanded_height };
                    let mut window_pos = self.get_window_pos(grid_index, &windows, expanded_height, collapsed_height, gap_height);

                    if let Some(ref dragged) = dragged_window {
                        if dragged.index == window_index {
                            let pos2 = egui::pos2(dragged.current_pos.0, dragged.current_pos.1);
                            window_pos = pos2;
                        }
                    }

                    let window_response = egui::Window::new(&window.title)
                        .resizable(false)
                        .collapsible(false)
                        .title_bar(false)
                        .fixed_size(egui::vec2(window.size.0, window_height))
                        .current_pos(window_pos)
                        .show(ctx, |ui| {
                            ui.horizontal(|ui| {
                                ui.label(&window.title);
                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                    let icon = if window.collapsed { &self.expand_icon } else { &self.collapse_icon };
                                    if ui.add(egui::ImageButton::new(icon)).clicked() {
                                        messages_to_send.push(Message::CollapseWindow(window.id));
                                    }
                                });
                            });
                            if !window.collapsed {
                                ui.label(&window.content);
                            }
                        });

                    if let Some(response) = window_response {
                        let response = response.response;
                        if response.drag_started() {
                            let pos = (window_pos.x, window_pos.y);
                            messages_to_send.push(Message::DragWindowStart(window.id,pos));
                            drag_started = true;
                        }

                        if response.dragged() {
                            if let Some(pos) = response.interact_pointer_pos() {
                                messages_to_send.push(Message::DragWindowMove(pos.into()));
                            }
                        }

                        if response.drag_released() {
                            messages_to_send.push(Message::DragWindowEnd);
                            drag_ended = true;
                        }
                    }
                }

                if drag_started || drag_ended || dragged_window.is_some() {
                    ctx.request_repaint();
                }
            });
        }).response.rect;

        messages_to_send
    }

    fn get_window_pos(&self, grid_index: usize, windows: &[WindowInfo], expanded_height: f32, collapsed_height: f32, gap_height: f32) -> egui::Pos2 {
        let left_edge = self.central_panel_rect.left() + 10.0;
        let top_edge = self.central_panel_rect.top() + 10.0;
        let mut y_offset = 0.0;
        for window in windows.iter().take(grid_index) {
            y_offset += if window.collapsed {
                collapsed_height + gap_height
            } else {
                expanded_height + gap_height
            };
        }
        egui::pos2(left_edge, top_edge + y_offset)
    }

    fn add_window(&self, state: &mut AboutWindowState) {
        let new_id = state.about_counter + 1;

        let wsize = (400.0, 100.0);

        let new_window = WindowInfo {
            id: new_id,
            title: format!("About{}", new_id),
            content: format!("This is About window #{}.\nThis is a Rust-only app using EGUI!", new_id),
            size: wsize,
            collapsed: false,
        };
        state.about_counter = new_id;
        state.windows.push(new_window);
        state.grid.push(state.windows.len() - 1);
    }

    fn toggle_collapse(&self, state: &mut AboutWindowState, window_id: usize) {
        if let Some(window) = state.windows.iter_mut().find(|w| w.id == window_id) {
            window.collapsed = !window.collapsed;
        }
    }

    fn start_drag(&self, state: &mut AboutWindowState, window_id: usize, pos: egui::Pos2) {
        let index = state.windows.iter().position(|w| w.id == window_id).unwrap();
        
        let pos = (pos.x, pos.y);
        state.dragged_window = Some(DraggedWindow {
            index,
            start_pos: pos,
            current_pos: pos,
        });
    }

    fn move_drag(&self, state: &mut AboutWindowState, pos: egui::Pos2) {
        if let Some(ref mut dragged) = state.dragged_window {
            let pos = (pos.x, pos.y);
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
                    state.expanded_height + state.gap_height
                };
                dragged.current_pos.1 < self.central_panel_rect.top() + 10.0 + cumulative_height
            }).unwrap_or_else(|| state.windows.len().saturating_sub(1));

            let old_index = state.grid.iter().position(|&x| x == dragged.index).unwrap();
            if new_index != old_index {
                let window = state.grid.remove(old_index);
                state.grid.insert(new_index, window);
            }
        }
    }

    fn close_window(&self, state: &mut AboutWindowState, window_id: usize) {
        if let Some(index) = state.windows.iter().position(|w| w.id == window_id) {
            state.windows.remove(index);
            state.grid.retain(|&x| x != index);
            for grid_index in state.grid.iter_mut() {
                if *grid_index > index {
                    *grid_index -= 1;
                }
            }
        }
    }
}
