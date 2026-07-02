use dioxus::prelude::*;
/// The button wrapping a conflict ability's icon; clicking deep-links into the editor.
#[derive(Props, Clone, PartialEq)]
pub struct ConflictAbilityTriggerProps {
    pub onclick: EventHandler<MouseEvent>,
    pub children: Element,
}
