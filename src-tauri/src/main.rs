use eframe::{egui, NativeOptions};

// MODEL
struct TauriEframeNativeAppModel {
    windows: Vec<WindowInfo>,
    about_counter: usize,
    central_panel_rect: egui::Rect,
    expand_icon: egui::TextureHandle,
    collapse_icon: egui::TextureHandle,
    expanded_height: f32,
    collapsed_height: f32,
    gap_height: f32,
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

// MESSAGE
enum Message {
    AddAboutWindow,
    CollapseWindow(usize),
    DragWindowStart(usize, egui::Pos2),
    DragWindowMove(egui::Pos2),
    DragWindowEnd,
    UpdateUserInput(String),
    SubmitUserInput,
}

// UPDATE
impl TauriEframeNativeAppModel {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let expand_icon = load_image_from_bytes(&cc.egui_ctx, include_bytes!("../../assets/expand.png"), "expand_icon");
        let collapse_icon = load_image_from_bytes(&cc.egui_ctx, include_bytes!("../../assets/collapse.png"), "collapse_icon");

        Self {
            windows: Vec::new(),
            about_counter: 0,
            central_panel_rect: egui::Rect::NOTHING,
            expand_icon,
            collapse_icon,
            expanded_height: 80.0,
            collapsed_height: 40.0,
            gap_height: 10.0,
            dragged_window: None,
            user_input: String::new(),
            grid: Vec::new(),
        }
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::AddAboutWindow => self.add_about_window(),
            Message::CollapseWindow(index) => self.collapse_window(index),
            Message::DragWindowStart(index, pos) => self.start_drag_window(index, pos),
            Message::DragWindowMove(pos) => self.move_drag_window(pos),
            Message::DragWindowEnd => self.end_drag_window(),
            Message::UpdateUserInput(input) => self.user_input = input,
            Message::SubmitUserInput => println!("User input: {}", self.user_input),
        }
    }

    fn add_about_window(&mut self) {
        self.about_counter += 1;
        self.windows.push(WindowInfo {
            title: format!("About{}", self.about_counter),
            content: format!("This is About Window #{}.\nThis is a Tauri Native App using EFrame!", self.about_counter),
            size: egui::vec2(400.0, 100.0),
            collapsed: false,
        });
        self.grid.push(self.windows.len() - 1);
    }

    fn collapse_window(&mut self, index: usize) {
        if let Some(window) = self.windows.get_mut(index) {
            window.collapsed = !window.collapsed;
        }
    }

    fn start_drag_window(&mut self, index: usize, pos: egui::Pos2) {
        self.dragged_window = Some(DraggedWindow {
            index,
            start_pos: pos,
            current_pos: pos,
        });
    }

    fn move_drag_window(&mut self, pos: egui::Pos2) {
        if let Some(ref mut dragged) = self.dragged_window {
            dragged.current_pos = pos;
        }
    }

    fn end_drag_window(&mut self) {
        if let Some(dragged) = self.dragged_window.take() {
            let mut cumulative_height = 0.0;
            let new_index = self.grid.iter().position(|&index| {
                cumulative_height += self.get_window_height(index);
                dragged.current_pos.y < self.central_panel_rect.top() + 10.0 + cumulative_height
            }).unwrap_or_else(|| self.grid.len().saturating_sub(1));

            let old_index = self.grid.iter().position(|&x| x == dragged.index).unwrap();

            if new_index != old_index {
                let window = self.grid.remove(old_index);
                self.grid.insert(new_index, window);
            }
        }
    }

    fn get_window_pos(&self, grid_index: usize) -> egui::Pos2 {
        let left_edge = self.central_panel_rect.left() + 10.0;
        let top_edge = self.central_panel_rect.top() + 10.0;
        let mut y_offset = 0.0;
        for &window_index in self.grid.iter().take(grid_index) {
            y_offset += if self.windows[window_index].collapsed {
                self.collapsed_height + self.gap_height
            } else {
                self.expanded_height
            };
        }
        egui::pos2(left_edge, top_edge + y_offset)
    }

    fn get_window_height(&self, window_index: usize) -> f32 {
        if self.windows[window_index].collapsed {
            self.collapsed_height
        } else {
            self.expanded_height
        }
    }
}

// VIEW
impl eframe::App for TauriEframeNativeAppModel {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.view(ctx);
    }
}

impl TauriEframeNativeAppModel {
    fn view(&mut self, ctx: &egui::Context) {
        self.view_top_panel(ctx);
        self.view_central_panel(ctx);
        self.view_windows(ctx);
    }

    fn view_top_panel(&mut self, ctx: &egui::Context) {
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
                        self.update(Message::AddAboutWindow);
                    }
                });
            });
        });
    }

    fn view_central_panel(&mut self, ctx: &egui::Context) {
        self.central_panel_rect = egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Tauri EFrame Native Demo App");
            ui.add_space(4.0);
            ui.label("Click 'Help' -> 'About' to open new About windows.");
            ui.label("Drag windows vertically to reorder them.");

            ui.horizontal(|ui| {
                ui.label("Enter text:");
                let response = ui.text_edit_singleline(&mut self.user_input);
                if response.changed() {
                    self.update(Message::UpdateUserInput(self.user_input.clone()));
                }
                if ui.button("Submit").clicked() {
                    self.update(Message::SubmitUserInput);
                }
            });
        }).response.rect;
    }

    fn view_windows(&mut self, ctx: &egui::Context) {
        let window_data: Vec<_> = self.grid.iter().enumerate()
            .map(|(grid_index, &window_index)| {
                let window = &self.windows[window_index];
                let window_pos = self.get_window_pos(grid_index);
                let window_height = self.get_window_height(window_index);
                (window_index, window.title.clone(), window.content.clone(), window.collapsed, window.size, window_pos, window_height)
            })
            .collect();
    
        let mut messages = Vec::new();
    
        for (window_index, title, content, collapsed, size, window_pos, window_height) in window_data {
            let mut current_pos = window_pos;
            if let Some(ref dragged) = self.dragged_window {
                if dragged.index == window_index {
                    current_pos = dragged.current_pos;
                }
            }
    
            let window_response = egui::Window::new(&title)
                .resizable(false)
                .collapsible(false)
                .title_bar(false)
                .fixed_size(egui::vec2(size.x, window_height))
                .current_pos(current_pos)
                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        ui.label(&title);
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            let icon = if collapsed { &self.expand_icon } else { &self.collapse_icon };
                            if ui.add(egui::ImageButton::new(icon)).clicked() {
                                messages.push(Message::CollapseWindow(window_index));
                            }
                        });
                    });
                    if !collapsed {
                        ui.label(&content);
                    }
                })
                .unwrap();
    
            let response = window_response.response;
    
            if response.drag_started() {
                messages.push(Message::DragWindowStart(window_index, window_pos));
            }
    
            if let Some(ref dragged) = self.dragged_window {
                if dragged.index == window_index {
                    messages.push(Message::DragWindowMove(current_pos + response.drag_delta()));
                }
            }
    
            if response.drag_released() {
                messages.push(Message::DragWindowEnd);
            }
        }
    
        for message in messages {
            self.update(message);
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let options = NativeOptions::default();
    eframe::run_native(
        "Tauri EFrame Native Demo",
        options,
        Box::new(|cc| Ok(Box::new(TauriEframeNativeAppModel::new(cc)))),
    )?;
    Ok(())
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