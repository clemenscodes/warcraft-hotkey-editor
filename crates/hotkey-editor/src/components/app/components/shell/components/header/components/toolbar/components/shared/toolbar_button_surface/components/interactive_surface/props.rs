use dioxus::prelude::*;

/// The interactive resting look of a toolbar surface: muted text at rest that
/// brightens to gold on hover. Carries every attribute needed to render the
/// `<button>`, including the icon glyph it draws. Built by the dispatcher from the
/// shared toolbar surface state.
#[derive(Props, Clone, PartialEq)]
pub struct InteractiveSurfaceProps {
    pub icon: &'static str,
    pub aria_label: &'static str,
    pub aria_haspopup: Option<&'static str>,
    pub aria_expanded: Option<bool>,
    pub aria_pressed: Option<bool>,
    pub disabled: bool,
    pub onclick: EventHandler<MouseEvent>,
}
