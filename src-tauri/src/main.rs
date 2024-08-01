#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::{egui, NativeOptions};

struct TauriEframeNativeApp {
    windows: Vec<WindowInfo>,
    about_counter: usize,
    central_panel_rect: egui::Rect,
    expand_icon: egui::TextureHandle,
    collapse_icon: egui::TextureHandle,
    grid_size: f32,
    dragged_window: Option<DraggedWindow>,
    user_input: String,
    grid: Vec<usize>,
}

struct WindowInfo {
    title: String,
    content: String,
    size: egui::Vec2,
    collapsed: bool,
}

struct DraggedWindow {
    index: usize,
    start_pos: egui::Pos2,
    current_pos: egui::Pos2,
}

impl TauriEframeNativeApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let expand_icon = load_image_from_bytes(&cc.egui_ctx, include_bytes!("../../assets/expand.png"), "expand_icon");
        let collapse_icon = load_image_from_bytes(&cc.egui_ctx, include_bytes!("../../assets/collapse.png"), "collapse_icon");

        Self {
            windows: Vec::new(),
            about_counter: 0,
            central_panel_rect: egui::Rect::NOTHING,
            expand_icon,
            collapse_icon,
            grid_size: 110.0,
            dragged_window: None,
            user_input: String::new(),
            grid: Vec::new(),
        }
    }

    fn add_about_window(&mut self) {
        self.about_counter += 1;
        self.windows.push(WindowInfo {
            title: format!("About{}", self.about_counter),
            content: format!("This is About window #{}.\nThis is a Tauri Native App using EFrame!", self.about_counter),
            size: egui::vec2(400.0, 100.0),
            collapsed: false,
        });
        self.grid.push(self.windows.len() - 1);
    }

    fn get_window_pos(&self, grid_index: usize) -> egui::Pos2 {
        let left_edge = self.central_panel_rect.left() + 10.0;
        let top_edge = self.central_panel_rect.top() + 10.0;
        egui::pos2(left_edge, top_edge + (grid_index as f32 * self.grid_size))
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
        egui::TextureOptions {
            magnification: egui::TextureFilter::Linear,
            minification: egui::TextureFilter::Linear,
            ..Default::default()
        }
    )
}

impl eframe::App for TauriEframeNativeApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("New").clicked() { println!("New file"); }
                    if ui.button("Open").clicked() { println!("Open file"); }
                    if ui.button("Save").clicked() { println!("Save file"); }
                    if ui.button("Quit").clicked() { std::process::exit(0); }
                });
                ui.menu_button("Edit", |ui| {
                    if ui.button("Cut").clicked() { println!("Cut"); }
                    if ui.button("Copy").clicked() { println!("Copy"); }
                    if ui.button("Paste").clicked() { println!("Paste"); }
                });
                ui.menu_button("Help", |ui| {
                    if ui.button("About").clicked() { 
                        self.add_about_window();
                    }
                });
            });
        });

        self.central_panel_rect = egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Tauri EFrame Native Demo App");
            ui.add_space(4.0);
            ui.label("Click 'Help' -> 'About' to open new About windows.");
            ui.label("Drag windows vertically to reorder them.");
        }).response.rect;

        let mut drag_started = false;
        let mut drag_ended = false;

        // Calculate window positions before the mutable borrow
        let window_positions: Vec<_> = self.grid.iter().enumerate()
            .map(|(grid_index, &window_index)| (grid_index, window_index, self.get_window_pos(grid_index)))
            .collect();

        for (grid_index, window_index, window_pos) in window_positions {
            let window = &mut self.windows[window_index];

            let mut current_pos = window_pos;
            if let Some(ref dragged) = self.dragged_window {
                if dragged.index == window_index {
                    current_pos = dragged.current_pos;
                }
            }

            let window_response = egui::Window::new(&window.title)
                .resizable(false)
                .collapsible(false)
                .title_bar(false)
                .fixed_size(if window.collapsed { egui::vec2(window.size.x, 30.0) } else { window.size })
                .current_pos(current_pos)
                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        ui.label(&window.title);
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            let icon = if window.collapsed { &self.expand_icon } else { &self.collapse_icon };
                            if ui.add(egui::ImageButton::new(icon)).clicked() {
                                window.collapsed = !window.collapsed;
                            }
                        });
                    });
                    if !window.collapsed {
                        ui.label(&window.content);
                    }
                })
                .unwrap();

            let response = window_response.response;

            if response.drag_started() {
                self.dragged_window = Some(DraggedWindow {
                    index: window_index,
                    start_pos: window_pos,
                    current_pos: window_pos,
                });
                drag_started = true;
            }

            if let Some(ref mut dragged) = self.dragged_window {
                if dragged.index == window_index {
                    dragged.current_pos += response.drag_delta();
                }
            }

            if response.drag_released() {
                drag_ended = true;
            }
        }

        // Update grid positions based on dragging
        if drag_ended {
            if let Some(dragged) = self.dragged_window.take() {
                let new_index = ((dragged.current_pos.y - self.central_panel_rect.top() - 10.0) / self.grid_size).round() as isize;
                let new_index = new_index.clamp(0, (self.grid.len() - 1) as isize) as usize;
                let old_index = self.grid.iter().position(|&x| x == dragged.index).unwrap();

                if new_index != old_index {
                    let window = self.grid.remove(old_index);
                    self.grid.insert(new_index, window);
                }
            }
        }

        if drag_started || drag_ended || self.dragged_window.is_some() {
            ctx.request_repaint();
        }
    }
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
  let options = NativeOptions::default();

  eframe::run_native(
      "Tauri EFrame Native Demo",
      options,
      Box::new(|cc| Ok(Box::new(TauriEframeNativeApp::new(cc)))),
  )?;

  Ok(())
}