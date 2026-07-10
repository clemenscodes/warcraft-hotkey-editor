mod components;
mod logic;
mod props;

use super::stat_figure::StatFigure;
use components::active_stat_gain::{ActiveStatGain, ActiveStatGainProps};
use components::muted_stat_gain::{MutedStatGain, MutedStatGainProps};
use dioxus::prelude::*;
pub use props::StatGainProps;
use tw_macro::assert_component;
assert_component!(StatGain);

/// A stat row's per-level growth in the default treatment: green, tabular text sitting
/// inline after the value. A thin dispatcher — the figure reports whether it is muted
/// and this leaf renders the active look (`ActiveStatGain`) xor the muted look
/// (`MutedStatGain`), each built by `From`.
#[component]
pub fn StatGain<Figure: StatFigure>(props: StatGainProps<Figure>) -> Element {
    let is_muted = props.value.is_muted();
    if is_muted {
        let muted = MutedStatGainProps::from(&props);
        rsx! {
            MutedStatGain { ..muted }
        }
    } else {
        let active = ActiveStatGainProps::from(&props);
        rsx! {
            ActiveStatGain { ..active }
        }
    }
}
