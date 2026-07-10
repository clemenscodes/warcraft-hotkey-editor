use super::view::AttentionSurfaceView;
use dioxus::prelude::*;

/// The attention resting look of a toolbar surface: a persistently gold surface used
/// when the button is surfacing a condition that needs the user's eye. Carries every
/// attribute needed to render the `<button>`, including the icon glyph it draws. Built
/// by the dispatcher from the shared toolbar surface state.
#[derive(Props, Clone, PartialEq)]
pub struct AttentionSurfaceProps {
    pub icon: &'static str,
    pub aria_label: &'static str,
    pub aria_haspopup: Option<&'static str>,
    pub aria_expanded: Option<bool>,
    pub aria_pressed: Option<bool>,
    pub disabled: bool,
    pub onclick: EventHandler<MouseEvent>,
}

impl From<&AttentionSurfaceView> for AttentionSurfaceProps {
    fn from(view: &AttentionSurfaceView) -> Self {
        let AttentionSurfaceView {
            icon,
            aria_label,
            aria_haspopup,
            aria_expanded,
            aria_pressed,
            disabled,
            onclick,
        } = view.clone();
        Self {
            icon,
            aria_label,
            aria_haspopup,
            aria_expanded,
            aria_pressed,
            disabled,
            onclick,
        }
    }
}

impl ddd::Props for AttentionSurfaceProps {
    type View = AttentionSurfaceView;
}
