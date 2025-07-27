use crate::tools::tool::Tool;
use eframe::egui;
use eframe::egui::ColorImage;
use eframe::egui::Ui;
use image::ImageReader;

use super::tool::ToolMetadata;

use barcoders::generators::image::*;
use barcoders::sym::code39::*;
use std::io::Cursor;

const NAME: &'static str = "Barcode Generator";
#[derive(Default)]
pub struct Barcode {
    text: String,
    texture: Option<egui::TextureHandle>,
}

impl Barcode {
    pub fn new() -> Self {
        Self::default()
    }
    fn create_barcode(&mut self, ctx: &eframe::egui::Context) {
        let barcode = Code39::new(self.text.to_uppercase()).unwrap();
        let png = Image::png(80); // You must specify the height in pixels.
        let encoded = barcode.encode();

        // Image generators return a Result<Vec<u8>, barcoders::error::Error) of encoded bytes.
        let bytes = png.generate(&encoded[..]).unwrap();

        let image = ImageReader::new(Cursor::new(bytes))
            .with_guessed_format()
            .unwrap()
            .decode()
            .unwrap()
            .to_rgba8();

        let size = [image.width() as usize, image.height() as usize];
        let pixels = image
            .pixels()
            .map(|p| eframe::egui::Color32::from_rgba_unmultiplied(p[0], p[1], p[2], p[3]))
            .collect();

        let color_image = ColorImage { size, pixels };
        self.texture = Some(ctx.load_texture("barcode", color_image, Default::default()));
    }
}

impl Tool for Barcode {
    fn metadata(&self) -> ToolMetadata {
        ToolMetadata {
            name: NAME,
            desc: "Barcode Generator and Utilites",
            icon: "🔧",
        }
    }

    fn ui(&mut self, ui: &mut Ui) {
        ui.heading(NAME);
        ui.separator();
        ui.add_space(20.0);

        if (ui.text_edit_singleline(&mut self.text).changed()
            || ui.button("Generate Barcode").clicked())
            && !self.text.is_empty()
        {
            self.create_barcode(ui.ctx());
        }
        if let Some(texture) = &self.texture {
            if !self.text.is_empty() {
                ui.image(texture);
            }
        }

        ui.separator();
        ui.label("References");
        ui.hyperlink_to("Wiki - Barcode", "https://en.wikipedia.org/wiki/Barcode");
    }
}
pub fn register() {
    crate::register_tool!(Barcode, NAME);
}
