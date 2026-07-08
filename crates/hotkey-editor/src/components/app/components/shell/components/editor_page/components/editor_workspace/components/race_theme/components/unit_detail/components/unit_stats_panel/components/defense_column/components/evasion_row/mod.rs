mod props;
mod style;

use super::super::super::shared::stat_value::StatValue;
use dioxus::prelude::*;
pub use props::EvasionRowProps;
use style::{CLASS, LABEL};
use tw_macro::assert_component;
assert_component!(EvasionRow);

const LABEL_TEXT: &str = "Evasion";

/// The evasion row, shown only when the unit can dodge. A unit with no evasion source
/// has no row at all — it would otherwise print a muted "0%". A guarded leaf that
/// early-returns, asking the domain whether there is any evasion.
#[component]
pub fn EvasionRow(props: EvasionRowProps) -> Element {
    let value = props.value;
    if value.is_zero() {
        return rsx! {};
    }
    rsx! {
        div {
            class: CLASS,
            span { class: LABEL, {LABEL_TEXT} }
            StatValue { value }
        }
    }
}
