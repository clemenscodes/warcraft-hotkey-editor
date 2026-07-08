mod props;
mod style;

use super::stat_figure::StatFigure;
use dioxus::prelude::*;
pub use props::StatValueProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(StatValue);

/// A stat row's value in the default treatment: the domain figure presented as
/// tabular, right-aligned text, dimmed when the figure reports itself muted. The
/// figure formats itself through [`StatFigure`]; this leaf only places it. Rows with a
/// distinctive value (hit points' green, mana's blue) render their own span instead.
#[component]
pub fn StatValue<Figure: StatFigure>(props: StatValueProps<Figure>) -> Element {
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
