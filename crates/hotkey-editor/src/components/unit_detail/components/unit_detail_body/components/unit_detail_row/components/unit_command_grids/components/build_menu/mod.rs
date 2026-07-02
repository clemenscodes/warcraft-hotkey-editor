mod props;

use crate::components::grid_editors::command_grid_editor::CommandGridEditor;
use dioxus::prelude::*;
pub use props::BuildMenuProps;

/// The unit's build menu, when it has one; renders nothing otherwise.
#[component]
pub fn BuildMenu(props: BuildMenuProps) -> Element {
    let Some(config) = props.config else {
        return rsx! {};
    };
    rsx! {
        CommandGridEditor { ..config }
    }
}
