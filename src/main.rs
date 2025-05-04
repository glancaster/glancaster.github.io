mod about;

use eframe::egui::{self, Align, Area, Color32, Layout, Pos2, Rect, RichText, Shape, Vec2, Widget};
use std::fs;
use std::{mem::swap, thread::sleep, time::Duration};

#[derive(PartialEq)]
enum Areas {
    Home,
    About,
    Projects,
    Tools,
}
struct Website {
    area: Areas,
    name: String,
}

impl Website {
    fn new() -> Self {
        Self {
            area: Areas::About,
            name: "brady".to_owned(),
        }
    }
}

#[cfg(target_arch = "wasm32")]
fn main() {
    use eframe::wasm_bindgen::JsCast as _;
    let web_options = eframe::WebOptions::default();
    wasm_bindgen_futures::spawn_local(async {
        let document = eframe::web_sys::window()
            .expect("No window")
            .document()
            .expect("No document");

        let canvas = document
            .get_element_by_id("the_canvas_id")
            .expect("Failed to find the_canvas_id")
            .dyn_into::<eframe::web_sys::HtmlCanvasElement>()
            .expect("the_canvas_id was not a HtmlCanvasElement");

        let start_result = eframe::WebRunner::new()
            .start(
                canvas,
                web_options,
                Box::new(|_cc| Ok(Box::new(Website::new()))),
            )
            .await;
    });
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "My Website",
        options,
        Box::new(|_| Ok(Box::new(Website::new()))),
    );
}

impl eframe::App for Website {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("menu").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.with_layout(
                    Layout::centered_and_justified(egui::Direction::LeftToRight),
                    |ui| {
                        ui.horizontal(|ui| {
                            ui.selectable_value(&mut self.area, Areas::Home, "home");
                            ui.selectable_value(&mut self.area, Areas::About, "about");
                            ui.selectable_value(&mut self.area, Areas::Projects, "projects");
                            ui.selectable_value(&mut self.area, Areas::Tools, "tools");
                        });
                    },
                );
                ui.with_layout(
                    Layout::centered_and_justified(egui::Direction::RightToLeft),
                    |ui| {
                        ui.horizontal(|ui| {
                            ui.hyperlink_to("github", "https://github.com/glancaster");
                            ui.hyperlink_to("linkedin", "https://www.linkedin.com/in/grlanca/");
                        });
                    },
                );
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| match self.area {
                Areas::Home => Website::home(self, ui),
                Areas::About => about::ui(ui),
                Areas::Projects => Website::projects(ui),
                Areas::Tools => Website::tools(ui),
            });
        });
    }
}

