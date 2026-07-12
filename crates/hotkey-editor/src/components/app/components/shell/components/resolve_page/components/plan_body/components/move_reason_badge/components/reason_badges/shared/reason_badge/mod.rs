mod model;
mod view;

pub use view::ReasonBadgeView;
mod style;

use dioxus::prelude::*;
use model::ReasonBadgeModel;
use style::CLASS;
use tw_macro::assert_component;

/// The base reason badge: the colour-coded pill itself, and the single owner of the pill
/// look. It is the most-nested leaf — its `style.rs` is private and never re-exported, so
/// no other component can name or reuse these classes. Each reason reuses it by
/// composition: a per-reason wrapper publishes `--reason-color` and renders `ReasonBadge`.
#[component]
pub fn ReasonBadge(props: ReasonBadgeModel) -> Element {
    let label = props.label;
    rsx! {
        span {
            class: CLASS,
            {label}
        }
    }
}

assert_component!(ReasonBadge);
