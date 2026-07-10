use dioxus::prelude::*;

/// The clear resting look of a toolbar surface: a gold-bordered surface with a soft
/// resting glow, the affirmative "all clear" look. Carries every attribute needed to
/// render the `<button>`, including the icon glyph it draws. Built by the dispatcher
/// from the shared toolbar surface state.
#[derive(Props, Clone, PartialEq)]
pub struct ClearSurfaceProps {
    pub icon: &'static str,
    pub aria_label: &'static str,
    pub aria_haspopup: Option<&'static str>,
    pub aria_expanded: Option<bool>,
    pub aria_pressed: Option<bool>,
    pub disabled: bool,
    pub onclick: EventHandler<MouseEvent>,
}
