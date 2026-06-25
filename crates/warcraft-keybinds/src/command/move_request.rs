use crate::grid::layout::GridLayout;
use crate::identity::slot::GridSlotId;

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
