use eframe::egui;
use crossbeam_channel::{Sender, Receiver};
use std::sync::{Arc, Mutex};

//use crate::plugins::window_management::window_plugin::{Message, Plugin, UIPlugin,WindowPluginMessage};
use crate::{plugins::window_management::WindowState, Message, Plugin, UIPlugin, WindowPluginMessage};

pub(crate)  struct UiControllerPlugin {
    state: Arc<Mutex<WindowState>>,
    tx: Sender<Message>,
    rx: Receiver<Message>,
}

impl UiControllerPlugin {
    pub fn new(tx: Sender<Message>, rx: Receiver<Message>, state: Arc<Mutex<WindowState>>) -> Self {
        Self {
            state,
            tx,
            rx,
        }
    }

    fn show_top_panel(&self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("New").clicked() {                
                        let _ = self.tx.send(Message::WindowPlugin(WindowPluginMessage::AddWindow));
                        println!("Add window button pressed");
                    }
                    if ui.button("Open").clicked() { println!("Open file"); }
                    if ui.button("Save").clicked() { println!("Save file"); }
                    if ui.button("Close").clicked() {
                        //TODO let _ = self.tx.send(Message::CloseWindow(1)); //todo: pass window_id
                        //let _ = self.tx.send(Message::WindowPlugin(crate::WindowPluginMessage::Close???));
                    }
                    if ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
                ui.menu_button("Edit", |ui| {
                    if ui.button("Cut").clicked() { println!("Cut"); }
                    if ui.button("Copy").clicked() { println!("Copy"); }
                    if ui.button("Paste").clicked() { println!("Paste"); }
                });
                ui.menu_button("Help", |ui| {
                    if ui.button("About").clicked() { println!("About"); }
                });
            });
        });
    }

    fn show_central_panel(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Tauri Eframe Native App Toolkit");
            ui.add_space(4.0);
            ui.label("Click 'File' -> 'New' to open new About windows.");
            ui.label("Drag windows vertically to reorder them.");
            ui.label("Scroll up and down if you added many windows.");

            egui::ScrollArea::vertical().show(ui, |ui| {
                let mut state = self.state.lock().unwrap();
                let mut windows_to_remove = Vec::new();

                for (index, window) in state.windows.iter_mut().enumerate() {
                    //let (response, is_closed, drag_started, drag_released, _, drag_delta) = 
                      let _ =  window.show(ui);//, index);

                    /*if is_closed {
                        windows_to_remove.push(index);
                    }

                    if drag_started {
                        state.dragged_window = Some(index);
                    }

                    if drag_released {
                        if let Some(dragged_index) = state.dragged_window {
                            if dragged_index != index {
                                let dragged_window = state.windows.remove(dragged_index);
                                state.windows.insert(index, dragged_window);
                            }
                        }
                        state.dragged_window = None;
                    }

                    if response.dragged() && state.dragged_window.is_some() {
                        ctx.request_repaint();
                    }*/
                }

                // Remove closed windows
                for index in windows_to_remove.iter().rev() {
                    state.windows.remove(*index);
                }
            });

            /* if ui.button("Add Window").clicked() {
                let _ = self.tx.send(Message::AddWindow);
            }*/
        });
    }
}

impl Plugin for UiControllerPlugin {
    fn name(&self) -> &str {
        "UiControllerPlugin"
    }

    fn update(&mut self, ctx: &egui::Context, _rx: &Receiver<Message>, _tx: &Sender<Message>) {
        // Process messages
        /*while let Ok(message) = self.rx.try_recv() {
            match message {
                Message::AddWindow => {
                    let mut state = self.state.lock().unwrap();
                    state.add_window(Frame::new("New Window"));
                },
                Message::CloseWindow(id) => {
                    let mut state = self.state.lock().unwrap();
                    state.windows.remove(id);
                },
                // Handle other message types as needed
                _ => {}
            }
        }*/

        self.show_top_panel(ctx);
        self.show_central_panel(ctx);
    }
}

impl UIPlugin for UiControllerPlugin {
    fn update_ui(&mut self, ctx: &egui::Context) {
        // This method is intentionally left empty as we're handling UI updates in the main update method
    }
}

