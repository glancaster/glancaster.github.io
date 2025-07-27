use eframe::egui::{self, Response};
use egui_flex::{FlexInstance, FlexItem, FlexWidget};

pub struct FlexButton {
    title: String,
    description: String,
    icon: String,
}

impl FlexButton {
    pub fn new(
        title: impl Into<String>,
        description: impl Into<String>,
        icon: impl Into<String>,
    ) -> Self {
        Self {
            title: title.into(),
            description: description.into(),
            icon: icon.into(),
        }
    }
}

impl FlexWidget for FlexButton {
    type Response = Response;

    fn flex_ui(self, item: FlexItem, flex_instance: &mut FlexInstance) -> Self::Response {
        flex_instance
            .add_ui(
                item.sense(egui::Sense::click())
                    .min_height(60.0)
                    .frame_builder(|ui, response| {
                        let style = ui.style().interact(response);

                        (
                            egui::Frame::NONE
                                .fill(style.bg_fill)
                                .stroke(style.bg_stroke),
                            egui::emath::TSTransform::default(),
                        )
                    }),
                |ui| {
                    ui.horizontal(|ui| {
                        ui.label(
                            egui::RichText::new(self.icon).font(egui::FontId::proportional(30.0)),
                        );
                        ui.vertical(|ui| {
                            ui.heading(self.title);
                            ui.label(self.description);
                        });
                    });
                },
            )
            .response
    }
}
