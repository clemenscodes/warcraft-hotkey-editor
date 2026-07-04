mod props;

use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::unit_detail::components::unit_detail_body::components::unit_detail_row::components::shared::grid_editors::command_grid_editor::CommandGridEditor;
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
