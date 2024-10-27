// plugins/window_management/src/window_controller_messages.rs

use app_core::PluginType;
use app_core::{messages::PluginMessage, Priority};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WindowControllerPluginMessage {
    AddWindow {
        title: String,
        content: String,
        plugin_type: PluginType,
    },
    CloseWindow {
        window_id: usize,
    },
    MinimizeWindow {
        window_id: usize,
    },
    MaximizeWindow {
        window_id: usize,
    },
    DragWindowStart {
        window_id: usize,
        position: (f32, f32),
    },
    DragWindowMove {
        window_id: usize,
        position: (f32, f32),
    },
    DragWindowEnd {
        window_id: usize,
        final_position: (f32, f32),
    },
    WindowAdded {
        window_id: usize,
    },
    WindowClosed {
        window_id: usize,
    },
    UpdateWindowContent {
        window_id: usize,
        new_content: String,
    },
    RequestWindowList,
    WindowListResponse {
        windows: Vec<WindowInfo>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowInfo {
    pub id: usize,
    pub title: String,
    pub plugin_type: PluginType,
    pub is_minimized: bool,
    pub position: (f32, f32),
}

impl WindowControllerPluginMessage {
    pub fn as_plugin_message(&self) -> Option<Box<dyn PluginMessage>> {
        Some(Box::new(self.clone()))
    }
}

impl PluginMessage for WindowControllerPluginMessage {
    fn priority(&self) -> Priority {
        match self {
            WindowControllerPluginMessage::AddWindow { .. } => Priority::Normal,
            WindowControllerPluginMessage::CloseWindow { .. } => Priority::High,
            WindowControllerPluginMessage::MinimizeWindow { .. } => Priority::Low,
            WindowControllerPluginMessage::MaximizeWindow { .. } => Priority::Low,
            WindowControllerPluginMessage::DragWindowStart { .. } => Priority::High,
            WindowControllerPluginMessage::DragWindowMove { .. } => Priority::High,
            WindowControllerPluginMessage::DragWindowEnd { .. } => Priority::High,
            WindowControllerPluginMessage::WindowAdded { .. } => Priority::Normal,
            WindowControllerPluginMessage::WindowClosed { .. } => Priority::Normal,
            WindowControllerPluginMessage::UpdateWindowContent { .. } => Priority::Normal,
            WindowControllerPluginMessage::RequestWindowList => Priority::Low,
            WindowControllerPluginMessage::WindowListResponse { .. } => Priority::Low,
        }
    }

    fn plugin_type(&self) -> PluginType {
        PluginType::WindowController
    }
}
