pub mod components;
mod model;
mod presentation;
mod style;
mod view;

pub use view::SystemHotkeysDialogBodyView;

use components::inventory_drag_overlay::InventoryDragOverlay;
use components::system_hotkeys_body::SystemHotkeysBody;
use components::system_hotkeys_breadcrumbs::SystemHotkeysBreadcrumbs;
use dioxus::prelude::*;
use presentation::{SystemHotkeysDialogBodyWiring, use_system_hotkeys_dialog_body};
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn SystemHotkeysDialogBody() -> Element {
    let SystemHotkeysDialogBodyWiring {
        active_category,
        drag_follower,
    } = use_system_hotkeys_dialog_body();
    rsx! {
        div {
            class: CLASS,
            SystemHotkeysBreadcrumbs {
                active_category,
            }
            SystemHotkeysBody {



            }
            InventoryDragOverlay {
                drag_follower,
            }
        }
    }
}

assert_component!(SystemHotkeysDialogBody);
