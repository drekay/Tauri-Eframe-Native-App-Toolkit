//app_core/src/ui.rs
use std::any::Any;

pub trait Window {
    fn new(title: &str) -> Self where Self: Sized;
    fn show(&mut self, context: &mut dyn Any) -> WindowResponse;
    fn title(&self) -> &str;
    fn set_title(&mut self, title: &str);
    fn is_minimized(&self) -> bool;
    fn set_minimized(&mut self, minimized: bool);
}

pub struct WindowResponse {
    pub is_closed: bool,
    pub is_minimized: bool,
    pub drag_delta: (f32, f32),
}