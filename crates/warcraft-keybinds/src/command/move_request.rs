use crate::display::grid_behavior::GridBehavior;
use crate::grid::layout::GridLayout;
use crate::identity::slot::GridSlotId;
use crate::model::GridCoordinate;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MoveRequest<'a> {
    layout: GridLayout,
    slot_ids: &'a [GridSlotId],
    moving_slot: &'a GridSlotId,
    target_column: u8,
    target_row: u8,
    is_research_context: bool,
    prevent_swap: bool,
    prevent_co_move: bool,
    assign_hotkey_on_move: bool,
}

impl<'a> MoveRequest<'a> {
    pub fn new(
        layout: GridLayout,
        slot_ids: &'a [GridSlotId],
        moving_slot: &'a GridSlotId,
        target_column: u8,
        target_row: u8,
        is_research_context: bool,
    ) -> Self {
        Self {
            layout,
            slot_ids,
            moving_slot,
            target_column,
            target_row,
            is_research_context,
            prevent_swap: false,
            prevent_co_move: false,
            assign_hotkey_on_move: true,
        }
    }

    /// Builds a move from the grid's behavior and a target coordinate. The
    /// behavior alone decides which position namespace the move reads and whether a
    /// colocated off-state co-moves; the UI never translates those itself. The
    /// caller may still layer on the UI-only toggles (`with_prevent_swap`,
    /// `with_assign_hotkey_on_move`).
    pub fn for_behavior<B: GridBehavior>(
        behavior: &B,
        layout: GridLayout,
        slot_ids: &'a [GridSlotId],
        moving_slot: &'a GridSlotId,
        target: GridCoordinate,
    ) -> Self {
        let is_research_context = behavior.research_positions();
        let prevent_co_move = !behavior.co_move_offstate();
        let target_column = u8::from(target.column());
        let target_row = u8::from(target.row());
        Self {
            layout,
            slot_ids,
            moving_slot,
            target_column,
            target_row,
            is_research_context,
            prevent_swap: false,
            prevent_co_move,
            assign_hotkey_on_move: true,
        }
    }

    pub fn with_prevent_swap(mut self, prevent: bool) -> Self {
        self.prevent_swap = prevent;
        self
    }

    pub fn with_prevent_co_move(mut self, prevent: bool) -> Self {
        self.prevent_co_move = prevent;
        self
    }

    pub fn with_assign_hotkey_on_move(mut self, assign_hotkey: bool) -> Self {
        self.assign_hotkey_on_move = assign_hotkey;
        self
    }

    pub fn layout(&self) -> GridLayout {
        self.layout
    }

    pub fn slot_ids(&self) -> &'a [GridSlotId] {
        self.slot_ids
    }

    pub fn moving_slot(&self) -> &'a GridSlotId {
        self.moving_slot
    }

    pub fn target_column(&self) -> u8 {
        self.target_column
    }

    pub fn target_row(&self) -> u8 {
        self.target_row
    }

    pub fn is_research_context(&self) -> bool {
        self.is_research_context
    }

    pub fn prevent_swap(&self) -> bool {
        self.prevent_swap
    }

    pub fn prevent_co_move(&self) -> bool {
        self.prevent_co_move
    }

    pub fn assign_hotkey_on_move(&self) -> bool {
        self.assign_hotkey_on_move
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::display::grid_behavior::{AlternateFormBehavior, CommandBehavior, ResearchBehavior};
    use crate::model::{ColumnIndex, RowIndex};

    fn target() -> GridCoordinate {
        GridCoordinate::new(ColumnIndex::One, RowIndex::Zero)
    }

    #[test]
    fn command_behavior_reads_primary_namespace_and_co_moves() {
        let slot_ids = [GridSlotId::ability("ACad")];
        let moving = GridSlotId::ability("ACad");
        let layout = GridLayout::qwerty_grid();
        let behavior = CommandBehavior;
        let request = MoveRequest::for_behavior(&behavior, layout, &slot_ids, &moving, target());
        assert!(!request.is_research_context());
        assert!(!request.prevent_co_move());
        assert_eq!(request.target_column(), 1);
        assert_eq!(request.target_row(), 0);
        assert!(request.assign_hotkey_on_move());
        assert!(!request.prevent_swap());
    }

    #[test]
    fn research_behavior_reads_secondary_namespace() {
        let slot_ids = [GridSlotId::ability("ACad")];
        let moving = GridSlotId::ability("ACad");
        let layout = GridLayout::qwerty_grid();
        let behavior = ResearchBehavior;
        let request = MoveRequest::for_behavior(&behavior, layout, &slot_ids, &moving, target());
        assert!(request.is_research_context());
        assert!(!request.prevent_co_move());
    }

    #[test]
    fn alternate_form_behavior_prevents_off_state_co_move() {
        let slot_ids = [GridSlotId::ability("ACad")];
        let moving = GridSlotId::ability("ACad");
        let layout = GridLayout::qwerty_grid();
        let behavior = AlternateFormBehavior;
        let request = MoveRequest::for_behavior(&behavior, layout, &slot_ids, &moving, target());
        assert!(!request.is_research_context());
        assert!(request.prevent_co_move());
    }
}
