pub mod components;
mod model;
mod view;
pub use view::StatValueView;

use super::stat_figure::StatFigure;
use components::active_stat_value::ActiveStatValue;
use components::muted_stat_value::MutedStatValue;
use dioxus::prelude::*;
use model::StatValueModel;
use tw_macro::assert_component;

#[component]
pub fn StatValue<Figure: StatFigure>(props: StatValueModel<Figure>) -> Element {
    let value = props.value;
    let is_muted = value.is_muted();
    let text = value.display();
    if is_muted {
        rsx! {
            MutedStatValue {
                text,
            }
        }
    } else {
        rsx! {
            ActiveStatValue {
                text,
            }
        }
    }
}

assert_component!(StatValue);
