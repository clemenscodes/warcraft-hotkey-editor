mod model;
mod style;
mod view;

pub use view::ConflictPanelBodyView;

use crate::components::app::components::shell::components::collisions_page::components::body::components::details::shared::conflict_card_caption::ConflictCardCaption;
use crate::components::app::components::shell::components::collisions_page::components::body::components::details::shared::conflict_card_model::ConflictCardModel;
use crate::components::app::components::shell::components::collisions_page::components::body::components::details::shared::conflict_multi_stack::ConflictMultiStack;
use crate::components::app::components::shell::components::collisions_page::components::body::components::details::shared::conflict_pair_row::ConflictPairRow;
use dioxus::prelude::*;
use model::ConflictPanelBodyModel;
use style::CLASS;
use tw_macro::assert_component;

/// The conflict panel's body region: the role caption over exactly one of the two clash
/// layouts (the pair row or the multi stack). Each layout renders itself away when it does
/// not apply. Rendered inside the shared `PanelCard` surface, which owns the panel chrome.
#[component]
pub fn ConflictPanelBody(props: ConflictPanelBodyModel) -> Element {
    let models = props.models;
    if let Some(model) = models.into_iter().next() {
        let ConflictCardModel {
            role_label,
            pair,
            multi,
            marker,
        } = model;
        rsx! {
            div {
                class: CLASS,
                ConflictCardCaption {
                    text: role_label,
                }
                ConflictPairRow {
                    pair,
                }
                ConflictMultiStack {
                    abilities: multi,
                    marker,
                }
            }
        }
    } else {
        rsx! {
            div {
                class: CLASS,
            }
        }
    }
}

assert_component!(ConflictPanelBody);
