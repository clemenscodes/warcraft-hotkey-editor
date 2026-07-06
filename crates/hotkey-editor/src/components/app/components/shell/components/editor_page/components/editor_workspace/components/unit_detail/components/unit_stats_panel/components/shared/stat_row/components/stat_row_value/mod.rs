mod props;
mod style;

use super::super::stat_figure::StatFigure;
use dioxus::prelude::*;
pub use props::StatRowValueProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(StatRowValue);

/// A stat row's value; hp/mana colour comes from the parent row group, and the
/// figure's own muted state (a mana of zero) dims it.
#[component]
pub fn StatRowValue<Figure: StatFigure>(props: StatRowValueProps<Figure>) -> Element {
    let value = props.value;
    let is_muted = value.is_muted();
    let text = value.display();
    rsx! {
        span {
            class: CLASS,
            "data-zero": is_muted,
            {text}
        }
    }
}
