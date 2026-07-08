mod props;
mod style;

use super::super::super::shared::stat_figure::StatFigure;
use dioxus::prelude::*;
pub use props::HitPointsRowProps;
use style::{CLASS, LABEL, VALUE};
use tw_macro::assert_component;
assert_component!(HitPointsRow);

const LABEL_TEXT: &str = "Hit Points";

/// The unit's hit points: the vitality column's headline figure. Green and enlarged —
/// the row wears that treatment directly rather than selecting it through a shared
/// variant flag. Hit points are never muted, so the value renders plainly.
#[component]
pub fn HitPointsRow(props: HitPointsRowProps) -> Element {
    let value = props.value;
    let text = value.display();
    rsx! {
        div {
            class: CLASS,
            span { class: LABEL, {LABEL_TEXT} }
            span { class: VALUE, {text} }
        }
    }
}
