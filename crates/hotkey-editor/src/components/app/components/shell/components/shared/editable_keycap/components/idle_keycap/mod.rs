mod props;
mod style;

use dioxus::prelude::*;
use style::CLASS;
use tw_macro::assert_component;

pub use props::IdleKeycapProps;

assert_component!(IdleKeycap);

/// The resting gold key-cap surface: the editable keycap's `Idle` look. Presentational —
/// the host owns size, focus, drag, and the click handler; this leaf draws the whole
/// resting cap with the corner radius the host selects through the inherited
/// `--keycap-radius` (panel when unset).
#[component]
pub fn IdleKeycap(props: IdleKeycapProps) -> Element {
    let label = props.label;
    rsx! {
        div {
            class: CLASS,
            {label}
        }
    }
}
