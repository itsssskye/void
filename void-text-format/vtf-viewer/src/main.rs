use eframe::{egui, App, Frame};
use egui::ScrollArea;
mod vtf_parser;
use vtf_parser::{parse_vtf, ParsedVtf};

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "VTF Viewer",
        options,
        Box::new(|_cc| Box::new(VtfViewer::default())),
    )
}

#[derive(Default)]
struct VtfViewer {
    content: String,
}

impl App for VtfViewer {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Void Text Format Viewer");

            ui.separator();

            if ui.button("Open .vtf file").clicked() {
                if let Some(path) = rfd::FileDialog::new().add_filter("VTF", &["vtf"]).pick_file() {
                   if let Ok(data) = std::fs::read_to_string(path) {
                        match parse_vtf(&data) {
                            Ok(parsed) => {
                                self.content = format!(
                                    "VERSION: {}\n\nTEXT:\n{}\n\nMETADATA:\n{:#?}\n\nFILES:\n{:#?}",
                                    parsed.version,
                                    parsed.text,
                                    parsed.metadata,
                                    parsed.embedded_files
                                );
                            }
                            Err(e) => {
                                self.content = format!("❌ Failed to parse VTF:\n{}", e);
                            }
                        }
                    }
                }
            }

            ui.separator();

            ScrollArea::vertical().show(ui, |ui| {
                ui.monospace(&self.content);
            });
        });
    }
}