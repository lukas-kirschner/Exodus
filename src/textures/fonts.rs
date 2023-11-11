use bevy_egui::egui::FontDefinitions;
use bevy_egui::egui::FontFamily::{Monospace, Proportional};
use bevy_egui::egui::FontId;
use bevy_egui::{egui, EguiContexts};

pub fn egui_fonts(mut ctx: EguiContexts) {
    let mut style = (*ctx.ctx_mut().style()).clone();
    style.text_styles = [
        (egui::TextStyle::Heading, FontId::new(30.0, Proportional)),
        (
            egui::TextStyle::Name("MainMenuGameTitle".into()),
            FontId::new(50.0, Proportional),
        ),
        (
            egui::TextStyle::Name("Context".into()),
            FontId::new(23.0, Proportional),
        ),
        (egui::TextStyle::Body, FontId::new(18.0, Proportional)),
        (egui::TextStyle::Monospace, FontId::new(16.0, Monospace)),
        (egui::TextStyle::Button, FontId::new(20.0, Proportional)),
        (egui::TextStyle::Small, FontId::new(10.0, Proportional)),
    ]
    .into();
    ctx.ctx_mut().set_style(style);
}
