mod props;
mod view;

pub use view::AltPositionPickerGridAnchorView;
mod style;

use dioxus::prelude::*;

use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::shared::unit_detail::components::unit_detail_body::components::unit_detail_row::components::shared::grid_editors::command_grid_editor::CommandGridEditor;
use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::shared::unit_detail::components::unit_detail_body::components::unit_detail_row::components::shared::grid_editors::grid_editor::GridEditorView;
use style::CLASS;
use tw_macro::assert_component;

use props::AltPositionPickerGridAnchorProps;

/// Centers and picker-restyles the embedded command grid inside a position picker.
#[component]
pub fn AltPositionPickerGridAnchor(props: AltPositionPickerGridAnchorProps) -> Element {
    let grid_config = GridEditorView::from(&props);
    rsx! {
        div { class: CLASS, CommandGridEditor { ..grid_config } }
    }
}

assert_component!(AltPositionPickerGridAnchor);
