use super::view::IdleModeSegmentView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct IdleModeSegmentModel {
    pub label: &'static str,
    pub onclick: EventHandler<MouseEvent>,
    pub onkeydown: EventHandler<KeyboardEvent>,
}

impl From<&IdleModeSegmentView> for IdleModeSegmentModel {
    fn from(view: &IdleModeSegmentView) -> Self {
        let IdleModeSegmentView {
            label,
            onclick,
            onkeydown,
        } = view.clone();
        Self {
            label,
            onclick,
            onkeydown,
        }
    }
}

impl ddd::Model for IdleModeSegmentModel {
    type View = IdleModeSegmentView;
}
