use super::super::super::GridTileState;
use super::model::EmptyTileModel;

/// The shaped inputs each overlay child of an empty tile needs, derived once from the
/// slot's state. At most one of the three look overlays mounts (drop-target / blocked /
/// highlight); the drag-over ring is independent. The body only binds these fields onto
/// its children and never computes.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub(super) struct EmptyTilePresentation {
    pub drop_target_active: bool,
    pub blocked_drop_target_active: bool,
    pub highlight_active: bool,
    pub is_drag_over: bool,
}

impl From<EmptyTileModel> for EmptyTilePresentation {
    fn from(props: EmptyTileModel) -> Self {
        let drop_target_active = matches!(props.state, GridTileState::DropTarget);
        let blocked_drop_target_active = matches!(props.state, GridTileState::BlockedDropTarget);
        let highlight_active = matches!(props.state, GridTileState::Highlighted);
        let is_drag_over = props.is_drag_over;
        Self {
            drop_target_active,
            blocked_drop_target_active,
            highlight_active,
            is_drag_over,
        }
    }
}

impl ddd::Presentation for EmptyTilePresentation {
    type Model = EmptyTileModel;
}
