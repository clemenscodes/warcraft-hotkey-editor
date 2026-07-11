use super::view::InteractiveSurfaceView;
use dioxus::prelude::*;

/// The interactive resting look of a toolbar surface: muted text at rest that
/// brightens to gold on hover. Carries every attribute needed to render the
/// `<button>`, including the icon glyph it draws. Built by the dispatcher from the
/// shared toolbar surface state.
#[derive(Props, Clone, PartialEq)]
pub struct InteractiveSurfaceModel {
    pub icon: &'static str,
    pub aria_label: &'static str,
    pub aria_haspopup: Option<&'static str>,
    pub aria_expanded: Option<bool>,
    pub aria_pressed: Option<bool>,
    pub disabled: bool,
    pub onclick: EventHandler<MouseEvent>,
}

impl From<&InteractiveSurfaceView> for InteractiveSurfaceModel {
    fn from(view: &InteractiveSurfaceView) -> Self {
        let InteractiveSurfaceView {
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

impl ddd::Model for InteractiveSurfaceModel {
    type View = InteractiveSurfaceView;
}
