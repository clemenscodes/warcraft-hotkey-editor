mod model;
mod view;

pub use view::HotkeyAltPositionPickerGridAnchorView;
mod style;

use dioxus::prelude::*;

use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::shared::unit_detail::components::unit_detail_body::components::unit_detail_row::components::shared::grid_editors::command_grid_editor::CommandGridEditor;
use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::shared::unit_detail::components::unit_detail_body::components::unit_detail_row::components::shared::grid_editors::shared::grid_editor::GridEditorView;
use style::CLASS;
use tw_macro::assert_component;

use model::HotkeyAltPositionPickerGridAnchorModel;

/// Centers and picker-restyles the embedded command grid inside a position picker.
#[component]
pub fn HotkeyAltPositionPickerGridAnchor(props: HotkeyAltPositionPickerGridAnchorModel) -> Element {
    let grid_config = GridEditorView::from(&props);
    rsx! {
        div { class: CLASS, CommandGridEditor { ..grid_config } }
    }
}

assert_component!(HotkeyAltPositionPickerGridAnchor);
