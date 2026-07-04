pub mod components;
mod list_scroll_kind;
mod props;
mod style;

use crate::assert_component;
use dioxus::prelude::*;
pub use list_scroll_kind::ListScrollKind;
pub use props::SidebarProps;
use style::CLASS;
assert_component!(Sidebar);

/// The base sidebar: the aside shell around a generic scroll slot. It owns the
/// chrome but not the scroll — the extension binds a [`ListScrollKind`], whose
/// impl renders the concrete list scroll around the fed-in cards.
#[component]
pub fn Sidebar<B: ListScrollKind>(props: SidebarProps<B>) -> Element {
    let children = props.children;
    let scroll = B::scroll(children);
    rsx! {
        aside {
            class: CLASS,
            {scroll}
        }
    }
}
