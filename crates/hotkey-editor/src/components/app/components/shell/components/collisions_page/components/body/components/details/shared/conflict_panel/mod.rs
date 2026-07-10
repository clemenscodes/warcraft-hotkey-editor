mod props;
mod style;

use crate::components::app::components::shell::components::collisions_page::components::body::components::details::shared::conflict_card_caption::ConflictCardCaption;
use crate::components::app::components::shell::components::collisions_page::components::body::components::details::shared::conflict_card_model::ConflictCardModel;
use crate::components::app::components::shell::components::collisions_page::components::body::components::details::shared::conflict_multi_stack::ConflictMultiStack;
use crate::components::app::components::shell::components::collisions_page::components::body::components::details::shared::conflict_pair_row::ConflictPairRow;
use dioxus::prelude::*;
use props::ConflictPanelProps;
use style::CLASS;
use tw_macro::assert_component;

/// The conflict card surface: the role caption over exactly one of the two clash
/// layouts (the pair row or the multi stack). Each layout renders itself away when it
/// does not apply. Shared by the hotkey and unit-position conflict cards.
#[component]
pub fn ConflictPanel(props: ConflictPanelProps) -> Element {
    let ConflictCardModel {
        role_label,
        pair,
        multi,
        marker,
    } = props.model;
    rsx! {
        div {
            class: CLASS,
            ConflictCardCaption { text: role_label }
            ConflictPairRow { pair }
            ConflictMultiStack {
                abilities: multi,
                marker,
            }
        }
    }
}

assert_component!(ConflictPanel);
