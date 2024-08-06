use serde::{Serialize, Deserialize};
use egui::Pos2;

pub struct AboutWindowState {
    pub windows: Vec<WindowInfo>,
    pub grid: Vec<usize>,
    pub about_counter: usize,
    pub expanded_height: f32,
    pub collapsed_height: f32,
    pub gap_height: f32,
    pub dragged_window: Option<DraggedWindow>,
    pub sender: crossbeam_channel::Sender<Message>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct WindowInfo {
    pub id: usize,
    pub title: String,
    pub content: String,
    pub size: (f32, f32),  // Changed from egui::Vec2 to (f32, f32
    pub collapsed: bool,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct DraggedWindow {
    pub index: usize,
  //  #[serde(with = "pos2_serde")]
    pub start_pos:  (f32, f32),
 //   #[serde(with = "pos2_serde")]
    pub current_pos:  (f32, f32),
}

mod pos2_serde {
    use serde::{Serializer, Deserializer, Serialize, Deserialize};
    use egui::Pos2;

    pub fn serialize<S>(pos: &Pos2, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        (pos.x, pos.y).serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Pos2, D::Error>
    where
        D: Deserializer<'de>,
    {
        let (x, y) = <(f32, f32)>::deserialize(deserializer)?;
        Ok(Pos2::new(x, y))
    }
}
#[derive(Clone, Serialize, Deserialize)]
pub enum Message {
    AddWindow,
    CollapseWindow(usize),
    DragWindowStart(usize, (f32, f32)),
    DragWindowMove((f32, f32)),
    DragWindowEnd,
    CloseWindow(usize)
}
