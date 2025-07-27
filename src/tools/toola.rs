use crate::tools::tool::Tool;
use eframe::egui::Ui;

use super::tool::ToolMetadata;

const NAME: &'static str = "Tool A";
pub struct ToolA {
    counter: usize,
}

impl ToolA {
    pub fn new() -> Self {
        Self { counter: 0 }
    }
}

impl Tool for ToolA {
    fn metadata(&self) -> ToolMetadata {
        ToolMetadata {
            name: NAME,
            desc: "Example Tool Structure",
            icon: "🔧",
        }
    }

    fn ui(&mut self, ui: &mut Ui) {
        if ui.button("Increment").clicked() {
            self.counter += 1;
        }
        ui.label(format!("Count: {}", self.counter));
    }
}
pub fn register() {
    crate::register_tool!(ToolA, NAME);
}
