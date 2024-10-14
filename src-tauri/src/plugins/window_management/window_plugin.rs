// plugins/window_management/about_window_plugin.rs
use std::sync::{Arc, Mutex};
use eframe::egui::{self};
use std::collections::HashMap;
use crossbeam_channel::{unbounded, Receiver, Sender};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use super::FramePlugin;
const FRAME_HEIGHT: f32 = 100.0;

impl FramePlugin for WindowPlugin {
    #[allow(unused_variables)]
    fn handle_message(&self, message: Message) {
        let mut state = self.state.lock().unwrap();
    match message {
        Message::AddWindow => {
            state.about_counter += 1;
            let new_window = Frame::new("About".to_string(), state.about_counter,false);
            let window_index = state.windows.len();
            state.windows.push(new_window);
            state.grid.push(window_index);
        },
        Message::ConfirmedCloseWindow(index) => {
            let mut state = self.state.lock().unwrap();
            if index < state.windows.len() {
                state.windows.remove(index);
                // Update grid and other state as necessary
                state.grid.retain(|&x| x != index);
                // Adjust indices in grid that are greater than the removed index
                for grid_index in state.grid.iter_mut() {
                    if *grid_index > index {
                        *grid_index -= 1;
                    }
                }
                println!("confirmed window close");
            }
        },  
            Message::CollapseWindow(window_id) => todo!(),
            Message::DragWindowStart(window_id, pos) => todo!(),
            Message::DragWindowMove(pos) => todo!(),
            _ => {
                println!("message ignored")
            } // Ignore messages not relevant to this plugin
        }
    }       

    fn update(&self) {
        if let Some(receiver) = &self.receiver {
        while let Ok(message) = receiver.try_recv() {
            println!("receiving...");
            self.handle_message(message);
        }
        }
    }  

   fn execute(&self, ui: &mut egui::Ui, ctx: &egui::Context) {
       // println!("about window plugin render windows");
        self.render_windows(ui);
    }

    fn is_dragging(&self) -> bool {
        self.dragged_window.is_some()
    }
    
    fn name(&self) -> &str {
        &self.name
    }
    
    fn set_receiver(&mut self, rx: Receiver<Message>) {
        self.receiver = Some(rx);  // Set the receiver when the plugin is registered
    }
    
   
}

pub struct WindowPlugin {
    windows: Vec<Frame>,
    about_counter: usize,
    grid: Vec<usize>,
    dragged_window: Option<DraggedWindow>,
    drag_offset: egui::Vec2,
    tx: Sender<Message>,
    receiver: Option<Receiver<Message>>,
    state: Arc<Mutex<WindowState>>,
    name: String,
}

pub struct Frame {
    title: String,
    uuid: usize,
    is_minimized: bool,
    frame_color: egui::Color32,
    title_bar_color: egui::Color32,
    is_being_dragged: bool,
    is_placeholder: bool,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct WindowInfo {
    pub id: usize,
    pub title: String,
    pub content: String,
    pub size: (f32, f32),
    pub collapsed: bool,
    pub is_being_dragged: bool,
}

pub struct WindowState {
    pub windows: Vec<Frame>,
    pub grid: Vec<usize>,
    pub about_counter: usize,
    pub expanded_height: f32,
    pub collapsed_height: f32,
    pub sender: Sender<Message>,
}

impl WindowState {
    pub fn new(sender: Sender<Message>) -> Self {
        Self {
            windows: Vec::new(),
            grid: Vec::new(),
            about_counter: 0,
            expanded_height: 100.0, 
            collapsed_height: 30.0, 
            sender,
        }
    }
}
pub struct DraggedWindow {
    index: usize,
    start_pos: egui::Pos2,
    current_pos: egui::Pos2,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Message {
    AddWindow,
    CollapseWindow(usize),
    DragWindowStart(usize, (f32, f32)),
    DragWindowMove((f32, f32)), 
    MinimizeWindow(usize),
   
    CloseWindow(usize),
    ConfirmedCloseWindow(usize),
} //DragWindowEnd,

//// MESSAGE BUS ////
pub struct MessageBus {
    pub senders: HashMap<String, Sender<Message>>,
    pub receiver: Receiver<Message>,
}

impl MessageBus {
    pub fn new() -> Self {
        let (tx, rx) = unbounded();
        Self {
            senders: HashMap::new(),
            receiver: rx,
        }
    }

