mod components;
mod model;
mod view;

pub use view::HitPointsRowView;
mod style;

use super::super::super::shared::stat_label::StatLabel;
use components::hit_points_value::HitPointsValue;
use dioxus::prelude::*;
use model::HitPointsRowModel;
use style::CLASS;
use tw_macro::assert_component;

const LABEL_TEXT: &str = "Hit Points";

/// The unit's hit points: the vitality column's headline figure. Green and enlarged —
/// the row wears that treatment directly rather than selecting it through a shared
/// variant flag. Hit points are never muted, so the value renders plainly.
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
