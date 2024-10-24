//app_core/src/lib.rs
pub mod ui;
pub mod plugin_system;
pub mod message_bus;
pub mod message_priority;
pub mod message_handler;
pub mod messages;
pub mod priority_message_handler;

use crossbeam_channel:: {Sender, Receiver};
use eframe::egui;
use messages::Message;
use std::cmp::Ordering;

pub trait Versioned {
    fn get_version(&self) -> &PluginVersion;
}

pub trait VersionComparable: Versioned {
    fn compare_version(&self, other: &Self) -> Ordering {
        self.get_version().compare(other.get_version())
    }
}

pub trait VersionEquatable: Versioned {
    fn version_eq(&self, other: &Self) -> bool {
        self.get_version().eq(other.get_version())
    }
}

#[derive(Clone, Debug)]
pub struct PluginVersion {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

impl PluginVersion {
    pub fn new(major: u32, minor: u32, patch: u32) -> Self {
        Self { major, minor, patch }
    }

    pub fn compare(&self, other: &Self) -> Ordering {
        self.major.cmp(&other.major)
            .then(self.minor.cmp(&other.minor))
            .then(self.patch.cmp(&other.patch))
    }

    pub fn eq(&self, other: &Self) -> bool {
        self.major == other.major && self.minor == other.minor && self.patch == other.patch
    }
}

// Macro for creating PluginVersion instances
#[macro_export]
macro_rules! plugin_version {
    ($major:expr, $minor:expr, $patch:expr) => {
        PluginVersion::new($major, $minor, $patch)
    };
}
pub enum PluginType {
    UI,
    Audio,
    Data,
    // Add other types as needed
}

// Update the Plugin trait to use Versioned
pub trait Plugin: Send + Sync {
    fn name(&self) -> &str;
    fn plugin_type(&self) -> PluginType;
    fn controller(&self) -> Option<&str>;
    fn is_enabled(&self) -> bool;
    fn set_enabled(&mut self, enabled: bool);

    fn update(&mut self, ctx: &egui::Context, message_handler: &mut dyn MessageHandler);
    fn handle_message(&mut self, message: Message, message_handler: &mut dyn MessageHandler);
    fn on_load(&mut self);
    fn on_unload(&mut self);
}
///

pub trait UIPlugin: Plugin {
    fn update_ui(&mut self, ctx: &egui::Context);
  }

  /* for non-blocking apporach only
pub trait MessageFilter: Send + Sync {
    fn filter(&self, message: &Message) -> bool;
}*/


pub trait MessageFilter: Send + Sync {
    fn filter(&self, message: &Message) -> bool;
    fn clone_box(&self) -> Box<dyn MessageFilter + Send + Sync>;
}

impl Clone for Box<dyn MessageFilter + Send + Sync> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

pub use plugin_system::PluginSystem;
pub use message_handler::MessageHandler;
pub use message_priority::Priority;
pub use priority_message_handler::PrioritizedMessage;
pub use ui::Window;
pub use ui::WindowResponse;
