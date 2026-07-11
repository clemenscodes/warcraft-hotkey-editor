use super::view::ClearSurfaceView;
use dioxus::prelude::*;

/// The clear resting look of a toolbar surface: a gold-bordered surface with a soft
/// resting glow, the affirmative "all clear" look. Carries every attribute needed to
/// render the `<button>`, including the icon glyph it draws. Built by the dispatcher
/// from the shared toolbar surface state.
#[derive(Props, Clone, PartialEq)]
pub struct ClearSurfaceModel {
    pub icon: &'static str,
    pub aria_label: &'static str,
    pub aria_haspopup: Option<&'static str>,
    pub aria_expanded: Option<bool>,
    pub aria_pressed: Option<bool>,
    pub disabled: bool,
    pub onclick: EventHandler<MouseEvent>,
}

impl From<&ClearSurfaceView> for ClearSurfaceModel {
    fn from(view: &ClearSurfaceView) -> Self {
        let ClearSurfaceView {
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

impl ddd::Model for ClearSurfaceModel {
    type View = ClearSurfaceView;
}
