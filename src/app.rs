use crate::about;
use crate::tools::{register_all_tools, tool_registry::ToolRegistry};
use crate::utils::flexbutton::FlexButton;
use eframe::egui::{self, Layout};
use egui_flex::{Flex, FlexItem};
use enum_iterator::{all, Sequence};

#[derive(Default, PartialEq, Sequence, Debug, Clone)]
enum View {
    #[default]
    Home,
    About,
    Tools,
}
#[derive(Default)]
pub struct Website {
    selected_view: View,
    tool_registry: ToolRegistry,
    selected_tool: Option<&'static str>,
}

impl Website {
    pub fn new() -> Self {
        register_all_tools();
        Self::default()
    }
}

impl eframe::App for Website {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("menu").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.with_layout(
                    Layout::centered_and_justified(egui::Direction::LeftToRight),
                    |ui| {
                        ui.horizontal(|ui| {
                            for view in all::<View>().collect::<Vec<_>>() {
                                let display = format!("{:?}", &view);
                                if ui
                                    .selectable_value(
                                        &mut self.selected_view,
                                        view.clone(),
                                        display,
                                    )
                                    .clicked()
                                {
                                    if view == View::Tools {
                                        self.selected_tool = None;
                                    }
                                };
                            }
                        });
                    },
                );
                ui.with_layout(
                    Layout::centered_and_justified(egui::Direction::RightToLeft),
                    |ui| {
                        ui.horizontal(|ui| {
                            ui.hyperlink_to("\u{E624}", "https://github.com/glancaster");
                            ui.hyperlink_to("\u{E608}", "https://www.linkedin.com/in/grlanca/");
                        });
                    },
                );
            });
        });
        match self.selected_view {
            View::Home => {}
            View::About => {}
            View::Tools => {
                egui::SidePanel::left("side_panel_tools").show(ctx, |ui| {
                    for tool_name in ToolRegistry::available_tool_names() {
                        ui.selectable_value(&mut self.selected_tool, Some(tool_name), tool_name);
                    }
                });
            }
        }
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| match self.selected_view {
                View::Home => {}
                View::About => {
                    about::ui(ui);
                }
                View::Tools => {
                    if let Some(name) = &self.selected_tool {
                        let tool = self.tool_registry.get_tool_mut(name);
                        tool.ui(ui);
                    } else {
                        ui.heading("Re-creating tools to learn how they work");
                        ui.add_space(20.0);
                        Flex::new()
                            .w_full()
                            .align_items(egui_flex::FlexAlign::Stretch)
                            .align_items_content(egui::Align2::CENTER_CENTER)
                            .wrap(true)
                            .show(ui, |flex| {
                                for metadata in ToolRegistry::available_tool_metadata() {
                                    if flex
                                        .add(
                                            FlexItem::default().grow(1.0).min_height(150.0),
                                            FlexButton::new(
                                                metadata.name,
                                                metadata.desc,
                                                metadata.icon,
                                            ),
                                        )
                                        .clicked()
                                    {
                                        self.selected_tool = Some(metadata.name);
                                    }
                                }
                            });
                    }
                }
            });
        });
    }
}
