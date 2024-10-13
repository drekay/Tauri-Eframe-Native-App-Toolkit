# Tauri Eframe Native App Toolkit

A toolkit for creating native applications using Tauri and Eframe, with a focus on producing cross-platform desktop applications with a great native experience. This toolkit now uses a Plugin Architecture for improved modularity and extensibility.

**NB: This is still EXPERIMENTAL and UNDER DEVELOPMENT!!**

## Dependencies

| Component | Description | Version |
|-----------|-------------|---------|
| tauri     | runtime core | [latest] |
| egui      | immediate mode GUI library for Rust | [latest] |

## Motivation

The goal of this toolkit is to enable developers to produce cross-platform desktop applications that offer a seamless and native user experience. By leveraging the power of Tauri and Eframe, developers can build applications that are both lightweight and performant. The new Plugin Architecture allows for more flexible and maintainable code.

## Plugin Architecture

The toolkit now uses a Plugin Architecture, which includes:

- `UiControllerPlugin`: Manages the overall UI and coordinates between plugins.
- `WindowPlugin`: Handles specific tasks related to windows (like showing and removing them).
- `WindowControllerPlugin`: Manages high-level tasks like verifying window closures and informing associated plugins.

## Example: How to Create an Application with Draggable Windows

Here's a simplified example of how to create a basic application with draggable windows using Tauri, Eframe, and our Plugin Architecture:

```rust
//main.rs
use eframe::{egui, CreationContext, NativeOptions, App};
use plugins::window_management::{WindowPlugin, WindowState, WindowControllerPlugin};
use std::sync::{Arc, Mutex};
use crossbeam_channel::{unbounded, Sender};

mod plugins;
use crate::plugins::ui_controller::UiControllerPlugin;

struct TauriEframeNativeApp {
    ui_controller: UiControllerPlugin,
}

impl TauriEframeNativeApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let (tx, rx) = crossbeam_channel::unbounded();
        let state = Arc::new(Mutex::new(WindowState::new(tx.clone())));
        
        let (tx, rx) = crossbeam_channel::unbounded();
        let mut ui_controller = UiControllerPlugin::new(cc, tx.clone(), state.clone());
        
        let window_plugin = WindowPlugin::new(tx.clone(),rx.clone());
        ui_controller.add_plugin(Box::new(window_plugin));
        
        let window_management_plugin = WindowControllerPlugin::new(tx.clone(),rx.clone());
        ui_controller.add_plugin(Box::new(window_management_plugin));
        
        // Add other plugins as needed
        // let new_plugin = NewPlugin::new(tx.clone(),rx.clone());
        // ui_controller.add_plugin(Box::new(new_plugin));

        Self {
            ui_controller,
            // initialize other fields if needed...
        }
    }
}

impl App for TauriEframeNativeApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.ui_controller.update(ctx, frame);
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = NativeOptions::default();

    eframe::run_native(
        "window_management Demo",
        options,
        Box::new(|cc| Ok(Box::new(TauriEframeNativeApp::new(cc))))
    )
}