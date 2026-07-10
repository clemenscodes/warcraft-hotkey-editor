use super::view::ModeTabView;
use dioxus::prelude::*;

/// One mode button: the label it shows, whether it is the active mode, and the
/// pointer and keyboard activation handlers.
#[derive(Props, Clone, PartialEq)]
pub struct ModeTabProps {
    pub label: &'static str,
    pub active: bool,
    pub onclick: EventHandler<MouseEvent>,
    pub onkeydown: EventHandler<KeyboardEvent>,
}

impl From<&ModeTabView> for ModeTabProps {
    fn from(view: &ModeTabView) -> Self {
        let ModeTabView {
            label,
            active,
            onclick,
            onkeydown,
        } = view.clone();
        Self {
            label,
            active,
            onclick,
            onkeydown,
        }
    }
}

impl ddd::Props for ModeTabProps {
    type View = ModeTabView;
}
