mod props;
mod style;

use dioxus::prelude::*;
use style::CLASS;
use tw_macro::assert_component;

pub use props::EditingKeycapProps;

assert_component!(EditingKeycap);

/// The pulsing gold key-cap surface: the editable keycap's `Editing` look, lit while its
/// key picker is open. Presentational — the host owns size, focus, drag, and the click
/// handler; this leaf draws the whole pulsing cap with the corner radius the host selects
/// through `data-radius`.
#[component]
pub fn EditingKeycap(props: EditingKeycapProps) -> Element {
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
