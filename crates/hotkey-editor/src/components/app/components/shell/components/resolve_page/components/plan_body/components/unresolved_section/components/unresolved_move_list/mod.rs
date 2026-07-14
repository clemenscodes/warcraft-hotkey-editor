pub mod components;
mod model;
mod view;

pub use view::UnresolvedMoveListView;
mod style;

use components::unresolved_row::UnresolvedRow;
use dioxus::prelude::*;
use model::UnresolvedMoveListModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn UnresolvedMoveList(props: UnresolvedMoveListModel) -> Element {
    let unresolved = props.unresolved;
    rsx! {
        div {
            class: CLASS,
            for unresolved_view in unresolved {
                UnresolvedRow {
                    unresolved_view,
                }
            }
        }
    }
}

assert_component!(UnresolvedMoveList);
