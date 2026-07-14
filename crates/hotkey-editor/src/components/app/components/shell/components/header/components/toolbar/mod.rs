pub mod components;
mod style;

use components::collisions_button_host::CollisionsButtonHost;
use components::toolbar_actions::ToolbarActions;
use dioxus::prelude::*;
use style::CLASS;
use tw_macro::assert_component;

/// The editor action bar: a nav with the always-visible collisions button and the
/// adaptive file actions (inline buttons at laptop width and up, a burger drawer
/// below). Pure layout — it threads no data; each child sources its own state, so
/// it carries no header-specific identity and can be placed wherever actions belong.
#[component]
pub fn Toolbar() -> Element {
    rsx! {
        nav {
            class: CLASS,
            aria_label: "Editor actions",
            CollisionsButtonHost {
            


            }
            ToolbarActions {
            


            }
        }
    }
}

assert_component!(Toolbar);
