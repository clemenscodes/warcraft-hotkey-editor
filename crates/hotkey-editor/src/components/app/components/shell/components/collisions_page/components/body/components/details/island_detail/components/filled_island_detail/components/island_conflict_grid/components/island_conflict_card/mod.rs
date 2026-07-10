pub mod components;
mod logic;
mod props;
mod style;

use components::island_conflict_panel::{IslandConflictPanel, IslandConflictPanelProps};
use dioxus::prelude::*;
use logic::IslandConflictCardModel;
pub use props::IslandConflictCardProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(IslandConflictCard);

/// One conflict card: the affected unit on top, its two clashing abilities flanking
/// the centered separator below. It owns its own card surface directly.
#[component]
pub fn IslandConflictCard(props: IslandConflictCardProps) -> Element {
    let model = IslandConflictCardModel::from(&props);
    let panel = IslandConflictPanelProps::from(model);
    rsx! {
        div {
            class: CLASS,
            IslandConflictPanel { ..panel }
        }
    }
}
