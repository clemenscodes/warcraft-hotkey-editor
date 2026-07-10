use crate::components::app::components::shell::components::shared::editable_keycap::EditableKeycapProps;
use dioxus::prelude::*;

/// The multi-character override key's input: the already-shaped gold-cap child props, the
/// button title, and the activation handler. Built by the `OverrideKey` dispatcher from
/// `OverrideKeyProps`.
#[derive(Props, Clone, PartialEq)]
pub struct SpecialOverrideKeyProps {
    pub keycap: EditableKeycapProps,
    #[props(into)]
    pub title: String,
    pub on_activate: EventHandler<()>,
}