    pub fn register_plugin(&mut self, plugin_name: &str) -> Receiver<Message> {
        let (tx, rx) = unbounded();
        self.senders.insert(plugin_name.to_string(), tx);
        rx
    }

    pub fn send(&self, target: &str, message: Message) {
        if let Some(sender) = self.senders.get(target) {
            sender.send(message).unwrap();
        }
    }

    pub fn broadcast(&self, message: Message) {
        for sender in self.senders.values() {
            sender.send(message.clone()).unwrap();
        }
    }
}
////  END MESSAGE BUS ////

impl WindowPlugin {
    pub fn new(tx: Sender<Message>, rx: Receiver<Message>)  -> Self {
        let mut plugin = Self {
            name: "WindowPlugin".to_string(),
            windows: Vec::new(),
            about_counter: 0,
            grid: Vec::new(),
            dragged_window: None,
            drag_offset: egui::Vec2::ZERO,
            state: Arc::new(Mutex::new(WindowState {
                windows: Vec::new(),
                grid: Vec::new(),
                about_counter: 0,
                expanded_height: 100.0,
                collapsed_height: 30.0, 
                sender: tx.clone(), 
            })),
            tx: tx,
            receiver: None,
        };
        plugin.add_placeholder_window();
        plugin
    }

    pub fn render_windows(&self, ui: &mut egui::Ui) {
        match self.state.try_lock() {
            Ok(mut state) => {
                for (index, window) in state.windows.iter_mut().enumerate() {
                    self.render_window(ui, window, index);
                }
            },
            Err(_) => {
                // Handle the case where we couldn't acquire the lock
                ui.label("Unable to render windows at this time.");
            }
        }
    }

    fn render_window(&self, ui: &mut egui::Ui, window: &mut Frame, index: usize) {

        let (response, is_closed, drag_started, drag_released, _, drag_delta) = if window.is_placeholder {
            window.show_placeholder(ui)
        } else {
            window.show(ui, index)
        };

        /*let mut messages_to_send = Vec::new();
    
        if is_closed {
            messages_to_send.push(Message::CloseWindow(index));
        }
    
        if drag_started {
            messages_to_send.push(Message::DragWindowStart(index, response.rect.min));
        }
    
        if drag_released {
            messages_to_send.push(Message::DragWindowEnd);
        }
    
        if response.dragged() {
            messages_to_send.push(Message::DragWindowMove(response.rect.min + drag_delta));
        }
    
        // Send collected messages
        for message in messages_to_send {
            let _ = self.sender.send(message);
        }*/
    }

     /*fn render_window(&self, ui: &mut egui::Ui, window: &mut Frame, index: usize) {
        let (response, is_closed, drag_started, drag_released, _, drag_delta) = 
            window.show(ui, index,None, None, None);

        let mut messages_to_send = Vec::new();

        if is_closed {
            messages_to_send.push(Message::CloseWindow(index));
        }


       todo if drag_started {
            messages_to_send.push(Message::DragWindowStart(index, response.rect.min));
        }

        if drag_released {
            messages_to_send.push(Message::DragWindowEnd);
        }

        if response.dragged() {
            messages_to_send.push(Message::DragWindowMove(response.rect.min + drag_delta));
        }

        // Send collected messages
        for message in messages_to_send {
            let _ = self.sender.send(message);
        }
    }*/
    ////Todo: use this instead
    fn add_window(&mut self) {
        self.about_counter += 1;
        self.windows.push(Frame::new(
            "&format!(About{}, self.about_counter".to_string(),  self.about_counter,
            false
        ));
        self.grid.insert(self.grid.len() - 1, self.windows.len() - 1);
    }

    fn add_placeholder_window(&mut self) {
        //self.windows.push(Frame::new("Placeholder".to_string(),   self.about_counter,true)); // is a placeholder
        //self.grid.push(self.windows.len() - 1);
        //println!("add_placeholder_window");

        let new_window = Frame::new("Placeholder".to_string(), 0,true);
        let mut state = self.state.lock().unwrap();
        let window_index = state.windows.len();
        state.windows.push(new_window);
        state.grid.push(window_index);
    }

