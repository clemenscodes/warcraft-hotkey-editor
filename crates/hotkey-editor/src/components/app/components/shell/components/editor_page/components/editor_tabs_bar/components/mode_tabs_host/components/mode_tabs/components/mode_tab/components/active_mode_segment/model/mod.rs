use super::view::ActiveModeSegmentView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ActiveModeSegmentModel {
    pub label: &'static str,
    pub onclick: EventHandler<MouseEvent>,
    pub onkeydown: EventHandler<KeyboardEvent>,
}

impl From<&ActiveModeSegmentView> for ActiveModeSegmentModel {
    fn from(view: &ActiveModeSegmentView) -> Self {
        let ActiveModeSegmentView {
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

impl ddd::Model for ActiveModeSegmentModel {
    type View = ActiveModeSegmentView;
}
