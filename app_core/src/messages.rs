use std::fmt;

use crate::Priority;

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
}

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

#[derive(Debug, Clone)]
pub enum WindowControllerPluginMessage {
    CloseWindow(usize),
    // Add other window controller messages
}

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