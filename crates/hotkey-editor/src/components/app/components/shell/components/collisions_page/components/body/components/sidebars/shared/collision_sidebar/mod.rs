pub mod components;
mod props;
mod style;

use components::collision_list_scroll::{CollisionListScroll, CollisionListScrollProps};
use dioxus::prelude::*;
pub use props::CollisionSidebarProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(CollisionSidebar);

/// The collision sidebar: the aside shell around the scrolling list of collision
/// cards. It owns the chrome and hands the card data down into the scroll region.
#[component]
pub fn CollisionSidebar(props: CollisionSidebarProps) -> Element {
    let cards = props.cards;
    let scroll = CollisionListScrollProps { cards };
    rsx! {
        aside {
            class: CLASS,
            CollisionListScroll { ..scroll }
        }
    }
}
