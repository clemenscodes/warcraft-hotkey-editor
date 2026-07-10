pub mod components;
mod props;
mod view;
pub use view::StatGainView;

use super::stat_figure::StatFigure;
use components::active_stat_gain::ActiveStatGain;
use components::muted_stat_gain::MutedStatGain;
use dioxus::prelude::*;
use props::StatGainProps;
use tw_macro::assert_component;

/// A stat row's per-level growth in the default treatment: green, tabular text sitting
/// inline after the value. A thin dispatcher — the figure reports whether it is muted
/// and this leaf renders the active look (`ActiveStatGain`) xor the muted look
/// (`MutedStatGain`), each built by `From`.
#[component]
pub fn StatGain<Figure: StatFigure>(props: StatGainProps<Figure>) -> Element {
    let value = props.value;
    let is_muted = value.is_muted();
    let text = value.display();
    if is_muted {
        rsx! {
            MutedStatGain { text }
        }
    } else {
        rsx! {
            ActiveStatGain { text }
        }
    }
}

assert_component!(StatGain);
