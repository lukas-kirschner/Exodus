use crate::ui::UIMARGIN;
use bevy_egui::egui::style::Spacing;
use bevy_egui::egui::{Color32, Context, FontData, FontFamily, FontId, Rounding, Visuals};
use bevy_egui::{egui, EguiContexts};
use font_kit::family_name::FamilyName;
use font_kit::properties::{Properties, Style, Weight};
use font_kit::source::SystemSource;
use font_kit::sources::fontconfig::FontconfigSource;
use std::collections::BTreeMap;
use std::sync::Arc;

const SANSSERIF_NORMAL: &str = "SansSerif";
const SANSSERIF_BOLD: &str = "SansSerifBold";
const SANSSERIF_ITALIC: &str = "SansSerifItalic";

/// Insert a single system font or panic, if the font cannot be found
fn insert_system_font(
    source: &FontconfigSource,
    family: &[FamilyName],
    properties: &Properties,
    font_data: &mut BTreeMap<String, Arc<FontData>>,
    name: String,
) {
    let font = source
        .select_best_match(family, properties)
        .unwrap()
        .load()
        .unwrap();
    font_data.insert(
        name,
        Arc::new(FontData::from_owned(
            (*font.copy_font_data().expect("Could not copy font data")).clone(),
        )),
    );
}

/// Load the fallback fonts in case font loading goes wrong later
fn load_system_fonts(ctx: &mut Context, source: &FontconfigSource) {
    fn sys(s: &str) -> String {
        format!("{}-System", s)
    }
    // For an example, see https://github.com/emilk/egui/discussions/2169
    let mut fonts = egui::FontDefinitions::default();

    // Init Fallback Fonts
    insert_system_font(
        source,
        &[FamilyName::SansSerif],
        &Properties::new(),
        &mut fonts.font_data,
        sys(SANSSERIF_NORMAL),
    );
    insert_system_font(
        source,
        &[FamilyName::SansSerif],
        Properties::new().weight(Weight::BOLD),
        &mut fonts.font_data,
        sys(SANSSERIF_BOLD),
    );
    insert_system_font(
        source,
        &[FamilyName::SansSerif],
        Properties::new().style(Style::Italic),
        &mut fonts.font_data,
        sys(SANSSERIF_ITALIC),
    );
    fonts
        .families
        .entry(FontFamily::Proportional)
        .or_default()
        .insert(0, sys(SANSSERIF_NORMAL));
    fonts
        .families
        .entry(FontFamily::Name(Arc::from(SANSSERIF_NORMAL)))
        .or_default()
        .insert(0, sys(SANSSERIF_NORMAL));
    fonts
        .families
        .entry(FontFamily::Name(Arc::from(SANSSERIF_BOLD)))
        .or_default()
        .insert(0, sys(SANSSERIF_BOLD));
    fonts
        .families
        .entry(FontFamily::Name(Arc::from(SANSSERIF_ITALIC)))
        .or_default()
        .insert(0, sys(SANSSERIF_ITALIC));
    ctx.set_fonts(fonts);
}

/// Load the specific fonts, if they exist
fn load_specific_fonts(_ctx: &mut Context, _source: &FontconfigSource) {
    // For an example, see https://github.com/emilk/egui/discussions/2169
    // let mut fonts = egui::FontDefinitions::default();
    //
    // let font_to_try = if cfg!(windows) {
    //     "Segoe UI".to_owned()
    // } else {
    //     std::process::Command::new("gsettings")
    //         .args(["get", "org.gnome.desktop.interface", "font-name"])
    //         .output()
    //         .and_then(|o| {
    //             String::from_utf8(o.stdout).map_err(|e| {
    //                 error!("Could not query system font via gsettings: {}", e);
    //                 std::io::Error::new(std::io::ErrorKind::Other, "")
    //             })
    //         })
    //         .unwrap_or_else(|_| "Ubuntu".to_owned())
    // };
    // if let Ok(system_font) = SystemSource::new().select_best_match(&font_to_try) {
    //     fonts
    //         .font_data
    //         .insert("System".to_owned(), FontData::from_owned(system_font..0));
    // }
    // ctx.set_fonts(fonts);
}

pub fn egui_fonts(mut ctx: EguiContexts) {
    let source = SystemSource::new();
    load_system_fonts(ctx.ctx_mut(), &source);
    load_specific_fonts(ctx.ctx_mut(), &source);
    let mut style = (*ctx.ctx_mut().style()).clone();
    style.text_styles = [
        (
            egui::TextStyle::Heading,
            FontId::new(30.0, FontFamily::Name(Arc::from(SANSSERIF_NORMAL))),
        ),
        (
            // The huge title text shown in the main menu
            egui::TextStyle::Name("MainMenuGameTitle".into()),
            FontId::new(50.0, FontFamily::Name(Arc::from(SANSSERIF_BOLD))),
        ),
        (
            egui::TextStyle::Name("Context".into()),
            FontId::new(23.0, FontFamily::Name(Arc::from(SANSSERIF_NORMAL))),
        ),
        (
            // Style for Highscores
            egui::TextStyle::Name("Highscore".into()),
            FontId::new(14.0, FontFamily::Name(Arc::from(SANSSERIF_ITALIC))),
        ),
        (
            // Style for Map Titles
            egui::TextStyle::Name("MapTitle".into()),
            FontId::new(22.0, FontFamily::Name(Arc::from(SANSSERIF_NORMAL))),
        ),
        (
            // Style for Map Authors
            egui::TextStyle::Name("MapAuthor".into()),
            FontId::new(18.0, FontFamily::Name(Arc::from(SANSSERIF_ITALIC))),
        ),
        (
            // Style for Subheadings (to separate sections of dialogs)
            egui::TextStyle::Name("Subheading".into()),
            FontId::new(18.0, FontFamily::Name(Arc::from(SANSSERIF_NORMAL))),
        ),
        (
            // Style for Subheadings (to separate sections of dialogs)
            egui::TextStyle::Name("DialogText".into()),
            FontId::new(16.0, FontFamily::Name(Arc::from(SANSSERIF_NORMAL))),
        ),
        (
            // Style for Descriptive UI Texts
            egui::TextStyle::Name("Description".into()),
            FontId::new(16.0, FontFamily::Name(Arc::from(SANSSERIF_ITALIC))),
        ),
        (
            egui::TextStyle::Body,
            FontId::new(18.0, FontFamily::Name(Arc::from(SANSSERIF_NORMAL))),
        ),
        (
            egui::TextStyle::Monospace,
            FontId::new(16.0, FontFamily::Name(Arc::from(SANSSERIF_NORMAL))),
        ),
        (
            egui::TextStyle::Button,
            FontId::new(20.0, FontFamily::Name(Arc::from(SANSSERIF_NORMAL))),
        ),
        (
            egui::TextStyle::Small,
            FontId::new(10.0, FontFamily::Name(Arc::from(SANSSERIF_NORMAL))),
        ),
    ]
    .into();
    style.spacing = Spacing::default();
    style.spacing.item_spacing = (2.0 * UIMARGIN, UIMARGIN).into();
    ctx.ctx_mut().set_style(style);
}
pub fn egui_visuals(mut ctx: EguiContexts) {
    let mut visuals = Visuals::dark();
    visuals.striped = true;
    visuals.faint_bg_color = Color32::from_rgb(
        visuals.panel_fill.r() + 15,
        visuals.panel_fill.g() + 15,
        visuals.panel_fill.b() + 15,
    );
    ctx.ctx_mut().set_visuals(visuals);
}
