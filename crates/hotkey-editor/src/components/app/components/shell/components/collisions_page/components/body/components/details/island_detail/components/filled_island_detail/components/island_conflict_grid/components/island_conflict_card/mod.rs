pub mod components;
pub mod logic;
mod props;
mod view;

pub use view::IslandConflictCardView;
mod style;

use components::island_conflict_panel::IslandConflictPanel;
use dioxus::prelude::*;
use logic::IslandConflictCardModel;
use props::IslandConflictCardProps;
use style::CLASS;
use tw_macro::assert_component;

/// One conflict card: the affected unit on top, its two clashing abilities flanking
/// the centered separator below. It owns its own card surface directly.
#[component]
pub fn IslandConflictCard(props: IslandConflictCardProps) -> Element {
    let model = IslandConflictCardModel::from(&props);
    let IslandConflictCardModel {
        unit,
        own_ability,
        shared_ability,
    } = model;
    rsx! {
        div {
            class: CLASS,
            IslandConflictPanel { unit, own_ability, shared_ability }
        }
    }
}

assert_component!(IslandConflictCard);
