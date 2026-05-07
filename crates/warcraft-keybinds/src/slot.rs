use std::fmt;

use warcraft_api::WarcraftObjectId;

use crate::model::GridCoordinate;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum GridSlotId {
    Ability(WarcraftObjectId),
    AbilityOff(WarcraftObjectId),
    Command(WarcraftObjectId),
}

impl GridSlotId {
    pub fn ability(id: impl Into<WarcraftObjectId>) -> Self {
        Self::Ability(id.into())
    }

    pub fn ability_off(id: impl Into<WarcraftObjectId>) -> Self {
        Self::AbilityOff(id.into())
    }

    pub fn command(id: impl Into<WarcraftObjectId>) -> Self {
        Self::Command(id.into())
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Ability(id) | Self::AbilityOff(id) | Self::Command(id) => id.value(),
        }
    }

    pub fn id(&self) -> WarcraftObjectId {
        match self {
            Self::Ability(id) | Self::AbilityOff(id) | Self::Command(id) => *id,
        }
    }
}

impl fmt::Display for GridSlotId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl From<GridSlotId> for WarcraftObjectId {
    fn from(slot: GridSlotId) -> Self {
        slot.id()
    }
}

const CARD_ROW_COUNT: usize = 3;
const CARD_COLUMN_COUNT: usize = 4;

/// A Warcraft III command card: always exactly 3 rows × 4 columns = 12 slots.
/// Each slot is either empty (`None`) or occupied by a `GridSlotId`.
/// Slots are addressed by their `GridCoordinate` (column, row).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CommandCard {
    slots: [[Option<GridSlotId>; CARD_COLUMN_COUNT]; CARD_ROW_COUNT],
}

impl CommandCard {
    pub fn empty() -> Self {
        Self {
            slots: [[None; CARD_COLUMN_COUNT]; CARD_ROW_COUNT],
        }
    }

    /// Place `slot` at `position`. Returns `false` if the position is already
    /// occupied; the caller is responsible for priority.
    pub fn place(&mut self, position: GridCoordinate, slot: GridSlotId) -> bool {
        let row = usize::from(position.row());
        let column = usize::from(position.column());
        if self.slots[row][column].is_some() {
            return false;
        }
        self.slots[row][column] = Some(slot);
        true
    }

    pub fn slot_at(&self, position: GridCoordinate) -> Option<GridSlotId> {
        let row = usize::from(position.row());
        let column = usize::from(position.column());
        let row_slots = self.slots.get(row)?;
        row_slots.get(column).copied().flatten()
    }

    pub fn is_empty(&self) -> bool {
        self.slots
            .iter()
            .all(|row| row.iter().all(|slot| slot.is_none()))
    }

    pub fn filled_slots(&self) -> impl Iterator<Item = GridSlotId> + '_ {
        self.slots
            .iter()
            .flat_map(|row| row.iter().copied())
            .flatten()
    }
}

#[cfg(test)]
mod slot_tests {
    use super::*;
    use crate::model::{ColumnIndex, RowIndex};

    #[test]
    fn ability_slot_display_shows_id() {
        let slot = GridSlotId::ability("Ahrl");
        assert_eq!(slot.to_string(), "Ahrl");
    }

    #[test]
    fn ability_off_slot_display_shows_id() {
        let slot = GridSlotId::ability_off("Ahrl");
        assert_eq!(slot.to_string(), "Ahrl");
    }

    #[test]
    fn command_slot_display_shows_id() {
        let slot = GridSlotId::command("CmdAttack");
        assert_eq!(slot.to_string(), "CmdAttack");
    }

    #[test]
    fn from_ability_slot_gives_warcraft_object_id() {
        let slot = GridSlotId::ability("Ahrl");
        let object_id = WarcraftObjectId::from(slot);
        assert_eq!(object_id.value(), "Ahrl");
    }

    #[test]
    fn from_command_slot_gives_warcraft_object_id() {
        let slot = GridSlotId::command("CmdMove");
        let object_id = WarcraftObjectId::from(slot);
        assert_eq!(object_id.value(), "CmdMove");
    }

    #[test]
    fn ability_slot_and_ability_off_slot_have_same_id() {
        let on_slot = GridSlotId::ability("AHbh");
        let off_slot = GridSlotId::ability_off("AHbh");
        assert_eq!(on_slot.id(), off_slot.id());
        assert_ne!(on_slot, off_slot);
    }

    #[test]
    fn command_card_empty_has_all_none_slots() {
        let card = CommandCard::empty();
        assert!(card.is_empty());
    }

    #[test]
    fn command_card_place_fills_slot_at_position() {
        let mut card = CommandCard::empty();
        let position = GridCoordinate::new(ColumnIndex::Zero, RowIndex::Zero);
        let slot = GridSlotId::ability("Ahrl");
        let placed = card.place(position, slot);
        assert!(placed);
        assert_eq!(card.slot_at(position), Some(slot));
    }

    #[test]
    fn command_card_place_rejects_occupied_position() {
        let mut card = CommandCard::empty();
        let position = GridCoordinate::new(ColumnIndex::One, RowIndex::One);
        let first_slot = GridSlotId::ability("Ahrl");
        let second_slot = GridSlotId::command("CmdAttack");
        card.place(position, first_slot);
        let rejected = card.place(position, second_slot);
        assert!(!rejected);
        assert_eq!(card.slot_at(position), Some(first_slot));
    }

    #[test]
    fn command_card_filled_slots_yields_placed_slots() {
        let mut card = CommandCard::empty();
        let position_a = GridCoordinate::new(ColumnIndex::Zero, RowIndex::Zero);
        let position_b = GridCoordinate::new(ColumnIndex::Three, RowIndex::Two);
        let slot_a = GridSlotId::ability("Ahrl");
        let slot_b = GridSlotId::command("CmdAttack");
        card.place(position_a, slot_a);
        card.place(position_b, slot_b);
        let filled: Vec<GridSlotId> = card.filled_slots().collect();
        assert_eq!(filled.len(), 2);
        assert!(filled.contains(&slot_a));
        assert!(filled.contains(&slot_b));
    }

    #[test]
    fn command_card_is_not_empty_after_placing_a_slot() {
        let mut card = CommandCard::empty();
        let position = GridCoordinate::new(ColumnIndex::Two, RowIndex::One);
        let slot = GridSlotId::command("CmdMove");
        card.place(position, slot);
        assert!(!card.is_empty());
    }
}
