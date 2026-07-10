use super::view::NormalOverrideKeyView;
use crate::components::app::components::shell::components::shared::editable_keycap::EditableKeycapState;
use dioxus::prelude::*;

/// The single-letter override key's input: the gold-cap glyph and its capture state, the
/// button title, and the activation handler. Set by the `OverrideKey` dispatcher.
#[derive(Props, Clone, PartialEq)]
pub struct NormalOverrideKeyProps {
    #[props(into)]
    pub label: String,
    pub state: EditableKeycapState,
    #[props(into)]
    pub title: String,
    pub on_activate: EventHandler<()>,
}

impl From<&NormalOverrideKeyView> for NormalOverrideKeyProps {
    fn from(view: &NormalOverrideKeyView) -> Self {
        let NormalOverrideKeyView {
            label,
            state,
            title,
            on_activate,
        } = view.clone();
        Self {
            label,
            state,
            title,
            on_activate,
        }
    }
}

impl ddd::Props for NormalOverrideKeyProps {
    type View = NormalOverrideKeyView;
}
