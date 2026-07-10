pub mod components;
mod props;
mod view;

pub use view::CollisionSidebarView;
mod style;

use components::collision_list_scroll::CollisionListScroll;
use dioxus::prelude::*;
use props::CollisionSidebarProps;
use style::CLASS;
use tw_macro::assert_component;

/// The collision sidebar: the aside shell around the scrolling list of collision
/// cards. It owns the chrome and hands the card data down into the scroll region.
#[component]
pub fn CollisionSidebar(props: CollisionSidebarProps) -> Element {
    let cards = props.cards;
    rsx! {
        aside {
            class: CLASS,
            CollisionListScroll { cards }
        }
    }
}

assert_component!(CollisionSidebar);
