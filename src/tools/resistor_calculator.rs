use crate::tools::tool::Tool;
use eframe::egui::{self, Ui};
use enum_iterator::{all, Sequence};

use super::tool::ToolMetadata;

const NAME: &'static str = "Resistor Calculator";
#[derive(Default, Debug, PartialEq)]
pub struct ResistorCalculator {
    description: String,
    icon: String,
    n_bands: u8,
    total_ohms: usize,
    tolerance: f32,
    band1: BandColor,
    band2: BandColor,
    band3: BandColor,
    multi: MultiplierColor,
    tol: ToleranceColor,
    temp: TempCoefColor,
}
#[derive(Default, Debug, PartialEq, Sequence, Clone, Copy)]
enum BandColor {
    #[default]
    Black,
    Brown,
    Red,
    Orange,
    Yellow,
    Green,
    Blue,
    Violet,
    Grey,
    White,
}
#[derive(Default, Debug, PartialEq, Sequence, Clone, Copy)]
enum MultiplierColor {
    #[default]
    Black = 1,
    Brown = 10,
    Red = 100,
    Orange = 1000,
    Yellow = 10000,
    Green = 100000,
    Blue = 1000000,
    Violet = 10000000,
    Grey = 100000000,
    White = 1000000000,
    Gold = 2,
    Silver = 3,
}
#[derive(Default, Debug, PartialEq, Sequence, Clone, Copy)]
enum ToleranceColor {
    #[default]
    Brown = 100,
    Red = 200,
    Orange = 5,
    Yellow = 2,
    Green = 50,
    Blue = 25,
    Violet = 10,
    Grey = 1,
    Gold = 500,
    Silver = 1000,
}
#[derive(Default, Debug, PartialEq, Sequence, Clone, Copy)]
enum TempCoefColor {
    #[default]
    Black = 250,
    Brown = 100,
    Red = 50,
    Orange = 15,
    Yellow = 25,
    Green = 20,
    Blue = 10,
    Violet = 5,
    Grey = 1,
}

impl ResistorCalculator {
    pub fn new() -> Self {
        Self {
            description: "Calculate the Ohms of a Resistor interactively.".to_string(),
            icon: "Ω".to_string(),
            ..Default::default()
        }
    }
}

