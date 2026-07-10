pub mod components;
mod style;

use components::burger_menu::BurgerMenu;
use components::inline_actions::InlineActions;
use dioxus::prelude::*;
use style::CLASS;
use tw_macro::assert_component;

/// The adaptive file-action controls: the inline button row at laptop width and up,
/// collapsing into the burger drawer on narrower screens. A layout-neutral grouping
/// wrapper — it owns no box and threads no data; each child self-sources. Each action
/// carries the dialog it opens (the inline button on laptop and up, the burger item
/// below), so a dialog is never placed apart from its trigger — only the visible
/// trigger's subtree renders at a given width, so exactly one copy is live.
#[component]
pub fn ToolbarActions() -> Element {
    rsx! {
        div {
            class: CLASS,
            InlineActions {}
            BurgerMenu {}
        }
    }
}

assert_component!(ToolbarActions);