    pub fn close_window(&mut self, index: usize) {
        self.windows.remove(index);
        self.grid.retain(|&x| x != index);
        for i in self.grid.iter_mut() {
            if *i > index {
                *i -= 1;
            }
        }
    }

    pub fn minimize_window(&mut self, index: usize) {
        self.windows[index].is_minimized = !self.windows[index].is_minimized;
    }

    /* 
    fn add_placeholder_window(&mut self) {
        self.windows.push(Frame::new("Placeholder"));
        self.grid.push(self.windows.len() - 1);
    }
    fn add_placeholder_window(&mut self) {
        self.windows.push(Frame::new("Placeholder".to_string(),   self.about_counter,true)); // is a placeholder
        self.grid.push(self.windows.len() - 1);
    println!("add_placeholder_window");
    }*/
}

impl Frame {
    fn new(title:String, uuid:usize, is_placeholder: bool) -> Self {
        Self {
            title,
            uuid:uuid,
            is_minimized: false,
            frame_color: egui::Color32::from_gray(240),
            title_bar_color: egui::Color32::from_gray(220),
            is_being_dragged: false,
            is_placeholder:is_placeholder
        }
    }

    fn show(&mut self, ui: &mut egui::Ui, index: usize) -> 
    (egui::Response, bool, bool, bool, usize, egui::Vec2) {
         let mut is_closed = false;
        let mut drag_started = false;
        let mut drag_released = false;
        let mut drag_delta = egui::Vec2::ZERO;

        let frame_response = egui::Frame::none()
           .fill(self.frame_color)
           .stroke(egui::Stroke::new(1.0, egui::Color32::BLACK))
           .rounding(egui::Rounding::same(5.0))
           .shadow(egui::epaint::Shadow {
                color: egui::Color32::from_black_alpha(60),
                offset: egui::vec2(2.0, 2.0),
                blur: 5.0,
                spread: 0.0,
            })
           .show(ui, |ui| {
                let available_width = ui.available_width();
                let (rect, mut response) = ui.allocate_exact_size(egui::vec2(available_width, FRAME_HEIGHT), egui::Sense::click_and_drag());
                
                ui.allocate_ui_at_rect(rect, |ui| {
                    ui.vertical(|ui| {
                        // Title bar
                        ui.horizontal(|ui| {
                            ui.label(&self.title);
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                if ui.button("❌").clicked() {
                                    is_closed = true;
                                    println!("Close button clicked");
                                }
                                if ui.button(if self.is_minimized { "🗖" } else { "🗕" }).clicked() {
                                    self.is_minimized = !self.is_minimized;
                                    println!("Minimize/Maximize button clicked");
                                }
                            });
                        });

                        // Content area
                        if !self.is_minimized {
                            ui.label(&self.uuid.to_string());
                        }
                    });
                });

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

                response
            });

       let response = frame_response.inner;
        (response, is_closed, drag_started, drag_released, index, drag_delta)
    }

    fn show_placeholderold(&self, ui: &mut egui::Ui) ->  (egui::Response, bool, bool, bool, usize, egui::Vec2){
        let frame_response = egui::Frame::none()
            .fill(egui::Color32::TRANSPARENT)
            .show(ui, |ui| {
                let available_width = ui.available_width();
                let (rect, response) = ui.allocate_exact_size(egui::vec2(available_width, FRAME_HEIGHT), egui::Sense::hover());
                
                ui.allocate_ui_at_rect(rect, |ui| {
                    //ui.label(&self.title);
                    //ui.label("holder");
                });
                response
            });

        let response = frame_response.inner;
        (response, todo!(), todo!(), todo!(), todo!(), todo!())    
    }

    fn show_placeholder(&self, ui: &mut egui::Ui) -> (egui::Response, bool, bool, bool, usize, egui::Vec2) {
        //println!("show placeholder");
        let (id, rect) = ui.allocate_space(egui::vec2(ui.available_width(), 20.0));
        let response = ui.interact(rect, id, egui::Sense::hover());
        (response, false, false, false, 0, egui::Vec2::ZERO)
    }

}