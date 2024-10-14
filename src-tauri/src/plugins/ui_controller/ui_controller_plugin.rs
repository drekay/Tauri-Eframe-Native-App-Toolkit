// plugins/ui_controller/ui_controller_plugin.rs
use std::sync::{Arc, Mutex};
use eframe::egui;
use crossbeam_channel::{unbounded, Receiver, Sender};
use crate::plugins::window_management::{FramePlugin, Message, MessageBus, WindowState};

pub struct UiControllerPlugin {
    state: Arc<Mutex<WindowState>>,
    message_bus: MessageBus,
    plugins: Vec<Box<dyn FramePlugin>>,
    receiver: Receiver<Message>,
    tx: Sender<Message>,
}

impl UiControllerPlugin {
    pub fn new(cc: &eframe::CreationContext<'_>, tx: Sender<Message>, state: Arc<Mutex<WindowState>>) -> Self {
        let (_, receiver) = unbounded();
        Self {
            message_bus: MessageBus::new(),
            state,
            tx,
            plugins: Vec::new(),
            receiver,
        }
    }

    pub fn add_plugin(&mut self, mut plugin: Box<dyn FramePlugin>) {
        let rx = self.message_bus.register_plugin(plugin.name());
        plugin.set_receiver(rx);
        self.plugins.push(plugin);
    }

    pub fn update(&mut self, ctx: &egui::Context) {
        self.show_top_panel(ctx);

        // Process messages    
        while let Ok(message) = self.message_bus.receiver.try_recv() {
            println!("UiController received message: {:?}", message);
            self.handle_message(message);
        }

        // Update plugins
        for plugin in &mut self.plugins {
            plugin.update();
        }

        self.show_central_panel(ctx);
    }

    fn handle_message(&mut self, message: Message) {
        println!("UiController handling message: {:?}", message);
        // Broadcast message to all plugins
        for plugin in &mut self.plugins {
            plugin.handle_message(message.clone());
        }
    }

    fn show_top_panel(&self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("top_panel")
            .show(ctx, |ui| {
                // Menu bar with file and help buttons
                egui::menu::bar(ui, |ui| {
                    ui.menu_button("File", |ui| {
                        if ui.button("New").clicked() {                
                            let _ = self.message_bus.broadcast(Message::AddWindow);
                            println!("Add window button pressed");
                        }
                        if ui.button("Open").clicked() { println!("Open file"); }
                        if ui.button("Save").clicked() { println!("Save file"); }
                        if ui.button("Close").clicked() {
                            let _ = self.tx.send(Message::CloseWindow(1));//todo: pass window_id
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
                        if ui.button("About").clicked() { println!("About");}
                    });
                });
            });
    }

      fn show_central_panel(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default()
            .show(ctx, |ui| {
                ui.heading("Tauri Eframe Native App Toolkit");
                ui.add_space(4.0);
                ui.label("Click 'File' -> 'New' to open new About windows.");
                ui.label("Drag windows vertically to reorder them.");
                ui.label("Scroll up and down if you added many windows.");

                egui::ScrollArea::vertical()
                    .show(ui, |ui| {
                        // Execute all plugins
                        for plugin in &mut self.plugins {
                            plugin.execute(ui, ctx);
                        }
                    });
            });
    }
}