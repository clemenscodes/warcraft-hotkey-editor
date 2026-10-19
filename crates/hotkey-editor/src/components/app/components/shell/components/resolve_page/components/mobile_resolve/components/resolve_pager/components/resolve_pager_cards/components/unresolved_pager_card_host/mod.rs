pub mod components;
mod model;
mod view;

pub use view::UnresolvedPagerCardHostView;
mod style;

use crate::components::app::components::shell::components::resolve_page::components::mobile_resolve::components::resolve_pager::components::resolve_pager_cards::components::unresolved_pager_card_host::components::unresolved_row::UnresolvedRow;
use dioxus::prelude::*;
use model::UnresolvedPagerCardHostModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn UnresolvedPagerCardHost(props: UnresolvedPagerCardHostModel) -> Element {
    let unresolved_view = props.unresolved_view;
    rsx! {
        div {
            class: CLASS,
            UnresolvedRow {
                unresolved_view,
            }
        }
    }
}

assert_component!(UnresolvedPagerCardHost);
