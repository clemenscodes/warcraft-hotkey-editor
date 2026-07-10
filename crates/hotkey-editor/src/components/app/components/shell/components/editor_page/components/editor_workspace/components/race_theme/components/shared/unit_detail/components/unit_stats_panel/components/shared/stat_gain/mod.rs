mod props;
mod style;

use super::stat_figure::StatFigure;
use dioxus::prelude::*;
pub use props::StatGainProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(StatGain);

/// A stat row's per-level growth in the default treatment: green, tabular text sitting
/// inline after the value, dimmed when the figure reports itself muted. The figure
/// formats itself through [`StatFigure`]; this leaf only places it.
#[component]
pub fn StatGain<Figure: StatFigure>(props: StatGainProps<Figure>) -> Element {
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
