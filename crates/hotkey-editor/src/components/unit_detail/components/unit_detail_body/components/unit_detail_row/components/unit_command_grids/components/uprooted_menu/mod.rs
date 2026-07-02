mod props;

use crate::components::grid_editors::uprooted_grid_editor::UprootedGridEditor;
use dioxus::prelude::*;
pub use props::UprootedMenuProps;

/// The unit's uprooted-form menu, when it has one; renders nothing otherwise.
#[component]
pub fn UprootedMenu(props: UprootedMenuProps) -> Element {
    let Some(config) = props.config else {
        return rsx! {};
    };
    rsx! {
        UprootedGridEditor { ..config }
    }
}
