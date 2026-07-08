mod props;
mod style;

use dioxus::prelude::*;
use style::CLASS;
use tw_macro::assert_component;

pub use props::IdleKeycapProps;

assert_component!(IdleKeycap);

/// The resting gold key-cap surface: the editable keycap's `Idle` look. Presentational —
/// the host owns size, focus, drag, and the click handler; this leaf draws the whole
/// resting cap with the corner radius the host selects through `data-radius`.
#[component]
pub fn IdleKeycap(props: IdleKeycapProps) -> Element {
    let radius = props.radius;
    let label = props.label;
    rsx! {
        div {
            class: CLASS,
            "data-radius": "{radius}",
            {label}
        }
    }
}
