mod props;
mod state;
mod style;

use dioxus::prelude::*;
use tw_macro::assert_component;

pub use props::EditableKeycapProps;
pub use state::{EditableKeycapRadius, EditableKeycapState};

assert_component!(EditableKeycap);

/// The shared gold key-cap surface worn by both editable hotkey cells: the editor's
/// override key and the layout-grid tile. It fills the box its host button gives it and
/// draws the whole cap — gold border and fill, glyph, outline shadow, hover glow, and
/// the capture pulse — with the corner radius the host selects. It is presentational:
/// the host owns size, focus, drag, and the click handler; this leaf only renders the
/// look, so the gallery can render it with any glyph, radius, and pulse state.
#[component]
pub fn EditableKeycap(props: EditableKeycapProps) -> Element {
    let class = style::class(props.state);
    let radius = props.radius;
    let label = props.label;
    rsx! {
        div {
            class,
            "data-radius": "{radius}",
            {label}
        }
    }
}
