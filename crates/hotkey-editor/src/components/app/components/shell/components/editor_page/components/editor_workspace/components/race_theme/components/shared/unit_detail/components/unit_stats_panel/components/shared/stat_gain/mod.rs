pub mod components;
mod model;
mod view;
pub use view::StatGainView;

use super::stat_figure::StatFigure;
use components::active_stat_gain::ActiveStatGain;
use components::muted_stat_gain::MutedStatGain;
use dioxus::prelude::*;
use model::StatGainModel;
use tw_macro::assert_component;

#[component]
pub fn StatGain<Figure: StatFigure>(props: StatGainModel<Figure>) -> Element {
    let value = props.value;
    let is_muted = value.is_muted();
    let text = value.display();
    if is_muted {
        rsx! {
            MutedStatGain {
                text,
            }
        }
    } else {
        rsx! {
            ActiveStatGain {
                text,
            }
        }
    }
}

assert_component!(StatGain);
