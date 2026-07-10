pub mod components;
mod props;
mod view;
pub use view::StatValueView;

use super::stat_figure::StatFigure;
use components::active_stat_value::ActiveStatValue;
use components::muted_stat_value::MutedStatValue;
use dioxus::prelude::*;
use props::StatValueProps;
use tw_macro::assert_component;

/// A stat row's value in the default treatment: the domain figure presented as
/// tabular, right-aligned text. A thin dispatcher — the figure reports whether it is
/// muted and this leaf renders the active look (`ActiveStatValue`) xor the muted look
/// (`MutedStatValue`), each built by `From`. Rows with a distinctive value (hit
/// points' green, mana's blue) render their own span instead.
#[component]
pub fn StatValue<Figure: StatFigure>(props: StatValueProps<Figure>) -> Element {
    let value = props.value;
    let is_muted = value.is_muted();
    let text = value.display();
    if is_muted {
        rsx! {
            MutedStatValue { text }
        }
    } else {
        rsx! {
            ActiveStatValue { text }
        }
    }
}

assert_component!(StatValue);
