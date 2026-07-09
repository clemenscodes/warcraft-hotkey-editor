use dioxus::prelude::*;

/// The reason pill's label text. Its colour is not here — it is the `--reason-color`
/// custom property the composing per-reason wrapper publishes.
#[derive(Props, Clone, PartialEq)]
pub struct ReasonBadgeProps {
    #[props(into)]
    pub label: String,
}
