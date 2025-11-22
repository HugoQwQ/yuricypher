use crate::pipeline::Pipeline;
use eframe::egui;

pub struct YuryCipherApp {
    pipeline: Pipeline,
    show_settings: bool,
    current_lang: String,
}

impl YuryCipherApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            pipeline: Pipeline::default(),
            show_settings: false,
            current_lang: "en".to_string(),
        }
    }
}

impl eframe::App for YuryCipherApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("Reset Pipeline").clicked() {
                    self.pipeline.clear();
                }
                if ui.button("Settings").clicked() {
                    self.show_settings = true;
                }
            });
        });

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Modules");
            ui.separator();

            egui::ScrollArea::vertical().show(ui, |ui| {
                egui::CollapsingHeader::new("Transform")
                    .default_open(true)
                    .show(ui, |ui| {
                        if ui
                            .button(rust_i18n::t!("modules.replace"))
                            .on_hover_text(rust_i18n::t!("tooltips.replace"))
                            .clicked()
                        {
                            self.pipeline.add_module("replace");
                        }
                        if ui
                            .button(rust_i18n::t!("modules.reverse"))
                            .on_hover_text(rust_i18n::t!("tooltips.reverse"))
                            .clicked()
                        {
                            self.pipeline.add_module("reverse");
                        }
                        if ui
                            .button(rust_i18n::t!("modules.case_transform"))
                            .on_hover_text(rust_i18n::t!("tooltips.case_transform"))
                            .clicked()
                        {
                            self.pipeline.add_module("case_transform");
                        }
                        if ui
                            .button(rust_i18n::t!("modules.numeral"))
                            .on_hover_text(rust_i18n::t!("tooltips.numeral"))
                            .clicked()
                        {
                            self.pipeline.add_module("numeral");
                        }
                        if ui
                            .button(rust_i18n::t!("modules.bitwise"))
                            .on_hover_text(rust_i18n::t!("tooltips.bitwise"))
                            .clicked()
                        {
                            self.pipeline.add_module("bitwise");
                        }
                    });

                egui::CollapsingHeader::new("Alphabets")
                    .default_open(false)
                    .show(ui, |ui| {
                        if ui
                            .button(rust_i18n::t!("modules.morse"))
                            .on_hover_text(rust_i18n::t!("tooltips.morse"))
                            .clicked()
                        {
                            self.pipeline.add_module("morse");
                        }
                        if ui
                            .button(rust_i18n::t!("modules.spelling"))
                            .on_hover_text(rust_i18n::t!("tooltips.spelling"))
                            .clicked()
                        {
                            self.pipeline.add_module("spelling");
                        }
                    });

                egui::CollapsingHeader::new("Ciphers")
                    .default_open(false)
                    .show(ui, |ui| {
                        // Enigma placeholder if implemented, otherwise skip or add TODO
                        // if ui.button(rust_i18n::t!("modules.enigma")).clicked() { self.pipeline.add_module("enigma"); }
                        if ui
                            .button(rust_i18n::t!("modules.caesar"))
                            .on_hover_text(rust_i18n::t!("tooltips.caesar"))
                            .clicked()
                        {
                            self.pipeline.add_module("caesar");
                        }
                        if ui
                            .button(rust_i18n::t!("modules.affine"))
                            .on_hover_text(rust_i18n::t!("tooltips.affine"))
                            .clicked()
                        {
                            self.pipeline.add_module("affine");
                        }
                        if ui
                            .button(rust_i18n::t!("modules.rot13"))
                            .on_hover_text(rust_i18n::t!("tooltips.rot13"))
                            .clicked()
                        {
                            self.pipeline.add_module("rot13");
                        }
                        if ui
                            .button(rust_i18n::t!("modules.a1z26"))
                            .on_hover_text(rust_i18n::t!("tooltips.a1z26"))
                            .clicked()
                        {
                            self.pipeline.add_module("a1z26");
                        }
                        if ui
                            .button(rust_i18n::t!("modules.vigenere"))
                            .on_hover_text(rust_i18n::t!("tooltips.vigenere"))
                            .clicked()
                        {
                            self.pipeline.add_module("vigenere");
                        }
                        if ui
                            .button(rust_i18n::t!("modules.bacon"))
                            .on_hover_text(rust_i18n::t!("tooltips.bacon"))
                            .clicked()
                        {
                            self.pipeline.add_module("bacon");
                        }
                        if ui
                            .button(rust_i18n::t!("modules.substitution"))
                            .on_hover_text(rust_i18n::t!("tooltips.substitution"))
                            .clicked()
                        {
                            self.pipeline.add_module("substitution");
                        }
                        if ui
                            .button(rust_i18n::t!("modules.rail_fence"))
                            .on_hover_text(rust_i18n::t!("tooltips.rail_fence"))
                            .clicked()
                        {
                            self.pipeline.add_module("rail_fence");
                        }
                    });

                egui::CollapsingHeader::new("Polybius Square Ciphers")
                    .default_open(false)
                    .show(ui, |ui| {
                        if ui
                            .button(rust_i18n::t!("modules.polybius"))
                            .on_hover_text(rust_i18n::t!("tooltips.polybius"))
                            .clicked()
                        {
                            self.pipeline.add_module("polybius");
                        }
                        if ui
                            .button(rust_i18n::t!("modules.tap_code"))
                            .on_hover_text(rust_i18n::t!("tooltips.tap_code"))
                            .clicked()
                        {
                            self.pipeline.add_module("tap_code");
                        }
                        // Placeholders for others
                        // if ui.button(rust_i18n::t!("modules.adfgx")).clicked() { self.pipeline.add_module("adfgx"); }
                    });

                egui::CollapsingHeader::new("Encoding")
                    .default_open(false)
                    .show(ui, |ui| {
                        if ui
                            .button(rust_i18n::t!("modules.base32"))
                            .on_hover_text(rust_i18n::t!("tooltips.base32"))
                            .clicked()
                        {
                            self.pipeline.add_module("base32");
                        }
                        if ui
                            .button(rust_i18n::t!("modules.base64"))
                            .on_hover_text(rust_i18n::t!("tooltips.base64"))
                            .clicked()
                        {
                            self.pipeline.add_module("base64");
                        }
                        if ui
                            .button(rust_i18n::t!("modules.ascii85"))
                            .on_hover_text(rust_i18n::t!("tooltips.ascii85"))
                            .clicked()
                        {
                            self.pipeline.add_module("ascii85");
                        }
                        if ui
                            .button(rust_i18n::t!("modules.baudot"))
                            .on_hover_text(rust_i18n::t!("tooltips.baudot"))
                            .clicked()
                        {
                            self.pipeline.add_module("baudot");
                        }
                        if ui
                            .button(rust_i18n::t!("modules.unicode"))
                            .on_hover_text(rust_i18n::t!("tooltips.unicode"))
                            .clicked()
                        {
                            self.pipeline.add_module("unicode");
                        }
                        if ui
                            .button(rust_i18n::t!("modules.url"))
                            .on_hover_text(rust_i18n::t!("tooltips.url"))
                            .clicked()
                        {
                            self.pipeline.add_module("url");
                        }
                        if ui
                            .button(rust_i18n::t!("modules.punycode"))
                            .on_hover_text(rust_i18n::t!("tooltips.punycode"))
                            .clicked()
                        {
                            self.pipeline.add_module("punycode");
                        }
                        if ui
                            .button(rust_i18n::t!("modules.bootstring"))
                            .on_hover_text(rust_i18n::t!("tooltips.bootstring"))
                            .clicked()
                        {
                            self.pipeline.add_module("bootstring");
                        }
                        if ui
                            .button(rust_i18n::t!("modules.integer"))
                            .on_hover_text(rust_i18n::t!("tooltips.integer"))
                            .clicked()
                        {
                            self.pipeline.add_module("integer");
                        }
                    });

                egui::CollapsingHeader::new("Modern Cryptography")
                    .default_open(false)
                    .show(ui, |ui| {
                        if ui
                            .button(rust_i18n::t!("modules.block_cipher"))
                            .on_hover_text(rust_i18n::t!("tooltips.block_cipher"))
                            .clicked()
                        {
                            self.pipeline.add_module("block_cipher");
                        }
                        if ui
                            .button(rust_i18n::t!("modules.rc4"))
                            .on_hover_text(rust_i18n::t!("tooltips.rc4"))
                            .clicked()
                        {
                            self.pipeline.add_module("rc4");
                        }
                        if ui
                            .button(rust_i18n::t!("modules.hash"))
                            .on_hover_text(rust_i18n::t!("tooltips.hash"))
                            .clicked()
                        {
                            self.pipeline.add_module("hash");
                        }
                        if ui
                            .button(rust_i18n::t!("modules.hmac"))
                            .on_hover_text(rust_i18n::t!("tooltips.hmac"))
                            .clicked()
                        {
                            self.pipeline.add_module("hmac");
                        }
                    });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                self.pipeline.ui(ui);
            });
        });

        if self.show_settings {
            egui::Window::new("Settings")
                .open(&mut self.show_settings)
                .show(ctx, |ui| {
                    ui.heading("Language");
                    egui::ComboBox::from_label("Select Language")
                        .selected_text(match self.current_lang.as_str() {
                            "en" => "English",
                            "zh-CN" => "中文 (Simplified)",
                            _ => "Unknown",
                        })
                        .show_ui(ui, |ui| {
                            if ui
                                .selectable_value(
                                    &mut self.current_lang,
                                    "en".to_string(),
                                    "English",
                                )
                                .clicked()
                            {
                                rust_i18n::set_locale("en");
                            }
                            if ui
                                .selectable_value(
                                    &mut self.current_lang,
                                    "zh-CN".to_string(),
                                    "中文 (Simplified)",
                                )
                                .clicked()
                            {
                                rust_i18n::set_locale("zh-CN");
                            }
                        });
                });
        }
    }
}
