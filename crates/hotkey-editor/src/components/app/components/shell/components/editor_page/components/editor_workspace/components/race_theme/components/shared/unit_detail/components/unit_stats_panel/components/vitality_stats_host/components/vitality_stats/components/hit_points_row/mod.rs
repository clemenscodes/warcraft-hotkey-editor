mod components;
mod model;
mod view;

pub use view::HitPointsRowView;
mod style;

use super::super::super::super::super::shared::stat_label::StatLabel;
use components::hit_points_value::HitPointsValue;
use dioxus::prelude::*;
use model::HitPointsRowModel;
use style::CLASS;
use tw_macro::assert_component;

const LABEL_TEXT: &str = "Hit Points";

#[component]
pub fn HitPointsRow(props: HitPointsRowModel) -> Element {
    let value = props.value;
    rsx! {
        div {
            class: CLASS,
            StatLabel {
                text: LABEL_TEXT,
            }
            HitPointsValue {
                value,
            }
        }
    }
}

assert_component!(HitPointsRow);
