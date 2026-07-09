use super::logic::{MoveRowInputs, MoveRowModel};
use super::props::MoveRowProps;
use crate::services::navigation::context::use_view_navigation;

/// Reads the navigation from context and shapes the move row: its reason badge, the
/// mover and rival columns, and the from/to placements. The mover's name link opens the
/// unit through the navigation read here.
pub(super) fn use_move_row(props: &MoveRowProps) -> MoveRowModel {
    let view_navigation = use_view_navigation();
    let inputs = MoveRowInputs {
        move_view: props.move_view.clone(),
        view_navigation,
    };
    MoveRowModel::from(inputs)
}
