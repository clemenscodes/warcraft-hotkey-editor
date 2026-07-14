pub mod components;
mod model;
mod view;

pub use view::ConflictPositionView;

use components::inline_conflict_position::InlineConflictPosition;
use components::top_conflict_position::TopConflictPosition;
use dioxus::prelude::*;
use model::ConflictPositionModel;
use tw_macro::assert_component;

#[component]
pub fn ConflictPosition(props: ConflictPositionModel) -> Element {
    let coordinate = props.coordinate;
    match props.is_top {
        true => rsx! {
            TopConflictPosition {
                coordinate,
            }
        },
        false => rsx! {
            InlineConflictPosition {
                coordinate,
            }
        },
    }
}

assert_component!(ConflictPosition);
