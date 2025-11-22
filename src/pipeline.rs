use crate::module::Module;
use crate::modules;
use eframe::egui;

pub struct Pipeline {
    modules: Vec<Box<dyn Module>>,
    input_text: String,
}

impl Default for Pipeline {
    fn default() -> Self {
        Self {
            modules: Vec::new(),
            input_text: String::from("The quick brown fox jumps over the lazy dog."),
        }
    }
}

impl Pipeline {
    pub fn add_module(&mut self, id: &str) {
        if let Some(module) = modules::create_module(id) {
            self.modules.push(module);
        }
    }

    pub fn clear(&mut self) {
        self.modules.clear();
        self.input_text = String::from("The quick brown fox jumps over the lazy dog.");
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        // Initial Input
        ui.group(|ui| {
            ui.heading("Input");
            ui.add(egui::TextEdit::multiline(&mut self.input_text).desired_width(f32::INFINITY));
        });

        ui.add_space(8.0);
        ui.separator();
        ui.add_space(8.0);

        let mut current_text = self.input_text.clone();

        // Process through modules
        let mut remove_idx = None;
        let mut move_from = None;
        let mut move_to = None;
        let modules_len = self.modules.len();

        for (idx, module) in self.modules.iter_mut().enumerate() {
            ui.push_id(idx, |ui| {
                ui.group(|ui| {
                    ui.horizontal(|ui| {
                        // Up/Down buttons for sorting
                        if ui.button("⬆").clicked()
                            && idx > 0 {
                                move_from = Some(idx);
                                move_to = Some(idx - 1);
                            }
                        if ui.button("⬇").clicked()
                            && idx < modules_len - 1 {
                                move_from = Some(idx);
                                move_to = Some(idx + 1);
                            }

                        ui.heading(module.name());
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui.button("❌").clicked() {
                                remove_idx = Some(idx);
                            }
                        });
                    });

                    module.ui(ui);
                    current_text = module.process(&current_text);

                    ui.separator();
                    ui.horizontal(|ui| {
                        ui.label("Output:");
                        if ui.button("📋").on_hover_text("Copy to clipboard").clicked() {
                            ui.output_mut(|o| o.copied_text = current_text.clone());
                        }
                    });
                    ui.add(
                        egui::TextEdit::multiline(&mut current_text)
                            .interactive(false)
                            .desired_width(f32::INFINITY),
                    );
                });
            });
            ui.add_space(8.0);

            // Draw arrow between modules
            if idx < modules_len - 1 {
                ui.vertical_centered(|ui| {
                    ui.label("⬇");
                });
                ui.add_space(8.0);
            }
        }

        if let Some(idx) = remove_idx {
            self.modules.remove(idx);
        }

        if let (Some(from), Some(to)) = (move_from, move_to) {
            self.modules.swap(from, to);
        }

        ui.add_space(8.0);
        ui.vertical_centered(|ui| {
            ui.label("⬇");
        });
        ui.add_space(8.0);

        ui.group(|ui| {
            ui.horizontal(|ui| {
                ui.heading("Final Output");
                if ui.button("📋").on_hover_text("Copy to clipboard").clicked() {
                    ui.output_mut(|o| o.copied_text = current_text.clone());
                }
            });
            ui.add(
                egui::TextEdit::multiline(&mut current_text)
                    .interactive(false)
                    .desired_width(f32::INFINITY),
            );
        });
    }
}
