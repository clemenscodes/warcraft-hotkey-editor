pub mod components;
mod logic;
mod props;

use super::stat_figure::StatFigure;
use components::active_stat_value::{ActiveStatValue, ActiveStatValueProps};
use components::muted_stat_value::{MutedStatValue, MutedStatValueProps};
use dioxus::prelude::*;
pub use props::StatValueProps;
use tw_macro::assert_component;
assert_component!(StatValue);

/// A stat row's value in the default treatment: the domain figure presented as
/// tabular, right-aligned text. A thin dispatcher — the figure reports whether it is
/// muted and this leaf renders the active look (`ActiveStatValue`) xor the muted look
/// (`MutedStatValue`), each built by `From`. Rows with a distinctive value (hit
/// points' green, mana's blue) render their own span instead.
#[component]
pub fn StatValue<Figure: StatFigure>(props: StatValueProps<Figure>) -> Element {
    let is_muted = props.value.is_muted();
    if is_muted {
        let muted = MutedStatValueProps::from(&props);
        rsx! {
            MutedStatValue { ..muted }
        }
    } else {
        let active = ActiveStatValueProps::from(&props);
        rsx! {
            ActiveStatValue { ..active }
        }
    }
}
