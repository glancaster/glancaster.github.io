use std::io::Cursor;

use crate::tools::tool::Tool;
use eframe::egui::{self, ColorImage, Ui};
use image::{ImageFormat, ImageReader, Luma};
use qrcode::QrCode;

use super::tool::ToolMetadata;

// There is a crate https://lib.rs/crates/qrcode that generates a qr code and an image
// definitely a short putt
// but looking to try my own

const NAME: &'static str = "QR Code Generator";
#[derive(Default)]
pub struct QRCode {
    text: String,
    texture: Option<egui::TextureHandle>,
}

impl QRCode {
    pub fn new() -> Self {
        Self {
            text: "Hello World".to_string(),
            ..Default::default()
        }
    }
    fn create_qrcode(&mut self, ctx: &egui::Context) {
        let scale = 1;
        let code = QrCode::new(self.text.clone()).unwrap();
        let image = code
            .render::<Luma<u8>>()
            .min_dimensions(0, 0)
            .max_dimensions(256, 256)
            .quiet_zone(true)
            .build();

        let image = image::DynamicImage::ImageLuma8(image).resize_exact(
            scale * code.width() as u32,
            scale * code.width() as u32,
            image::imageops::FilterType::Nearest,
        );

        let mut buffer = Vec::new();
        image
            .write_to(&mut Cursor::new(&mut buffer), ImageFormat::Png)
            .unwrap();
        let image = ImageReader::new(Cursor::new(buffer))
            .with_guessed_format()
            .unwrap()
            .decode()
            .unwrap()
            .to_rgba8();

        let size = [image.width() as usize, image.height() as usize];
        let pixels = image
            .pixels()
            .map(|p| egui::Color32::from_rgba_unmultiplied(p[0], p[1], p[2], p[3]))
            .collect();

        let color_image = ColorImage { size, pixels };
        self.texture = Some(ctx.load_texture("qrcode", color_image, Default::default()));
    }
}

impl Tool for QRCode {
    fn metadata(&self) -> ToolMetadata {
        ToolMetadata {
            name: NAME,
            desc: "QR Code Generator and Utilities",
            icon: "⛶",
        }
    }

    fn ui(&mut self, ui: &mut Ui) {
        ui.heading(NAME);
        ui.separator();
        ui.add_space(20.0);

        ui.horizontal(|ui| {
            ui.label("text:");
            ui.add(egui::TextEdit::singleline(&mut self.text)); //&'static str doesn't work on this ui widget
        });

        if let Some(texture) = &self.texture {
            ui.image(texture);
        }

        if ui.button("Generate").clicked() {
            self.create_qrcode(ui.ctx());
        }
        ui.separator();
        ui.label("References");
        ui.hyperlink_to("Wiki - QR code", "https://en.wikipedia.org/wiki/QR_code");
    }
}
pub fn register() {
    crate::register_tool!(QRCode, NAME);
}
