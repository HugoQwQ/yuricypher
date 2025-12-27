use crate::module::Module;
use crate::modules;
use eframe::egui;

pub struct Pipeline {
    modules: Vec<Box<dyn Module>>,
    input_text: String,
    dragged_item_idx: Option<usize>,
}

impl Default for Pipeline {
    fn default() -> Self {
        Self {
            modules: Vec::new(),
            input_text: String::from("The quick brown fox jumps over the lazy dog."),
            dragged_item_idx: None,
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
        self.dragged_item_idx = None;
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
        let mut swap_request = None;

        // Handle drag release
        if ui.input(|i| i.pointer.any_released()) {
            self.dragged_item_idx = None;
        }

        let mut next_dragged_idx = self.dragged_item_idx;
        let current_dragged_idx = self.dragged_item_idx;

        let modules_len = self.modules.len();

        for (idx, module) in self.modules.iter_mut().enumerate() {
            let is_being_dragged = current_dragged_idx == Some(idx);

            ui.push_id(idx, |ui| {
                // Highlight if dragged
                if is_being_dragged {
                    let highlight = ui.style().visuals.selection.bg_fill.linear_multiply(0.3);
                    ui.style_mut().visuals.panel_fill = highlight;
                }

                let response = ui.group(|ui| {
                    ui.horizontal(|ui| {
                        // Drag Handle
                        let handle_response = ui
                            .scope(|ui| {
                                ui.style_mut().interaction.selectable_labels = false;
                                ui.add(egui::Label::new("::").sense(egui::Sense::drag()))
                            })
                            .inner;
                        if handle_response.hovered() {
                            ui.output_mut(|o| o.cursor_icon = egui::CursorIcon::Grab);
                        }
                        if handle_response.drag_started() {
                            next_dragged_idx = Some(idx);
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

                // Swap logic: if dragging and hovering over another item
                if let Some(dragged_idx) = current_dragged_idx {
                    if dragged_idx != idx
                        && response
                            .response
                            .rect
                            .contains(ui.input(|i| i.pointer.hover_pos().unwrap_or_default()))
                    {
                        swap_request = Some((dragged_idx, idx));
                    }
                }
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

        self.dragged_item_idx = next_dragged_idx;

        if let Some(idx) = remove_idx {
            self.modules.remove(idx);
            // If we removed the dragged item, reset drag state
            if self.dragged_item_idx == Some(idx) {
                self.dragged_item_idx = None;
            } else if let Some(dragged) = self.dragged_item_idx {
                // Adjust index if needed
                if idx < dragged {
                    self.dragged_item_idx = Some(dragged - 1);
                }
            }
        }

        if let Some((from, to)) = swap_request {
            self.modules.swap(from, to);
            // Update dragged index to follow the item
            self.dragged_item_idx = Some(to);
        }
    }
}