impl Website {
    pub fn home(&mut self, ui: &mut egui::Ui) {
        let rect = ui.available_rect_before_wrap();
        let s = 0.7;
        let crect = egui::Rect::from_center_size(
            rect.center(),
            egui::vec2(rect.width() * s, rect.width() * s / 3.0),
        );

        let mut ui_builder = egui::UiBuilder::new();
        ui_builder = ui_builder.max_rect(crect);

        ui.allocate_new_ui(ui_builder, |ui| {
            ui.vertical_centered(|ui| {
                ui.label("undergoing changes");
                //ui.horizontal(|ui|{
                //    let w = ui.available_width()*0.8;
                //    for c in "0123456789ABCDEF".chars() {
                //        ui.add(FourteenSegmentDisplay::new(c, crect.width()/16.));
                //    }
                //});
                //ui.horizontal(|ui|{
                //    let w = ui.available_width()*0.8;
                //    for c in "0123456789ABCDEF".chars() {
                //        ui.add(FourteenSegmentDisplay::new(c, w/17.0));
                //    }
                //});
            });
        });
    }
    pub fn about(ui: &mut egui::Ui) {
        ui.label(RichText::new("Neovim").heading().color(Color32::WHITE));
        ui.label("One of the most helpful editors I have used to tackle navigating and modifying text or code. It's a fork of Vim thats built with the Lua scripting language to have better support for plugins. It does take some to learn the keymaps but once you understand the motions, you will be flying through code and all without touching your mouse. 

> My config files can be found on (github link)[todo!()]
> - rust analyzer");

        ui.label(
            RichText::new("Rust Programming Language")
                .heading()
                .color(Color32::WHITE),
        );
        ui.label("Such a great language to use and learn from. I've been exclusively trying to have any new projects be written with Rust to further explore the language. I believe its a language that will last a long time and be compared equally or better to C/C++. Here are some language specific features/tools to look at.  

> Favorite tools");

        ui.label(RichText::new("Ghostty").heading().color(Color32::WHITE));
        ui.label("Quick and feature rich terminal emulator. Loads big files with no problems. I use this with zsh or nushell.");

        ui.label(RichText::new("Obsidian").heading().color(Color32::WHITE));
        ui.label("Great tool for making notes with Markdown and has plenty of plugins to support your workflow.");
        ui.label(RichText::new("VS Code").heading().color(Color32::WHITE));
        ui.label("Sometimes I'll take a break from Neovim and see what neat features are available on VS Code.");
        ui.label(RichText::new("CLI tools").heading().color(Color32::WHITE));
        ui.label("* ripgrep");

        ui.label(RichText::new("3D Modeling").heading().color(Color32::WHITE));
        ui.label("Exploring FreeCAD currently but may adventure on making my own parametric precision 3D modeling tool.");

        ui.label(
            RichText::new("Let's Adventure")
                .heading()
                .color(Color32::WHITE),
        );
        ui.label(
            "- U.S. States visited: 25/50
- National Parks visited: 10/63
",
        );

        //});
    }
    pub fn projects(ui: &mut egui::Ui) {
        ui.with_layout(
            Layout::centered_and_justified(egui::Direction::LeftToRight),
            |ui| {
                ui.label("Projects");
            },
        );
    }
    pub fn tools(ui: &mut egui::Ui) {
        ui.with_layout(
            Layout::centered_and_justified(egui::Direction::LeftToRight),
            |ui| {
                ui.label("Tools");
            },
        );
    }
}

pub struct FourteenSegmentDisplay {
    letter: char,
    on_color: Color32,
    off_color: Color32,
    width: f32,
}

impl FourteenSegmentDisplay {
    pub fn new(letter: char, width: f32) -> Self {
        Self {
            letter,
            on_color: Color32::LIGHT_BLUE,
            off_color: Color32::DARK_GRAY,
            width,
        }
    }

    // hashmap?
    pub fn map(letter: char) -> [u8; 14] {
        match letter as u8 {
            0 => [1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 0, 0],
            1 => [0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            2 => [1, 1, 0, 1, 1, 0, 1, 1, 0, 0, 0, 0, 0, 0],
            3 => [1, 1, 1, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0],
            4 => [0, 1, 1, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0],
            5 => [1, 0, 0, 1, 0, 1, 1, 0, 0, 0, 0, 0, 0, 1],
            6 => [1, 0, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0],
            7 => [1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            8 => [1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0],
            9 => [1, 1, 1, 1, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0],
            65 => [1, 1, 1, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0],
            66 => [1, 1, 1, 1, 0, 0, 0, 1, 0, 1, 0, 0, 1, 0],
            67 => [1, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0],
            68 => [1, 1, 1, 1, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0],
            69 => [1, 0, 0, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0],
            70 => [1, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0],
            71 => [1, 0, 1, 1, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0],
            72 => [0, 1, 1, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0],
            73 => [1, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0],
            74 => [0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            75 => [0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 1, 0, 0, 1],
            76 => [0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0],
            77 => [0, 1, 1, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0, 0],
            78 => [0, 1, 1, 0, 1, 1, 0, 0, 1, 0, 0, 0, 0, 1],
            79 => [1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0],
            80 => [1, 1, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0],
            81 => [1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1],
            82 => [1, 1, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 1],
            83 => [1, 0, 1, 1, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0],
            84 => [1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0],
            85 => [0, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0],
            86 => [0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 1, 1, 0, 0],
            87 => [0, 1, 1, 0, 1, 1, 0, 0, 0, 0, 0, 1, 0, 1],
            88 => [0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 1, 0, 1],
            89 => [0, 1, 1, 1, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0],
            90 => [1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0],

            _ => [0; 14],
        }
    }
}

impl egui::Widget for FourteenSegmentDisplay {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let size = Vec2::new(self.width, self.width * 2.);
        let (rect, response) = ui.allocate_exact_size(size, egui::Sense::hover());
        if ui.is_rect_visible(rect) {
            let painter = ui.painter();

            let x = rect.min.x;
            let y = rect.min.y;

            let seg_width = size.x * 0.15;
            let seg_height = size.y * 0.45;

            let hw = seg_width / 2.0;
            let hh = seg_height / 2.0;

            let pts_a = vec![
                Pos2::new(x + hw, y + hw),
                Pos2::new(x + seg_width, y),
                Pos2::new(x + seg_height, y),
                Pos2::new(x + seg_height + hw, y + hw),
                Pos2::new(x + seg_height, y + seg_width),
                Pos2::new(x + seg_width, y + seg_width),
            ];
            let pts_b = vec![
                Pos2::new(x + seg_height + hw, y + hw),
                Pos2::new(x + seg_height + seg_width, y + seg_width),
                Pos2::new(x + seg_height + seg_width, y + seg_height),
                Pos2::new(x + seg_height + hw, y + seg_height + hw),
                Pos2::new(x + seg_height, y + seg_height),
                Pos2::new(x + seg_height, y + seg_width),
            ];
            let pts_c = vec![
                Pos2::new(x + seg_height + hw, seg_height + y + hw),
                Pos2::new(x + seg_height + seg_width, seg_height + y + seg_width),
                Pos2::new(x + seg_height + seg_width, seg_height * 2. + y),
                Pos2::new(x + seg_height + hw, seg_height * 2. + y + hw),
                Pos2::new(x + seg_height, seg_height * 2. + y),
                Pos2::new(x + seg_height, seg_height + y + seg_width),
            ];
            let pts_d = vec![
                Pos2::new(x + hw, seg_height * 2. + y + hw),
                Pos2::new(x + seg_width, seg_height * 2. + y),
                Pos2::new(x + seg_height, seg_height * 2. + y),
                Pos2::new(x + seg_height + hw, seg_height * 2. + y + hw),
                Pos2::new(x + seg_height, seg_height * 2. + y + seg_width),
                Pos2::new(x + seg_width, seg_height * 2. + y + seg_width),
            ];
            let pts_e = vec![
                Pos2::new(x + hw, seg_height + y + hw),
                Pos2::new(x + seg_width, seg_height + y + seg_width),
                Pos2::new(x + seg_width, seg_height * 2. + y),
                Pos2::new(x + hw, seg_height * 2. + y + hw),
                Pos2::new(x, seg_height * 2. + y),
                Pos2::new(x, seg_height + y + seg_width),
            ];
            let pts_f = vec![
                Pos2::new(x + hw, y + hw),
                Pos2::new(x + seg_width, y + seg_width),
                Pos2::new(x + seg_width, y + seg_height),
                Pos2::new(x + hw, y + seg_height + hw),
                Pos2::new(x, y + seg_height),
                Pos2::new(x, y + seg_width),
            ];
            let pts_g1 = vec![
                Pos2::new(x + hw, seg_height + y + hw),
                Pos2::new(x + seg_width, seg_height + y),
                Pos2::new(x + seg_height / 2., seg_height + y),
                Pos2::new(x + seg_height / 2. + hw, seg_height + y + hw),
                Pos2::new(x + seg_height / 2., seg_height + y + seg_width),
                Pos2::new(x + seg_width, seg_height + y + seg_width),
            ];
            let pts_g2 = vec![
                Pos2::new(
                    x + seg_width / 2. + seg_height / 2.,
                    seg_height + y + seg_width / 2.,
                ),
                Pos2::new(x + seg_width + seg_height / 2., seg_height + y),
                Pos2::new(x + seg_height, seg_height + y),
                Pos2::new(x + seg_height + hw, seg_height + y + hw),
                Pos2::new(x + seg_height, seg_height + y + seg_width),
                Pos2::new(x + seg_width + seg_height / 2., seg_height + y + seg_width),
            ];
            let pts_h = vec![
                Pos2::new(x + seg_width, y + seg_width),
                Pos2::new(x + seg_width + seg_width * 0.6, y + seg_width),
                Pos2::new(x + hh, y + seg_height - seg_width * 1.2),
                Pos2::new(x + hh, y + seg_height),
                Pos2::new(x + hh - seg_width * 0.6, y + seg_height),
                Pos2::new(x + seg_width, y + seg_width + seg_width * 1.2),
            ];
            let pts_j = vec![
                Pos2::new(x + seg_height / 2. + seg_width / 2., y + seg_width),
                Pos2::new(x + seg_height / 2. + seg_width, y + seg_width * 1.5),
                Pos2::new(x + seg_height / 2. + seg_width, y + seg_height),
                Pos2::new(
                    x + seg_height / 2. + seg_width / 2.,
                    y + seg_height + seg_width / 2.,
                ),
                Pos2::new(x + seg_height / 2., y + seg_height),
                Pos2::new(x + seg_height / 2., y + seg_width * 1.5),
            ];
            let pts_k = vec![
                Pos2::new(x + seg_height, y + seg_width),
                Pos2::new(x + seg_height - seg_width * 0.6, y + seg_width),
                Pos2::new(x + hh + seg_width, y + seg_height - seg_width * 1.2),
                Pos2::new(x + hh + seg_width, y + seg_height),
                Pos2::new(x + hh + seg_width + seg_width * 0.6, y + seg_height),
                Pos2::new(x + seg_height, y + seg_width + seg_width * 1.2),
            ];
            let pts_l = vec![
                Pos2::new(x + seg_height / 2., y + seg_height + seg_width),
                Pos2::new(
                    x + seg_height / 2. - seg_width * 0.6,
                    y + seg_height + seg_width,
                ),
                Pos2::new(x + seg_width, y + seg_height * 2. - seg_width * 1.2),
                Pos2::new(x + seg_width, y + seg_height * 2.),
                Pos2::new(x + seg_width + seg_width * 0.6, y + seg_height * 2.),
                Pos2::new(
                    x + seg_height / 2.,
                    y + seg_width + seg_width * 1.2 + seg_height,
                ),
            ];
            let pts_m = vec![
                Pos2::new(
                    x + seg_height / 2. + seg_width / 2.,
                    y + seg_width / 2. + seg_height,
                ),
                Pos2::new(x + seg_height / 2. + seg_width, y + seg_width + seg_height),
                Pos2::new(
                    x + seg_height / 2. + seg_width,
                    y - seg_width / 2. + seg_height + seg_height,
                ),
                Pos2::new(
                    x + seg_height / 2. + seg_width / 2.,
                    y + seg_height + seg_height,
                ),
                Pos2::new(
                    x + seg_height / 2.,
                    y - seg_width / 2. + seg_height + seg_height,
                ),
                Pos2::new(x + seg_height / 2., y + seg_width + seg_height),
            ];
            let pts_n = vec![
                Pos2::new(x + seg_width + hh, y + seg_width + seg_height),
                Pos2::new(
                    x + seg_width + hh + seg_width * 0.6,
                    y + seg_width + seg_height,
                ),
                Pos2::new(
                    x + seg_height,
                    y + seg_height - seg_width * 1.2 + seg_height,
                ),
                Pos2::new(x + seg_height, y + seg_height + seg_height),
                Pos2::new(
                    x + seg_height - seg_width * 0.6,
                    y + seg_height + seg_height,
                ),
                Pos2::new(
                    x + seg_width + hh,
                    y + seg_width + seg_width * 1.2 + seg_height,
                ),
            ];

            let segments = [
                pts_a, pts_b, pts_c, pts_d, pts_e, pts_f, pts_g1, pts_g2, pts_h, pts_j, pts_k,
                pts_l, pts_m, pts_n,
            ];
            for (i, status) in FourteenSegmentDisplay::map(self.letter).iter().enumerate() {
                painter.add(Shape::convex_polygon(
                    segments[i].clone(),
                    if *status == 1 {
                        self.on_color
                    } else {
                        self.off_color
                    },
                    egui::Stroke::NONE,
                ));
            }
        }
        response
    }
}
