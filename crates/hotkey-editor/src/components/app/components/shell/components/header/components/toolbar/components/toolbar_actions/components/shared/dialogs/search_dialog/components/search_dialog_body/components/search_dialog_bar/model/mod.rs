use super::view::SearchDialogBarView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct SearchDialogBarModel {
    pub value: ReadSignal<String>,
    pub placeholder: &'static str,
    pub on_input: EventHandler<FormEvent>,
    pub on_keydown: EventHandler<KeyboardEvent>,
    pub active_filter_count: usize,
    pub panel_open: bool,
    pub on_toggle_panel: EventHandler<MouseEvent>,
}

impl From<&SearchDialogBarView> for SearchDialogBarModel {
    fn from(view: &SearchDialogBarView) -> Self {
        let SearchDialogBarView {
            value,
            placeholder,
            on_input,
            on_keydown,
            active_filter_count,
            panel_open,
            on_toggle_panel,
        } = view.clone();
        Self {
            value,
            placeholder,
            on_input,
            on_keydown,
            active_filter_count,
            panel_open,
            on_toggle_panel,
        }
    }
}

impl ddd::Model for SearchDialogBarModel {
    type View = SearchDialogBarView;
}
