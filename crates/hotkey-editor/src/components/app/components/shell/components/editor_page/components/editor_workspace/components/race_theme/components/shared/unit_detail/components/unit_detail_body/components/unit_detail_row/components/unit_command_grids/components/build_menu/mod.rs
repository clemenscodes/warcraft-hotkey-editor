mod model;
mod view;

pub use view::BuildMenuView;

use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::shared::unit_detail::components::unit_detail_body::components::unit_detail_row::components::shared::grid_editors::command_grid_editor::CommandGridEditor;
use dioxus::prelude::*;
use tw_macro::assert_component;
use model::BuildMenuModel;

/// The unit's build menu, when it has one; renders nothing otherwise.
#[component]
pub fn BuildMenu(props: BuildMenuModel) -> Element {
    let Some(config) = props.config else {
        return rsx! {};
    };
    rsx! {
        CommandGridEditor { ..config }
    }
}

assert_component!(BuildMenu);
