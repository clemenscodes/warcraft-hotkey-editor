mod model;
mod view;

pub use view::UprootedMenuView;

use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::shared::unit_detail::components::unit_detail_body::components::unit_detail_row::components::shared::grid_editors::uprooted_grid_editor::UprootedGridEditor;
use dioxus::prelude::*;
use tw_macro::assert_component;
use model::UprootedMenuModel;

/// The unit's uprooted-form menu, when it has one; renders nothing otherwise.
#[component]
pub fn UprootedMenu(props: UprootedMenuModel) -> Element {
    let Some(config) = props.config else {
        return rsx! {};
    };
    rsx! {
        UprootedGridEditor { ..config }
    }
}

assert_component!(UprootedMenu);
