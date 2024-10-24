use egui::{Context, Ui};
use app_core::messages::{ControllerPluginMessage, MenuMessage, Message, WindowPluginMessage};
use app_core::{plugin_version, MessageHandler, Plugin, UIPlugin, PluginType, PluginVersion, VersionComparable, VersionEquatable, Versioned, WindowResponse};
use egui_impl::EguiWindow as Window;
use crossbeam_channel::{Sender, Receiver};
use std::sync::{Arc, Mutex};

pub struct UIViewPlugin {
    shared_state: Arc<Mutex<WindowState>>,
    tx: Sender<Message>,
    rx: Receiver<Message>,
}

impl UIViewPlugin {
    pub fn new(shared_state: Arc<Mutex<WindowState>>, tx: Sender<Message>, rx: Receiver<Message>) -> Self {
        Self {
            shared_state,
            tx,
            rx,
        }
    }

    fn render_top_menu(&self, ui: &mut Ui) {
        egui::menu::bar(ui, |ui| {
            ui.menu_button("File", |ui| {
                if ui.button("New").clicked() {
                    self.tx.send(Message::ControllerPlugin(ControllerPluginMessage::NewFile)).unwrap();
                }
                if ui.button("Open").clicked() {
                    self.tx.send(Message::ControllerPlugin(ControllerPluginMessage::OpenFile)).unwrap();
                }
                if ui.button("Save").clicked() {
                    self.tx.send(Message::ControllerPlugin(ControllerPluginMessage::SaveFile)).unwrap();
                }
                if ui.button("Exit").clicked() {
                    self.tx.send(Message::ControllerPlugin(ControllerPluginMessage::Exit)).unwrap();
                }
            });
            // Add more menu items as needed
        });
    }

    fn render_windows(&self, ui: &mut Ui) {
        let mut state = self.shared_state.lock().unwrap();
        for (index, window) in state.windows.iter_mut().enumerate() {
            let response = window.show(ui);
            self.handle_window_interaction(response, index);
        }
    }

    fn handle_window_interaction(&self, response: EguiWindowResponse, window_index: usize) {
        if response.is_closed {
            self.tx.send(Message::ControllerPlugin(ControllerPluginMessage::WindowClosed(window_index))).unwrap();
        }
        if response.is_minimized {
            self.tx.send(Message::ControllerPlugin(ControllerPluginMessage::WindowMinimized(window_index))).unwrap();
        }
        if response.is_dragged {
            if let Some(delta) = response.drag_delta {
                self.tx.send(Message::ControllerPlugin(ControllerPluginMessage::WindowDragged(window_index, delta))).unwrap();
            }
        }
        for interaction in response.component_interactions {
            match interaction {
                ComponentInteraction::ButtonClicked(button_id) => {
                    self.tx.send(Message::ControllerPlugin(ControllerPluginMessage::ButtonClicked(window_index, button_id))).unwrap();
                },
                ComponentInteraction::CheckboxChanged(checkbox_id, value) => {
                    self.tx.send(Message::ControllerPlugin(ControllerPluginMessage::CheckboxChanged(window_index, checkbox_id, value))).unwrap();
                },
                ComponentInteraction::SliderChanged(slider_id, value) => {
                    self.tx.send(Message::ControllerPlugin(ControllerPluginMessage::SliderChanged(window_index, slider_id, value))).unwrap();
                },
                ComponentInteraction::TextChanged(text_id, value) => {
                    self.tx.send(Message::ControllerPlugin(ControllerPluginMessage::TextChanged(window_index, text_id, value))).unwrap();
                },
                // Handle other interactions as needed
            }
        }
    }
}

impl Plugin for UIViewPlugin {
    fn name(&self) -> &str {
        "UIViewPlugin"
    }

    fn update(&mut self, ctx: &Context, _rx: &Receiver<Message>, _tx: &Sender<Message>) {
        // Process incoming messages that are relevant to the view
        while let Ok(message) = self.rx.try_recv() {
            match message {
                // Handle view-specific messages if any
                _ => {} // Ignore messages not relevant to the view
            }
        }
    }

    fn on_load(&mut self) {
        // Initialization logic
    }

    fn on_unload(&mut self) {
        // Cleanup logic
    }
    
    fn plugin_type(&self) -> PluginType {
        todo!()
    }
    
    fn controller(&self) -> Option<&str> {
        todo!()
    }
    
    fn is_enabled(&self) -> bool {
        todo!()
    }
    
    fn set_enabled(&mut self, enabled: bool) {
        todo!()
    }
    
    fn handle_message(&mut self, message: Message, message_handler: &mut dyn MessageHandler) {
        todo!()
    }
}

impl UIPlugin for UIViewPlugin {
    fn update_ui(&mut self, ctx: &Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.render_top_menu(ui);
            egui::ScrollArea::vertical().show(ui, |ui| {
                self.render_windows(ui);
            });
        });
    }
}