use super::view::ApplyButtonView;
use dioxus::prelude::*;

/// The Apply button that runs the cascade; disabled and labelled "Applying…"
/// while a run is in flight.
#[derive(Props, Clone, PartialEq)]
pub struct ApplyButtonModel {
    pub running: bool,
    pub onclick: EventHandler<MouseEvent>,
}

impl From<&ApplyButtonView> for ApplyButtonModel {
    fn from(view: &ApplyButtonView) -> Self {
        let ApplyButtonView { running, onclick } = view.clone();
        Self { running, onclick }
    }
}

impl ddd::Model for ApplyButtonModel {
    type View = ApplyButtonView;
}
