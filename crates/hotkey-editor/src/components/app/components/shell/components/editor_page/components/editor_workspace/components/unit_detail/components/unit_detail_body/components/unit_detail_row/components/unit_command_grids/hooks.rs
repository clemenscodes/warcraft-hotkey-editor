use super::logic::UnitCommandGridsModel;
use super::props::UnitCommandGridsProps;
use crate::services::customkeys::context::use_loaded_keys;
use crate::services::editor_state::context::use_editor_state;
use crate::services::grid_layout::context::use_grid_layout;

/// Sources the shared editor signals from context (rather than having them threaded
/// down as props) and shapes the four grid configs, so the body only names the result.
pub(super) fn use_unit_command_grids(props: &UnitCommandGridsProps) -> UnitCommandGridsModel {
    let loaded_keys = use_loaded_keys();
    let grid_layout = use_grid_layout();
    let editor = use_editor_state();
    UnitCommandGridsModel::build(props, loaded_keys, grid_layout, editor)
}
