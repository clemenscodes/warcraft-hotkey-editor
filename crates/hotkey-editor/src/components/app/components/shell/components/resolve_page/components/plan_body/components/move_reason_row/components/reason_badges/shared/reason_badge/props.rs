use super::view::ReasonBadgeView;
use dioxus::prelude::*;

/// The reason pill's label text. Its colour is not here — it is the `--reason-color`
/// custom property the composing per-reason wrapper publishes.
#[derive(Props, Clone, PartialEq)]
pub struct ReasonBadgeProps {
    #[props(into)]
    pub label: String,
}

impl From<&ReasonBadgeView> for ReasonBadgeProps {
    fn from(view: &ReasonBadgeView) -> Self {
        let ReasonBadgeView { label } = view.clone();
        Self { label }
    }
}

impl ddd::Props for ReasonBadgeProps {
    type View = ReasonBadgeView;
}
