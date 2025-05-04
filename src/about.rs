use eframe::egui::{self, Color32, RichText};

pub fn ui(ui: &mut egui::Ui) {
    ui.label(
        RichText::new("Hey, I'm Graham")
            .heading()
            .color(Color32::WHITE),
    );

    ui.separator();

    ui.label("Welcome to my personal website! I hope to use this space to share my projects and technical topics that I've learned over the years. Will mostly be software or mechanical engineering as I am currently exploring all the ways I can use the Rust programming language, and microcontrollers. I enjoy creating new ideas, learning new skills, and building anything mechanical or programmable. if i'm not behind my computer, you'll find me walking around the city or bouldering. 

I recently graduated from Clemson University with my Master's in Mechanical Engineering and currently working for GE Vernova to help develop software (embedded/UI/networking) for custom machines of various manufacturing processes such as Eletrical Discharge Machining and Laser Ablation.");

    ui.label(RichText::new("My Skills").heading().color(Color32::WHITE));
    ui.label("Here's a list of skills that I am experienced in at various levels.");
    ui.horizontal(|ui| {
        for w in [
            "Programming Languages",
            "Embedded Systems",
            "Networking",
            "SQL",
        ] {
            egui::Frame::group(ui.style())
                .fill(egui::Color32::from_gray(60))
                .show(ui, |ui| {
                    ui.vertical(|ui| {
                        ui.label(w);
                        ui.horizontal(|ui| {
                            let list = match w {
                                "Programming Languages" => vec!["Rust", "Python", "C", "C++", "C#"],
                                "Embedded Systems" => {
                                    vec!["STM32", "ESP32", "Arduino", "SPI", "UART"]
                                }
                                "Networking" => vec!["TCP", "SSH", "Telnet"],
                                "SQL" => vec!["Servers", "Schemas", "Databases"],
                                _ => vec![],
                            };
                            for p in list {
                                ui.button(p);
                            }
                        });
                    });
                });
        }
    });

    ui.label(RichText::new("My Toolkit").heading().color(Color32::WHITE));
    ui.label("Here's the set of tools that I use regularly or at least try to incorporate them into my projects to deepen my experience and knowledge with them. This section could become its own blogpost or something if i get around to noting down the all the tools and config I use. Most of these will be a link to the tool or my config.");

    ui.horizontal(|ui| {
        for w in ["neovim", "sharex", "ghostty", "obsidian", "surfingkeys"] {
            ui.button(w);
        }
    });
}
