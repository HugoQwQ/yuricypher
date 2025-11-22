use eframe::egui;

pub trait Module {
    fn name(&self) -> &str;
    fn process(&self, input: &str) -> String;
    fn ui(&mut self, ui: &mut egui::Ui);
    fn as_any(&self) -> &dyn std::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
}
