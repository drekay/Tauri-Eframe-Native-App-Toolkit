
use std::fmt;
use std::any::Any;
use std::sync::Arc;
use std::collections::HashMap;
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub enum Message {
    Broadcast {
        content: String,
        priority: Priority,
    },
    ControllerMessage {
        target_controller_id: String,
        content: ControllerMessage,
        priority: Priority,
    },
    Plugin {
        plugin_type: PluginType,
        message:  Arc<dyn Any + Send + Sync>,
        priority: Priority,
    },
    PluginSpecific {
        content: Arc<dyn PluginMessage>,
        priority: Priority,
    },
    Whatever {
        content: Arc<dyn Any + Send + Sync>,
        priority: Priority,
    },
    ControllerUpdated {
        controller_id: String,
        priority: Priority,
    },
    CriticalData {
        payload: Arc<dyn Any + Send + Sync>,
        priority: Priority,
    },
    FilteredOut {
        original_message: Box<Message>,
        priority: Priority,
    },
}

#[derive(Debug, Clone)]
pub enum ControllerMessage {
    WindowController(WindowControllerMessage),
    // Other controller types can be added here
}

#[derive(Debug, Clone)]
pub enum WindowControllerMessage {
    RequestCloseWindow {
        window_index: usize,
        priority: Priority,
    },
    RequestAddWindow {
        priority: Priority,
    },
    // ... other window controller specific messages ...
}

