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
- `AboutWindowPlugin`: Handles specific tasks related to windows (like showing and removing them).
- `WindowControllerPlugin`: Manages high-level tasks like verifying window closures and informing associated plugins.

## Example: How to Create an Application with Draggable Windows

Here's a simplified example of how to create a basic application with draggable windows using Tauri, Eframe, and our Plugin Architecture:

```rust
use eframe::{egui, NativeOptions};
use your_crate::{UiControllerPlugin, AboutWindowPlugin, WindowControllerPlugin};

struct TauriEframeNativeApp {
    ui_controller: UiControllerPlugin,
}

impl TauriEframeNativeApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let (tx, rx) = crossbeam_channel::unbounded();
        let about_window_plugin = AboutWindowPlugin::new(tx.clone(), rx.clone());
        let window_controller_plugin = WindowControllerPlugin::new(tx.clone(), rx);

        let ui_controller = UiControllerPlugin::new(vec![
            Box::new(about_window_plugin),
            Box::new(window_controller_plugin),
        ]);

        Self { ui_controller }
    }
}

impl eframe::App for TauriEframeNativeApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.ui_controller.update();
        
        egui::CentralPanel::default().show(ctx, |ui| {
            self.ui_controller.execute(ui, ctx);
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = NativeOptions::default();
    eframe::run_native(
        "Tauri Eframe Native Toolkit Demo",
        options,
        Box::new(|cc| Ok(Box::new(TauriEframeNativeApp::new(cc)))),
    )
}