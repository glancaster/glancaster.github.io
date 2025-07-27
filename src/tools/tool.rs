use std::{collections::HashMap, sync::Mutex};

use eframe::egui::Ui;
use once_cell::sync::Lazy;

pub trait Tool {
    fn metadata(&self) -> ToolMetadata;
    fn ui(&mut self, ui: &mut Ui);
}

pub struct ToolMetadata {
    pub name: &'static str,
    pub desc: &'static str,
    pub icon: &'static str,
}

// Type alias for factory function
type ToolFactory = fn() -> Box<dyn Tool>;

// Static global registry for tool factories
pub static TOOL_REGISTRY: Lazy<Mutex<HashMap<&'static str, ToolFactory>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

// Macro to register a tool
#[macro_export]
macro_rules! register_tool {
    ($tool_type:ty, $name:expr) => {{
        fn create() -> Box<dyn $crate::tools::tool::Tool> {
            Box::new(<$tool_type>::new())
        }

        $crate::tools::tool::TOOL_REGISTRY
            .lock()
            .unwrap()
            .insert($name, create);
    }};
}
