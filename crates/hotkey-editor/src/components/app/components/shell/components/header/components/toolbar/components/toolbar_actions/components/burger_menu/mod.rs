pub mod components;
mod presentation;
mod style;

use components::burger_backdrop::BurgerBackdrop;
use components::burger_drawer::BurgerDrawer;
use components::burger_toggle_icon::BurgerToggleIcon;
use dioxus::prelude::*;
use presentation::{BurgerMenuView, use_burger_menu};
use style::CLASS;
use tw_macro::assert_component;

/// The compact-layout menu: a hamburger button that opens a slide-in drawer with every
/// file action. Shown only in the compact header (the full header shows the inline toolbar
/// instead). It only shows and toggles the drawer; each drawer row that opens a dialog owns
/// that dialog itself, so the burger threads no document and mounts no dialog.
#[component]
pub fn BurgerMenu() -> Element {
    let view = use_burger_menu();
    let BurgerMenuView {
        is_open,
        toggle,
        on_close,
    } = view;
    rsx! {
        button {
            class: CLASS,
            r#type: "button",
            aria_label: "Open menu",
            aria_expanded: "{is_open}",
            aria_controls: "burger-drawer",
            onclick: toggle,
            BurgerToggleIcon {}
        }
        if is_open {
            BurgerBackdrop { onclick: on_close }
            BurgerDrawer {
                on_close,
            }
        }
    }
}

assert_component!(BurgerMenu);
