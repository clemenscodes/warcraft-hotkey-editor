use super::super::super::GridTileState;
use super::model::EmptyTileModel;

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
