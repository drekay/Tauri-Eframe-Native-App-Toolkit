// egui_impl/src/components/window.rs

use eframe::egui;
use crate::{EguiWindowTrait, EguiWindowResponse};
use app_core::ui::{Window as CoreWindow, WindowResponse as CoreWindowResponse};
use super::frame::Frame;

pub struct Window {
    frame: Frame,
    id: usize,
    position: egui::Pos2,
    size: egui::Vec2,
    is_open: bool,
}

impl Window {
    pub fn new(id: usize, title: &str, position: egui::Pos2, size: egui::Vec2) -> Self {
        Self {
            frame: Frame::new(title),
            id,
            position,
            size,
            is_open: true,
        }
    }

    pub fn set_content(&mut self, content: String) {
        self.frame.content = content;
    }

    pub fn is_open(&self) -> bool {
        self.is_open
    }

    pub fn close(&mut self) {
        self.is_open = false;
    }

    pub fn open(&mut self) {
        self.is_open = true;
    }

    pub fn toggle(&mut self) {
        self.is_open = !self.is_open;
    }
}

impl CoreWindow for Window {
    fn new(title: &str) -> Self where Self: Sized {
        Self::new(0, title, egui::Pos2::ZERO, egui::Vec2::new(300.0, 200.0))
    }
    
    fn show(&mut self, context: &mut dyn std::any::Any) -> CoreWindowResponse {
        let ctx = context.downcast_ref::<egui::Context>().expect("Expected egui::Context");
        
        let mut egui_response = EguiWindowResponse {
            core: CoreWindowResponse {
                is_closed: false,
                is_minimized: false,
                drag_delta: (0.0, 0.0),
            },
            is_closed: false,
            drag_started: false,
            drag_released: false,
            drag_delta: egui::Vec2::ZERO,
        };
    
        egui::Area::new(egui::Id::new(self.id))
            .movable(true)
            .default_pos(self.position)
            .show(ctx, |ui| {
                // Use the show_egui method to render the window contents
                egui_response = self.show_egui(ui, self.id);
                
                //todo: Do we need to track this here?
                // Update the window position based on the area's new position
                /*if let Some(new_pos) = ui.min_rect().min {
                    self.position = new_pos;
                }*/
            });
    
        // Return the CoreWindowResponse
        egui_response.core
    }
    
     fn title(&self) -> &str {
        &self.frame.title
    }

    fn set_title(&mut self, title: &str) {
        self.frame.title = title.to_string();
    }

    fn is_minimized(&self) -> bool {
        self.frame.is_minimized
    }

    fn set_minimized(&mut self, minimized: bool) {
        self.frame.is_minimized = minimized;
    }
}

impl EguiWindowTrait for Window {
    fn show_egui(&mut self, ui: &mut egui::Ui, index: usize) -> EguiWindowResponse {
        if !self.is_open {
            return EguiWindowResponse {
                core: CoreWindowResponse {
                    is_closed: true,
                    is_minimized: false,
                    drag_delta: (0.0, 0.0),
                },
                is_closed: true,
                drag_started: false,
                drag_released: false,
                drag_delta: egui::Vec2::ZERO,
            };
        }

        let window = egui::Window::new(&self.frame.title)
            .id(egui::Id::new(self.id))
            .default_pos(self.position)
            .default_size(self.size)
            .resizable(true)
            .collapsible(true);

        let mut is_closed = false;
        let mut drag_started = false;
        let mut drag_released = false;
        let mut drag_delta = egui::Vec2::ZERO;

        let response = window.show(ui.ctx(), |ui| {
            let (frame_response, frame_is_closed, frame_drag_started, frame_drag_released, _, frame_drag_delta) = 
                self.frame.show(ui, self.id);
            
            is_closed = frame_is_closed;
            drag_started = frame_drag_started;
            drag_released = frame_drag_released;
            drag_delta = frame_drag_delta;

            frame_response
        });

        if let Some(inner_response) = response {
            self.position = inner_response.response.rect.min;
            self.size = inner_response.response.rect.size();

            self.is_open = !is_closed;

            EguiWindowResponse {
                core: CoreWindowResponse {
                    is_closed,
                    is_minimized: self.frame.is_minimized,
                    drag_delta: (drag_delta.x, drag_delta.y),
                },
                is_closed,
                drag_started,
                drag_released,
                drag_delta,
            }
        } else {
            EguiWindowResponse {
                core: CoreWindowResponse {
                    is_closed: true,
                    is_minimized: false,
                    drag_delta: (0.0, 0.0),
                },
                is_closed: true,
                drag_started: false,
                drag_released: false,
                drag_delta: egui::Vec2::ZERO,
            }
        }
    }
}