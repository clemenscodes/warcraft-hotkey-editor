mod model;
mod view;

pub use view::HotkeyOverrideEmptyView;
mod style;

use dioxus::prelude::*;

use style::CLASS;
use tw_macro::assert_component;

use model::HotkeyOverrideEmptyModel;

/// The placeholder shown in the hotkey-override section before a grid tile is selected.
#[component]
pub fn HotkeyOverrideEmpty(props: HotkeyOverrideEmptyModel) -> Element {
    let message = props.message;
    rsx! {
        div { class: CLASS,
            p { {message} }
        }
    }
}

assert_component!(HotkeyOverrideEmpty);
