use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct SearchDialogBarView {
    pub value: ReadSignal<String>,
    pub placeholder: &'static str,
    pub on_input: EventHandler<FormEvent>,
    pub on_keydown: EventHandler<KeyboardEvent>,
    pub active_filter_count: usize,
    pub panel_open: bool,
    pub on_toggle_panel: EventHandler<MouseEvent>,
}

impl ddd::Layered for SearchDialogBarView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for SearchDialogBarView {}