impl Tool for ResistorCalculator {
    fn metadata(&self) -> ToolMetadata {
        ToolMetadata {
            name: NAME,
            desc:
                "Interactively Calculate the Ohms of a Resistor based on Number and Color of Bands.",
            icon: "Ω",
        }
    }
    // pub fn desc(&self) -> String {
    //     self.description.clone()
    // }
    // pub fn icon(&self) -> String {
    //     self.icon.clone()
    // }
    fn ui(&mut self, ui: &mut Ui) {
        ui.heading("Resistor Calculator");
        ui.separator();
        ui.add_space(20.0);
        if self.n_bands == 6u8 {
            ui.label(
                egui::RichText::new(format!(
                    "{} Ω ±{}% {} ppm/K",
                    self.total_ohms, self.tolerance, self.temp as usize
                ))
                .font(egui::FontId::proportional(36.0)),
            );
        } else {
            ui.label(
                egui::RichText::new(format!("{} Ω ±{}%", self.total_ohms, self.tolerance))
                    .font(egui::FontId::proportional(36.0)),
            );
        }
        ui.add_space(20.0);
        ui.label("Number of Bands");
        ui.horizontal(|ui| {
            for i in 3..=6 {
                ui.selectable_value(&mut self.n_bands, i, format!("\t{i}\t"));
            }
        });
        self.n_bands = self.n_bands.clamp(3, 6);

        match &self.n_bands {
            3 => {
                egui::ComboBox::from_label("First band")
                    .selected_text(format!("{:?}", self.band1))
                    .show_ui(ui, |ui| {
                        for color in all::<BandColor>().collect::<Vec<_>>() {
                            let display = format!("{:?}", &color);
                            ui.selectable_value(&mut self.band1, color, display);
                        }
                    });
                egui::ComboBox::from_label("Second band")
                    .selected_text(format!("{:?}", self.band2))
                    .show_ui(ui, |ui| {
                        for color in all::<BandColor>().collect::<Vec<_>>() {
                            let display = format!("{:?}", &color);
                            ui.selectable_value(&mut self.band2, color, display);
                        }
                    });
                egui::ComboBox::from_label("Multiplier")
                    .selected_text(format!("{:?}", self.multi))
                    .show_ui(ui, |ui| {
                        for color in all::<MultiplierColor>().collect::<Vec<_>>() {
                            let display = format!("{:?}", &color);
                            ui.selectable_value(&mut self.multi, color, display);
                        }
                    });
                self.tolerance = 20.0;
                match self.multi {
                    MultiplierColor::Gold => {
                        self.total_ohms = ((self.band1 as usize * 10 + self.band2 as usize) as f64
                            * 0.1) as usize;
                    }
                    MultiplierColor::Silver => {
                        self.total_ohms = ((self.band1 as usize * 10 + self.band2 as usize) as f64
                            * 0.01) as usize;
                    }
                    _ => {
                        self.total_ohms =
                            (self.band1 as usize * 10 + self.band2 as usize) * self.multi as usize;
                    }
                }
            }
            4 => {
                egui::ComboBox::from_label("First band")
                    .selected_text(format!("{:?}", self.band1))
                    .show_ui(ui, |ui| {
                        for color in all::<BandColor>().collect::<Vec<_>>() {
                            let display = format!("{:?}", &color);
                            ui.selectable_value(&mut self.band1, color, display);
                        }
                    });
                egui::ComboBox::from_label("Second band")
                    .selected_text(format!("{:?}", self.band2))
                    .show_ui(ui, |ui| {
                        for color in all::<BandColor>().collect::<Vec<_>>() {
                            let display = format!("{:?}", &color);
                            ui.selectable_value(&mut self.band2, color, display);
                        }
                    });
                egui::ComboBox::from_label("Multiplier")
                    .selected_text(format!("{:?}", self.multi))
                    .show_ui(ui, |ui| {
                        for color in all::<MultiplierColor>().collect::<Vec<_>>() {
                            let display = format!("{:?}", &color);
                            ui.selectable_value(&mut self.multi, color, display);
                        }
                    });
                egui::ComboBox::from_label("Tolerance")
                    .selected_text(format!("{:?}", self.tol))
                    .show_ui(ui, |ui| {
                        for color in all::<ToleranceColor>().collect::<Vec<_>>() {
                            let display = format!("{:?}", &color);
                            ui.selectable_value(&mut self.tol, color, display);
                        }
                    });
                self.tolerance = (self.tol as usize) as f32 / 100.0;
                match self.multi {
                    MultiplierColor::Gold => {
                        self.total_ohms = ((self.band1 as usize * 10 + self.band2 as usize) as f64
                            * 0.1) as usize;
                    }
                    MultiplierColor::Silver => {
                        self.total_ohms = ((self.band1 as usize * 10 + self.band2 as usize) as f64
                            * 0.01) as usize;
                    }
                    _ => {
                        self.total_ohms =
                            (self.band1 as usize * 10 + self.band2 as usize) * self.multi as usize;
                    }
                }
            }
            5 => {
                egui::ComboBox::from_label("First band")
                    .selected_text(format!("{:?}", self.band1))
                    .show_ui(ui, |ui| {
                        for color in all::<BandColor>().collect::<Vec<_>>() {
                            let display = format!("{:?}", &color);
                            ui.selectable_value(&mut self.band1, color, display);
                        }
                    });
                egui::ComboBox::from_label("Second band")
                    .selected_text(format!("{:?}", self.band2))
                    .show_ui(ui, |ui| {
                        for color in all::<BandColor>().collect::<Vec<_>>() {
                            let display = format!("{:?}", &color);
                            ui.selectable_value(&mut self.band2, color, display);
                        }
                    });
                egui::ComboBox::from_label("Third band")
                    .selected_text(format!("{:?}", self.band3))
                    .show_ui(ui, |ui| {
                        for color in all::<BandColor>().collect::<Vec<_>>() {
                            let display = format!("{:?}", &color);
                            ui.selectable_value(&mut self.band3, color, display);
                        }
                    });
                egui::ComboBox::from_label("Multiplier")
                    .selected_text(format!("{:?}", self.multi))
                    .show_ui(ui, |ui| {
                        for color in all::<MultiplierColor>().collect::<Vec<_>>() {
                            let display = format!("{:?}", &color);
                            ui.selectable_value(&mut self.multi, color, display);
                        }
                    });
                egui::ComboBox::from_label("Tolerance")
                    .selected_text(format!("{:?}", self.tol))
                    .show_ui(ui, |ui| {
                        for color in all::<ToleranceColor>().collect::<Vec<_>>() {
                            let display = format!("{:?}", &color);
                            ui.selectable_value(&mut self.tol, color, display);
                        }
                    });
                self.tolerance = (self.tol as usize) as f32 / 100.0;
                match self.multi {
                    MultiplierColor::Gold => {
                        self.total_ohms = ((self.band1 as usize * 100
                            + self.band2 as usize * 10
                            + self.band3 as usize)
                            as f64
                            * 0.1) as usize;
                    }
                    MultiplierColor::Silver => {
                        self.total_ohms = ((self.band1 as usize * 100
                            + self.band2 as usize * 10
                            + self.band3 as usize)
                            as f64
                            * 0.01) as usize;
                    }
                    _ => {
                        self.total_ohms = (self.band1 as usize * 100
                            + self.band2 as usize * 10
                            + self.band3 as usize)
                            * self.multi as usize;
                    }
                }
            }
            6 => {
                egui::ComboBox::from_label("First band")
                    .selected_text(format!("{:?}", self.band1))
                    .show_ui(ui, |ui| {
                        for color in all::<BandColor>().collect::<Vec<_>>() {
                            let display = format!("{:?}", &color);
                            ui.selectable_value(&mut self.band1, color, display);
                        }
                    });
                egui::ComboBox::from_label("Second band")
                    .selected_text(format!("{:?}", self.band2))
                    .show_ui(ui, |ui| {
                        for color in all::<BandColor>().collect::<Vec<_>>() {
                            let display = format!("{:?}", &color);
                            ui.selectable_value(&mut self.band2, color, display);
                        }
                    });
                egui::ComboBox::from_label("Third band")
                    .selected_text(format!("{:?}", self.band3))
                    .show_ui(ui, |ui| {
                        for color in all::<BandColor>().collect::<Vec<_>>() {
                            let display = format!("{:?}", &color);
                            ui.selectable_value(&mut self.band3, color, display);
                        }
                    });
                egui::ComboBox::from_label("Multiplier")
                    .selected_text(format!("{:?}", self.multi))
                    .show_ui(ui, |ui| {
                        for color in all::<MultiplierColor>().collect::<Vec<_>>() {
                            let display = format!("{:?}", &color);
                            ui.selectable_value(&mut self.multi, color, display);
                        }
                    });
                egui::ComboBox::from_label("Tolerance")
                    .selected_text(format!("{:?}", self.tol))
                    .show_ui(ui, |ui| {
                        for color in all::<ToleranceColor>().collect::<Vec<_>>() {
                            let display = format!("{:?}", &color);
                            ui.selectable_value(&mut self.tol, color, display);
                        }
                    });
                egui::ComboBox::from_label("Temperature Coefficient")
                    .selected_text(format!("{:?}", self.temp))
                    .show_ui(ui, |ui| {
                        for color in all::<TempCoefColor>().collect::<Vec<_>>() {
                            let display = format!("{:?}", &color);
                            ui.selectable_value(&mut self.temp, color, display);
                        }
                    });
                self.tolerance = (self.tol as usize) as f32 / 100.0;
                match self.multi {
                    MultiplierColor::Gold => {
                        self.total_ohms = ((self.band1 as usize * 100
                            + self.band2 as usize * 10
                            + self.band3 as usize)
                            as f64
                            * 0.1) as usize;
                    }
                    MultiplierColor::Silver => {
                        self.total_ohms = ((self.band1 as usize * 100
                            + self.band2 as usize * 10
                            + self.band3 as usize)
                            as f64
                            * 0.01) as usize;
                    }
                    _ => {
                        self.total_ohms = (self.band1 as usize * 100
                            + self.band2 as usize * 10
                            + self.band3 as usize)
                            * self.multi as usize;
                    }
                }
            }
            _ => {}
        }
        ui.separator();
        //table
        ui.separator();
        ui.heading("roadmap");

        ui.checkbox(&mut false, "diagram, maybe interactive");
        ui.checkbox(&mut false, "resistor table");

        ui.separator();
        ui.label("References");
        ui.hyperlink_to(
            "Wiki - Electronic Color Code",
            "https://en.wikipedia.org/wiki/Electronic_color_code",
        );
        ui.hyperlink_to("Resistor Color Code Calculator", "https://www.calculator.net/resistor-calculator.html?bandnum=4&band1=brown&band2=black&band3=black&multiplier=black&tolerance=brown&temperatureCoefficient=brown&type=c&x=Calculate");
    }
}

pub fn register() {
    crate::register_tool!(ResistorCalculator, NAME);
}
