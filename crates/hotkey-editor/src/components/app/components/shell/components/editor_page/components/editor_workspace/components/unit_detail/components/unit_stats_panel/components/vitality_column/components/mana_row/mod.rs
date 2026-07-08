mod props;
mod style;

use super::super::super::shared::stat_figure::StatFigure;
use dioxus::prelude::*;
pub use props::ManaRowProps;
use style::{CLASS, LABEL, VALUE};
use tw_macro::assert_component;
assert_component!(ManaRow);

const LABEL_TEXT: &str = "Mana";

/// The unit's mana pool. Wears the human-blue accent directly; a unit with no mana
/// reports itself muted and the figure dims to faint.
#[component]
pub fn ManaRow(props: ManaRowProps) -> Element {
    let value = props.value;
    let is_muted = value.is_muted();
    let text = value.display();
    rsx! {
        div {
            class: CLASS,
            span { class: LABEL, {LABEL_TEXT} }
            span { class: VALUE, "data-zero": is_muted, {text} }
        }
    }
}
