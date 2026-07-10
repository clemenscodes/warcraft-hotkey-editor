use crate::components::app::components::shell::components::shared::editable_keycap::EditableKeycapState;
use dioxus::prelude::*;

/// The multi-character override key's input: the gold-cap glyph and its capture state, the
/// button title, and the activation handler. Set by the `OverrideKey` dispatcher.
#[derive(Props, Clone, PartialEq)]
pub struct SpecialOverrideKeyProps {
    #[props(into)]
    pub label: String,
    pub state: EditableKeycapState,
    #[props(into)]
    pub title: String,
    pub on_activate: EventHandler<()>,
}