impl Message {
    pub fn get_target_controller_id(&self) -> Option<&str> {
        match self {
            Message::Broadcast { .. } => None,
            Message::ControllerMessage { target_controller_id, .. } => Some(target_controller_id),
            Message::PluginSpecific { .. } => None,
            Message::Whatever { .. } => None,
            Message::ControllerUpdated { .. } => None,
            Message::Plugin { plugin_type, message, priority } => todo!(),
            Message::CriticalData { payload, priority } => todo!(),
            Message::FilteredOut { original_message, priority } => todo!(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PluginManifest {
    pub name: String,
    pub version: String,
    pub controller_id: String,
    pub dependencies: Vec<String>,
    pub config_options: HashMap<String, String>,
}

/*#[derive(Debug, Clone)]
pub enum Message {
    Broadcast(String),
    WindowPlugin(WindowPluginMessage),
    WindowControllerPlugin(WindowControllerPluginMessage),
    Sample(SampleMessage),
    Menu(MenuMessage),
    System(SystemMessage),
    PluginControl(PluginControlMessage), 
    ControllerPlugin(ControllerPluginMessage),
    CriticalData(CriticalDataPayload),
}*/

/* 
#[derive(Debug, Clone)]
pub enum Message {
    Broadcast {
        content: String,
        priority: Priority,
    },
    WindowPlugin {
        message: WindowPluginMessage,
        priority: Priority,
    },
    WindowControllerPlugin {
        message: WindowControllerPluginMessage,
        priority: Priority,
    },
    Sample {
        message: SampleMessage,
        priority: Priority,
    },
    Menu {
        message: MenuMessage,
        priority: Priority,
    },
    System {
        message: SystemMessage,
        priority: Priority,
    },
    PluginControl {
        message: PluginControlMessage,
        priority: Priority,
    },
    ControllerPlugin {
        message: ControllerPluginMessage,
        priority: Priority,
    },
    CriticalData(CriticalDataPayload), // Assuming CriticalData always has high priority
}*/

#[derive(Debug, Clone)]
pub enum WindowPluginMessage {
    AddWindow,
    ConfirmedCloseWindow(usize),
    MinimizeWindow(usize),
    DragWindowStart(usize, egui::Pos2),
    DragWindowMove(egui::Pos2),
    DragWindowEnd,
    CollapseWindow(usize),
}

/* deleted
#[derive(Debug, Clone)]
pub enum WindowControllerPluginMessage {
    CloseWindow(usize),
    // Add other window controller messages
}*/

//oher system messages
#[derive(Clone, Debug)]
pub enum SampleMessage {
    Start { id: u32, data: Vec<u8> },
    Stop { id: u32 },
    Update { id: u32, data: Vec<u8> },
    // Add other sample-related message types
}

#[derive(Clone, Debug)]
pub enum MenuMessage {
    FileOpen,
    FileSave,
    FileNew,
    FileClose,
    EditCopy,
    EditPaste,
    EditCut,
    ViewZoomIn,
    ViewZoomOut,
    HelpAbout,
    // Add more menu options as needed
}

#[derive(Debug, Clone)]
pub enum SystemMessage {
    ShutdownPlugin(String),
    ReloadPlugin(String),
    LogMessage(LogLevel, String),
    PerformanceMetric(String, f64),
    Update,
}

#[derive(Debug, Clone)]
pub enum LogLevel {
    Debug,
    Info,
    Warning,
    Error,
}

impl fmt::Display for SystemMessage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SystemMessage::ShutdownPlugin(plugin) => write!(f, "Shutdown plugin: {}", plugin),
            SystemMessage::ReloadPlugin(plugin) => write!(f, "Reload plugin: {}", plugin),
            SystemMessage::LogMessage(level, msg) => write!(f, "[{}] {}", level, msg),
            SystemMessage::PerformanceMetric(metric, value) => write!(f, "Performance metric - {}: {}", metric, value),
            SystemMessage::Update => write!(f, "System update"),
        }
    }
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LogLevel::Debug => write!(f, "DEBUG"),
            LogLevel::Info => write!(f, "INFO"),
            LogLevel::Warning => write!(f, "WARNING"),
            LogLevel::Error => write!(f, "ERROR"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum PluginControlMessage {
    Enable(String),
    Disable(String),
    Configure(String, ConfigData),
}
#[derive(Debug, Clone)]
pub struct ConfigData {
    // This would contain configuration parameters
    // The exact fields would depend on your plugin system's needs
}

/** UI Messages **/
#[derive(Debug, Clone)]
pub enum ControllerPlugin {
    WindowController,
    MenuController,
    FileController,
    // Add other controller types as needed
}

#[derive(Debug, Clone)]
pub enum ControllerPluginMessage {
    NewFile,
    OpenFile,
    SaveFile,
    Exit,
    WindowClicked(usize),
    WindowDragged(usize, egui::Pos2),
    WindowDragEnded(usize),
    WindowAdded,
    WindowClosed(usize),
    WindowMinimized(usize),
    // Add other controller-specific messages as needed
}

/** **/
pub enum ComponentInteraction {
    ButtonClicked(String),
    CheckboxChanged(String, bool),
    SliderChanged(String, f32),
    TextChanged(String, String),
    // ... other component interactions
}

/** **/
#[derive(Debug, Clone)]
pub struct CriticalDataPayload {
    // Define fields as needed
}
///////////////////////////////////////
pub trait PluginMessage: Any + Send + Sync + Debug {
    fn priority(&self) -> Priority;
    fn plugin_type(&self) -> PluginType;
    fn as_any(&self) -> &dyn Any;

}

impl PluginMessage for WindowPluginMessage {
    fn priority(&self) -> Priority {
        self.priority()
    }

    fn plugin_type(&self) -> PluginType {
        PluginType::Window
    }

    fn as_any(&self) -> &dyn Any where Self: Sized {
        self
    }
}

// Implement for other message types (AudioPluginMessage, etc.)

use std::cmp::{Ord, PartialOrd, Ordering};

use serde::{Deserialize, Serialize};

use crate::{MessageFilter, PluginType, Priority};

struct PrioritizedMessage(Box<dyn PluginMessage>);

impl PartialEq for PrioritizedMessage {
    fn eq(&self, other: &Self) -> bool {
        self.0.priority() == other.0.priority()
    }
}

impl Eq for PrioritizedMessage {}

impl PartialOrd for PrioritizedMessage {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.0.priority().cmp(&other.0.priority()))
    }
}

impl Ord for PrioritizedMessage {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.priority().cmp(&other.0.priority())
    }
}