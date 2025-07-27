use crate::tools::tool::{Tool, TOOL_REGISTRY};
use std::collections::HashMap;

use super::tool::ToolMetadata;

pub struct ToolRegistry {
    tools: HashMap<&'static str, Box<dyn Tool>>,
}

impl Default for ToolRegistry {
    fn default() -> Self {
        let tools = HashMap::new();
        Self { tools }
    }
}

impl ToolRegistry {
    pub fn available_tool_names() -> Vec<&'static str> {
        let reg = TOOL_REGISTRY.lock().unwrap();
        reg.keys().copied().collect()
    }

    pub fn available_tool_metadata() -> Vec<ToolMetadata> {
        let reg = TOOL_REGISTRY.lock().unwrap();
        reg.values()
            .map(|&tool| tool().metadata())
            .collect::<Vec<_>>()
    }

    pub fn get_tool_mut(&mut self, name: &'static str) -> &mut Box<dyn Tool> {
        self.tools.entry(name).or_insert_with(|| {
            let reg = TOOL_REGISTRY.lock().unwrap();
            let factory = reg
                .get(name)
                .unwrap_or_else(|| panic!("No tool registered with name: {}", name));
            factory()
        })
    }
}
