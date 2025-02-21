use eframe::egui::{self, Align, Area, Layout};

#[derive(PartialEq)]
enum Areas {
    Home,
    About,
    Resume,
    Projects,
    Tools,
}
struct Website {
    area: Areas,
}

impl Website {
    fn new() -> Self {
        Self {
            area: Areas::Home,
        }
    }
}

#[derive(Clone, Copy)]
struct FlipDot {
    state: bool,
    progress: f32,
}

impl FlipDot {
    fn new(state: bool) -> Self {
        Self { state, progress: 0.0 }
    }
}

impl egui::Widget for &mut FlipDot {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let (rect, response) = ui.allocate_exact_size(egui::vec2(20.0,20.0), egui::Sense::click());

        if response.clicked() {
            self.state = !self.state;
            self.progress = 0.0;
        }
        let flip_color = if self.state { egui::Color32::YELLOW } else { egui::Color32::BLACK };
        let bg_color = if self.state { egui::Color32::BLACK } else { egui::Color32::YELLOW };
        
        // Draw background
        ui.painter().rect_filled(rect, 4.0, bg_color);
        
        // Simulate flip effect by interpolating between colors
        let interp_color = egui::Color32::from_rgba_premultiplied(
            (flip_color.r() as f32 * self.progress) as u8,
            (flip_color.g() as f32 * self.progress) as u8,
            (flip_color.b() as f32 * self.progress) as u8,
            255,
        );
        
        ui.painter().circle_filled(rect.center(), 8.0, interp_color);
        
        response
    }
}

fn flipdot_display(ui: &mut egui::Ui, data: &mut Vec<Vec<FlipDot>>) {
    egui::Grid::new("flipdot_grid").show(ui, |ui| {
        for row in data.iter_mut() {
            for dot in row.iter_mut() {
                ui.add(dot);
            }
            ui.end_row();
        }
    });
}

#[cfg(target_arch = "wasm32")]
fn main() {
use eframe::wasm_bindgen::JsCast as _;

    // Redirect `log` message to `console.log` and friends:
    //eframe::WebLogger::init(log::LevelFilter::Debug).ok();

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
    eframe::run_native("My Website", options, Box::new(|_| Ok(Box::new(Website::new()))));
}


impl eframe::App for Website {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        
        egui::TopBottomPanel::top("menu").show(ctx, |ui | {
            ui.horizontal(|ui| {
           ui.with_layout(Layout::centered_and_justified(egui::Direction::LeftToRight), |ui| {
                ui.horizontal(|ui| {
                    ui.selectable_value(&mut self.area, Areas::Home, "home");
                    ui.selectable_value(&mut self.area, Areas::About, "about");
                    ui.selectable_value(&mut self.area, Areas::Resume, "resume");
                    ui.selectable_value(&mut self.area, Areas::Projects, "projects");
                    ui.selectable_value(&mut self.area, Areas::Tools, "tools");
                });
           });
           ui.with_layout(Layout::centered_and_justified(egui::Direction::RightToLeft), |ui| {
                ui.horizontal(|ui| {
                    ui.hyperlink_to("github", "https://github.com/glancaster");
                    ui.hyperlink_to("linkedin", "https://www.linkedin.com/in/grlanca/");
                });
           });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            match self.area {
                Areas::Home => Website::home(ui),
                Areas::About => Website::about(ui),
                Areas::Resume => Website::resume(ui),
                Areas::Projects => Website::projects(ui),
                Areas::Tools => Website::tools(ui),
            }
        });
    }
}

impl Website {
    pub fn home(ui: &mut egui::Ui) {
        ui.with_layout(Layout::centered_and_justified(egui::Direction::LeftToRight), |ui| {
            ui.label("Graham");
        });
    }
    pub fn about(ui: &mut egui::Ui) {
        ui.with_layout(Layout::centered_and_justified(egui::Direction::LeftToRight), |ui| {
            ui.label("About");
        });
    }
    pub fn resume(ui: &mut egui::Ui) {
        ui.with_layout(Layout::centered_and_justified(egui::Direction::LeftToRight), |ui| {
            ui.label("Resume");
        }); 
    }
    pub fn projects(ui: &mut egui::Ui) {
        ui.with_layout(Layout::centered_and_justified(egui::Direction::LeftToRight), |ui| {
            ui.label("Projects");
        }); 
    }
    pub fn tools(ui: &mut egui::Ui) {
        ui.with_layout(Layout::centered_and_justified(egui::Direction::LeftToRight), |ui| {
            ui.label("Tools");
        }); 
    }
}