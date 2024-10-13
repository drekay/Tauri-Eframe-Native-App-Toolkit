# Tauri Eframe Native App Toolkit

A toolkit for creating native applications using Tauri and Eframe, with a focus on producing cross-platform desktop applications with a great native experience.

# NB: This is still EXPERIMENTAL and UNDER DEVELOPMENT!!

## Dependencies

| Component | Description | Version |
|-----------|-------------|---------|
| tauri | runtime core | [![](https://img.shields.io/crates/v/tauri.svg)](https://crates.io/crates/tauri) |
| egui | immediate mode GUI library for Rust | [![](https://img.shields.io/crates/v/egui.svg)](https://crates.io/crates/egui) |

## Motivation

The goal of this toolkit is to enable developers to produce cross-platform desktop applications that offer a seamless and native user experience. By leveraging the power of Tauri and Eframe, developers can build applications that are both lightweight and performant.

## Example: How to Create an Application with Draggable Windows

Here's a simplified example of how to create a basic application with draggable windows using Tauri and Eframe. For full implementation details, refer to the `main.rs` source file.

### Steps:

1. **Initialize the Project**:
    - Create a new Rust project.
    - Add dependencies for `tauri` and `eframe` in your `Cargo.toml`.

2. **Define the Application State**:
    - Create a struct to hold the application state, including window information and dragging state.

3. **Implement the Application Logic**:
    - Define methods for adding windows and calculating their positions.
    - Implement the `eframe::App` trait for your application state.

4. **Handle Dragging Logic**:
    - Track the dragged window and update its position in real-time.
    - Reorder windows based on the final position of the dragged window.

5. **Run the Application**:
    - Use `eframe::run_native` to run your application.

### Code Example:

```rust
struct MyEguiApp {
    windows: Vec<WindowInfo>,
    dragged_window: Option<DraggedWindow>,
    grid: Vec<usize>,
    // other fields...
}

impl MyEguiApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // initialization code...
    }

    fn add_about_window(&mut self) {
        // code to add a new window...
    }

    fn get_window_pos(&self, grid_index: usize) -> egui::Pos2 {
        // code to calculate window position...
    }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // code to handle UI layout, dragging, and reordering...
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let options = NativeOptions::default();
    eframe::run_native(
        "Tauri EGUI Demo",
        options,
        Box::new(|cc| Ok(Box::new(MyEguiApp::new(cc)))),
    )?;
    Ok(())
}
```

For the full implementation, please refer to the main.rs source file.

## License
This project is licensed under the MIT License.