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
/// instead). It only shows and toggles the drawer of action rows; each row is a pure
/// trigger that flips a shared overlay signal, and every dialog is mounted once by
/// `ToolbarActions`, so the burger threads no document and mounts no dialog itself.
#[component]
pub fn BurgerMenu() -> Element {
    let view = use_burger_menu();
    let BurgerMenuView {
        burger_open,
        toggle,
        on_close,
        layout,
        items,
    } = view;
    rsx! {
        button {
            class: CLASS,
            r#type: "button",
            aria_label: "Open menu",
            aria_expanded: "{burger_open()}",
            aria_controls: "burger-drawer",
            onclick: toggle,
            BurgerToggleIcon {}
        }
        if burger_open() {
            BurgerBackdrop { onclick: on_close }
            BurgerDrawer {
                on_close,
                layout,
                items,
            }
        }
    }
}

assert_component!(BurgerMenu);
