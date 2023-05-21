use bevy_egui::egui::FontFamily::Proportional;
use bevy_egui::egui::FontId;
use bevy_egui::{egui, EguiContexts};

pub fn egui_fonts(mut ctx: EguiContexts) {
    let mut fonts = egui::FontDefinitions::default();
    fonts.font_data.insert(
        "exodus".to_owned(),
        egui::FontData::from_static(include_bytes!("../../assets/fonts/PublicPixel.ttf")),
    );
    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "exodus".to_owned());
    fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .push("exodus".to_owned());
    ctx.ctx_mut().set_fonts(fonts);

    let mut style = (*ctx.ctx_mut().style()).clone();
    style.text_styles = [
        (egui::TextStyle::Heading, FontId::new(30.0, Proportional)),
        (
            egui::TextStyle::Name("Heading2".into()),
            FontId::new(25.0, Proportional),
        ),
        (
            egui::TextStyle::Name("Context".into()),
            FontId::new(23.0, Proportional),
        ),
        (egui::TextStyle::Body, FontId::new(18.0, Proportional)),
        (egui::TextStyle::Monospace, FontId::new(16.0, Proportional)),
        (egui::TextStyle::Button, FontId::new(20.0, Proportional)),
        (egui::TextStyle::Small, FontId::new(10.0, Proportional)),
    ]
    .into();
    ctx.ctx_mut().set_style(style);
}
