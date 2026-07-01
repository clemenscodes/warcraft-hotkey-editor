mod logic;
mod props;
mod style;

use crate::assert_component;
use crate::components::grid_editors::command_grid_editor::CommandGridEditor;
use crate::components::grid_editors::research_grid_editor::ResearchGridEditor;
use crate::components::grid_editors::uprooted_grid_editor::UprootedGridEditor;
use dioxus::prelude::*;
use logic::UnitCommandGridsModel;
pub use props::UnitCommandGridsProps;
use style::CLASS;
assert_component!(UnitCommandGrids);

/// The unit's command grids: the command card plus any build, uprooted, and research
/// menus the unit has.
#[component]
pub fn UnitCommandGrids(props: UnitCommandGridsProps) -> Element {
    let model = UnitCommandGridsModel::from(&props);
    rsx! {
        div {
            class: CLASS,
            CommandGridEditor { ..model.command_card }
            if let Some(build_menu) = model.build_menu {
                CommandGridEditor { ..build_menu }
            }
            if let Some(uprooted) = model.uprooted {
                UprootedGridEditor { ..uprooted }
            }
            if let Some(research) = model.research {
                ResearchGridEditor { ..research }
            }
        }
    }
}
