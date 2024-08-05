use crossbeam_channel::Sender;
use serde::{Serialize, Deserialize};

pub struct AboutWindowState {
    pub windows: Vec<WindowInfo>,
    pub about_counter: usize,
    pub expanded_height: f32,
    pub collapsed_height: f32,
    pub gap_height: f32,
    pub dragged_window: Option<DraggedWindow>,
    pub sender: Sender<Message>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct WindowInfo {
    pub id: usize,
    pub title: String,
    pub content: String,
    pub size: (f32, f32),
    pub collapsed: bool,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct DraggedWindow {
    pub index: usize,
    pub start_pos: (f32, f32),
    pub current_pos: (f32, f32),
}

#[derive(Clone, Serialize, Deserialize)]
pub enum Message {
    AddWindow,
    CollapseWindow(usize),
    DragWindowStart(usize, (f32, f32)),
    DragWindowMove((f32, f32)),
    DragWindowEnd,
}
