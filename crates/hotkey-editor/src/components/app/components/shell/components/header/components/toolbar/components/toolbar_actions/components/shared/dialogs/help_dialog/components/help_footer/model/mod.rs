use super::view::HelpFooterView;
use dioxus::prelude::*;

/// The footer's only input: the dismiss handler fired by the button, handed down by the
/// dialog that owns the open signal. The region carries it as a `Callback` (for `Default`);
/// the button below takes an `EventHandler`, so it is adapted here at the boundary.
#[derive(Props, Clone, PartialEq)]
pub struct HelpFooterModel {
    pub on_dismiss: EventHandler<MouseEvent>,
}

impl From<&HelpFooterView> for HelpFooterModel {
    fn from(view: &HelpFooterView) -> Self {
        let dismiss = view.on_dismiss;
        let on_dismiss = EventHandler::new(move |event: MouseEvent| dismiss.call(event));
        Self { on_dismiss }
    }
}

impl ddd::Model for HelpFooterModel {
    type View = HelpFooterView;
}
