use bevy_egui::egui;
use bevy_egui::egui::{Button, Response, Ui, Widget, WidgetText};

pub fn selectable_value_with_image<'a, Value: PartialEq>(
    ui: &mut Ui,
    current_value: &mut Value,
    selected_value: Value,
    image: impl Into<bevy_egui::egui::Image<'a>>,
    text: impl Into<WidgetText>,
) -> egui::response::Response {
    let mut response =
        selectable_label_with_image(ui, *current_value == selected_value, image, text);
    if response.clicked() && *current_value != selected_value {
        *current_value = selected_value;
        response.mark_changed();
    }
    response
}
fn selectable_label_with_image<'a>(
    ui: &mut Ui,
    checked: bool,
    image: impl Into<bevy_egui::egui::Image<'a>>,
    text: impl Into<WidgetText>,
) -> Response {
    selectable_image_button(checked, image, text).ui(ui)
}

pub fn selectable_image_button<'a>(
    selected: bool,
    image: impl Into<bevy_egui::egui::Image<'a>>,
    atoms: impl Into<WidgetText>,
) -> Button<'a> {
    Button::image_and_text(image, atoms)
        .selected(selected)
        .frame_when_inactive(selected)
        .frame(true)
}
