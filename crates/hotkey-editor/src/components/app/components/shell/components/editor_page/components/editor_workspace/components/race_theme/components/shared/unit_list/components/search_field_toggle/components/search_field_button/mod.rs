mod props;

use crate::components::app::components::shell::components::editor_page::components::shared::toggle_button::{
    ToggleButton, ToggleButtonProps,
};
use dioxus::prelude::*;
use tw_macro::assert_component;

pub use props::SearchFieldButtonProps;

/// One button of the search-field toggle. It is the shared [`ToggleButton`]
/// configured for the Unit / Ability field switch.
#[component]
pub fn SearchFieldButton(props: SearchFieldButtonProps) -> Element {
    let button = ToggleButtonProps::from(&props);
    rsx! {
        ToggleButton { ..button }
    }
}

assert_component!(SearchFieldButton);
