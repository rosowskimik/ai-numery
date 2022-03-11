use std::fmt::Display;

use eframe::{
    egui::{InnerResponse, Layout, Response, RichText, Ui},
    emath::Align,
    epaint::Color32,
};

pub fn centered_label(ui: &mut Ui, text: impl Into<String>) -> Response {
    let text = RichText::new(text)
        .size(30.0)
        .color(if ui.visuals().dark_mode {
            Color32::WHITE
        } else {
            Color32::BLACK
        });
    ui.with_layout(Layout::top_down(Align::Center), |ui| ui.label(text))
        .response
}

pub fn control_button(ui: &mut Ui, text: impl Into<String>) -> Response {
    let text = RichText::new(text)
        .size(26.0)
        .color(if ui.visuals().dark_mode {
            Color32::WHITE
        } else {
            Color32::BLACK
        });
    ui.spacing_mut().button_padding *= 2.5;
    let res = ui.button(text);
    ui.reset_style();

    res
}

pub fn select_list<T>(
    ui: &mut Ui,
    layout: Layout,
    target: &mut T,
    entries: &mut dyn Iterator<Item = T>,
) -> InnerResponse<Vec<Response>>
where
    T: Display + PartialEq,
{
    ui.with_layout(layout, |ui| {
        entries
            .map(|entry| {
                let text = RichText::new(format!("{}", entry)).size(26.0).color(
                    if ui.visuals().dark_mode {
                        Color32::WHITE
                    } else {
                        Color32::BLACK
                    },
                );
                ui.selectable_value(target, entry, text)
            })
            .collect()
    })
}
