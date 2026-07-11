mod model;
mod view;

pub use view::ResearchMenuView;

use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::shared::unit_detail::components::unit_detail_body::components::unit_detail_row::components::shared::grid_editors::research_grid_editor::ResearchGridEditor;
use dioxus::prelude::*;
use tw_macro::assert_component;
use model::ResearchMenuModel;

/// The unit's research menu, when it has one; renders nothing otherwise.
#[component]
pub fn ResearchMenu(props: ResearchMenuModel) -> Element {
    let Some(config) = props.config else {
        return rsx! {};
    };
    rsx! {
        ResearchGridEditor { ..config }
    }
}

assert_component!(ResearchMenu);
