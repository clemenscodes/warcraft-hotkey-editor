mod props;
mod style;

use super::super::super::shared::stat_figure::StatFigure;
use dioxus::prelude::*;
pub use props::ManaRegenRowProps;
use style::{CLASS, GAIN, LABEL};
use tw_macro::assert_component;
assert_component!(ManaRegenRow);

const LABEL_TEXT: &str = "Regeneration";

/// The unit's mana regeneration: an indented companion to the mana row, wearing the
/// human-blue accent. Dimmed when the unit does not regenerate mana.
#[component]
pub fn ManaRegenRow(props: ManaRegenRowProps) -> Element {
    let value = props.value;
    let is_muted = value.is_muted();
    let text = value.display();
    rsx! {
        div {
            class: CLASS,
            span { class: LABEL, {LABEL_TEXT} }
            span { class: GAIN, "data-zero": is_muted, {text} }
        }
    }
}
