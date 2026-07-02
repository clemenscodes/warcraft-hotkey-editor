mod props;
mod style;

use super::super::stat_figure::StatFigure;
use crate::assert_component;
use dioxus::prelude::*;
pub use props::StatRowGainProps;
use style::CLASS;
assert_component!(StatRowGain);

/// A stat row's gain figure; regen alignment/colour comes from the parent row
/// group, and the figure's own muted state (a regen of zero) dims it.
#[component]
pub fn StatRowGain<Figure: StatFigure>(props: StatRowGainProps<Figure>) -> Element {
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
