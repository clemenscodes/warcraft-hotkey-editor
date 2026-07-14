pub mod components;
mod data;
mod model;
mod view;

pub use view::UnresolvedSectionView;
mod style;

use components::unresolved_move_list::UnresolvedMoveList;
use components::unresolved_title::UnresolvedTitle;
use dioxus::prelude::*;
use model::UnresolvedSectionModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn UnresolvedSection(props: UnresolvedSectionModel) -> Element {
    let unresolved = props.unresolved;
    rsx! {
        div {
            class: CLASS,
            UnresolvedTitle {
                text: data::TITLE,
            }
            UnresolvedMoveList {
                unresolved,
            }
        }
    }
}

assert_component!(UnresolvedSection);
