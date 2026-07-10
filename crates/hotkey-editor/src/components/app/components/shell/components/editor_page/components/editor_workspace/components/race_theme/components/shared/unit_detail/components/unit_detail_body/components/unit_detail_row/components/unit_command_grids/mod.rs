pub mod components;
mod data;
mod hooks;
mod logic;
mod props;
mod view;

pub use view::UnitCommandGridsView;
mod style;

use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::shared::unit_detail::components::unit_detail_body::components::unit_detail_row::components::shared::grid_editors::command_grid_editor::CommandGridEditor;
use components::build_menu::BuildMenu;
use components::research_menu::ResearchMenu;
use components::uprooted_menu::UprootedMenu;
use dioxus::prelude::*;
use hooks::use_unit_command_grids;
use props::UnitCommandGridsProps;
use style::CLASS;
use tw_macro::assert_component;

/// The unit's command grids: the command card plus any build, uprooted, and research
/// menus the unit has. Each optional menu renders itself away when the unit lacks it.
#[component]
pub fn UnitCommandGrids(props: UnitCommandGridsProps) -> Element {
    let model = use_unit_command_grids(&props);
    let build_menu = model.build_menu;
    let uprooted = model.uprooted;
    let research = model.research;
    rsx! {
        div {
            class: CLASS,
            CommandGridEditor { ..model.command_card }
            BuildMenu { config: build_menu }
            UprootedMenu { config: uprooted }
            ResearchMenu { config: research }
        }
    }
}

assert_component!(UnitCommandGrids);
