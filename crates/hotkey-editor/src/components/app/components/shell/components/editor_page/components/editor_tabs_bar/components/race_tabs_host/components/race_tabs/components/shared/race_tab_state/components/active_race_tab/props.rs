use super::view::ActiveRaceTabView;
use dioxus::prelude::*;

/// The active variant's props: the tab's display name and handlers, forwarded to the
/// `RaceTab` it composes. The active look is added on top (the accent overlay and the
/// `--label-color` its root publishes), never by changing these.
#[derive(Props, Clone, PartialEq)]
pub struct ActiveRaceTabProps {
    pub label: String,
    pub onclick: EventHandler<MouseEvent>,
    pub onkeydown: EventHandler<KeyboardEvent>,
}

impl From<&ActiveRaceTabView> for ActiveRaceTabProps {
    fn from(view: &ActiveRaceTabView) -> Self {
        let ActiveRaceTabView {
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

impl ddd::Props for ActiveRaceTabProps {
    type View = ActiveRaceTabView;
}
