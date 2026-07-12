use super::view::ReasonBadgeView;
use dioxus::prelude::*;

/// The reason pill's label text. Its colour is not here — it is the `--reason-color`
/// custom property the composing per-reason wrapper publishes.
#[derive(Props, Clone, PartialEq)]
pub struct ReasonBadgeModel {
    #[props(into)]
    pub label: String,
}

impl From<&ReasonBadgeView> for ReasonBadgeModel {
    fn from(view: &ReasonBadgeView) -> Self {
        let ReasonBadgeView { label } = view.clone();
        Self { label }
    }
}

impl ddd::Model for ReasonBadgeModel {
    type View = ReasonBadgeView;
}
