// plugins/ui_controller/ui_controller_plugin.rs
use std::sync::{Arc, Mutex};
use eframe::egui;
use crossbeam_channel::{unbounded, Receiver, Sender};
use crate::plugins::window_management::{WindowState, FramePlugin, Message};

pub struct UiControllerPlugin {
    state: Arc<Mutex<WindowState>>,
    plugins: Vec<Box<dyn FramePlugin>>,
    receiver: Receiver<Message>,
    tx: Sender<Message>,
}

impl UiControllerPlugin {
    pub fn new(cc: &eframe::CreationContext<'_>, tx: Sender<Message>, state: Arc<Mutex<WindowState>>) -> Self {
        let (_, receiver) = unbounded();
        Self {
            state,
            tx,
            plugins: Vec::new(),
            receiver,
        }
    }

    pub fn add_plugin(&mut self, plugin: Box<dyn FramePlugin>) {
        self.plugins.push(plugin);
    }

    pub fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.show_top_panel(ctx);
        self.show_central_panel(ctx, frame);
    }

    fn show_top_panel(&self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("top_panel")
            .show(ctx, |ui| {
                // Menu bar with file and help buttons
                egui::menu::bar(ui, |ui| {
                    ui.menu_button("File", |ui| {
                        if ui.button("New").clicked() {                
                            let _ = self.tx.send(Message::AddWindow);
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

    fn show_central_panel(&self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // Update all plugins
        for plugin in &self.plugins {
            plugin.update();
        }

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
                        for plugin in &self.plugins {
                            plugin.execute(ui, ctx);
                        }
                    });
            });
    }
}