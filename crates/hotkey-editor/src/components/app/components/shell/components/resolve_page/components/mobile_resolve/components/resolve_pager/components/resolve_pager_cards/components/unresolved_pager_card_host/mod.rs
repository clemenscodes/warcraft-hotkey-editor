mod model;
mod view;

pub use view::UnresolvedPagerCardHostView;
mod style;

use crate::components::app::components::shell::components::resolve_page::components::plan_body::components::unresolved_section::components::unresolved_move_list::components::unresolved_row::UnresolvedRow;
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
