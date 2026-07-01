use dioxus::prelude::*;

/// The Apply button that runs the cascade; disabled and labelled "Applying…"
/// while a run is in flight.
#[derive(Props, Clone, PartialEq)]
pub struct ResolveApplyButtonProps {
    pub running: bool,
    pub onclick: EventHandler<MouseEvent>,
}
