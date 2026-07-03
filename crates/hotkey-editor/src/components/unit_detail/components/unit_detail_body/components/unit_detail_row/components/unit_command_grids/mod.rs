pub mod components;
mod data;
mod logic;
mod props;
mod style;

use crate::assert_component;
use crate::components::grid_editors::command_grid_editor::CommandGridEditor;
use components::build_menu::BuildMenu;
use components::research_menu::ResearchMenu;
use components::uprooted_menu::UprootedMenu;
use dioxus::prelude::*;
use logic::UnitCommandGridsModel;
pub use props::UnitCommandGridsProps;
use style::CLASS;
assert_component!(UnitCommandGrids);

/// The unit's command grids: the command card plus any build, uprooted, and research
/// menus the unit has. Each optional menu renders itself away when the unit lacks it.
#[component]
pub fn UnitCommandGrids(props: UnitCommandGridsProps) -> Element {
    let model = UnitCommandGridsModel::from(&props);
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
