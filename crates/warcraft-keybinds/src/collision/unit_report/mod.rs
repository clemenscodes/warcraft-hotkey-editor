use crate::custom_keys::CustomKeys;
use crate::grid::layout::GridLayout;
use crate::unit::grids::{HotkeyCollisionCard, PositionCollisionCard, UnitGrids};
use crate::unit::slots::UnitCommandSlots;
use std::fmt;
use warcraft_api::WarcraftObjectId;
use warcraft_database::WARCRAFT_DATABASE;

#[derive(Debug)]
pub struct UnitCollisionReport {
    entries: Vec<UnitCollisionEntry>,
}

impl PartialEq for UnitCollisionReport {
    fn eq(&self, other: &Self) -> bool {
        self.entries == other.entries
    }
}

#[derive(Clone, Copy, Debug)]
pub struct UnitCollisionEntry {
    unit_id: WarcraftObjectId,
    unit_name: &'static str,
    position_cards: [PositionCollisionCard; 2],
    hotkey_cards: [HotkeyCollisionCard; 2],
}

impl UnitCollisionEntry {
    pub fn unit_id(&self) -> WarcraftObjectId {
        self.unit_id
    }

    pub fn unit_name(&self) -> &'static str {
        self.unit_name
    }

    pub fn position_cards(&self) -> [PositionCollisionCard; 2] {
        self.position_cards
    }

    pub fn hotkey_cards(&self) -> [HotkeyCollisionCard; 2] {
        self.hotkey_cards
    }
}

impl PartialEq for UnitCollisionEntry {
    fn eq(&self, other: &Self) -> bool {
        if self.unit_id != other.unit_id || self.unit_name != other.unit_name {
            return false;
        }
        let self_position_cards: Vec<PositionCollisionCard> = self
            .position_cards
            .iter()
            .filter(|card| !card.is_empty())
            .copied()
            .collect();
        let other_position_cards: Vec<PositionCollisionCard> = other
            .position_cards
            .iter()
            .filter(|card| !card.is_empty())
            .copied()
            .collect();
        if self_position_cards != other_position_cards {
            return false;
        }
        let self_hotkey_cards: Vec<HotkeyCollisionCard> = self
            .hotkey_cards
            .iter()
            .filter(|card| !card.is_empty())
            .copied()
            .collect();
        let other_hotkey_cards: Vec<HotkeyCollisionCard> = other
            .hotkey_cards
            .iter()
            .filter(|card| !card.is_empty())
            .copied()
            .collect();
        self_hotkey_cards == other_hotkey_cards
    }
}

impl UnitCollisionReport {
    pub fn compute(custom_keys: &CustomKeys, layout: GridLayout) -> Self {
        let mut entries: Vec<UnitCollisionEntry> = WARCRAFT_DATABASE
            .all_unit_ids()
            .filter_map(|unit_id| {
                let unit_name = WARCRAFT_DATABASE
                    .by_id(unit_id.value())
                    .and_then(|object| object.names().first().copied())
                    .filter(|name| !name.is_empty())?;
                let unit_grids = UnitGrids::for_unit(unit_id);
                let position_cards = unit_grids.position_collisions(custom_keys);
                let hotkey_cards = unit_grids.hotkey_collisions(custom_keys, layout);
                let no_position_collisions = position_cards.iter().all(|card| card.is_empty());
                let no_hotkey_collisions = hotkey_cards.iter().all(|card| card.is_empty());
                if no_position_collisions && no_hotkey_collisions {
                    return None;
                }
                Some(UnitCollisionEntry {
                    unit_id,
                    unit_name,
                    position_cards,
                    hotkey_cards,
                })
            })
            .collect();
        entries.sort_by(|left, right| {
            left.unit_name
                .cmp(right.unit_name)
                .then_with(|| left.unit_id.value().cmp(right.unit_id.value()))
        });
        Self { entries }
    }

    pub fn entries(&self) -> &[UnitCollisionEntry] {
        &self.entries
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn for_unit(&self, unit_id: &str) -> Self {
        let entries = self
            .entries
            .iter()
            .filter(|entry| entry.unit_id().value().eq_ignore_ascii_case(unit_id))
            .copied()
            .collect();
        Self { entries }
    }
}

impl fmt::Display for UnitCollisionReport {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.entries.is_empty() {
            return writeln!(formatter, "No collisions.");
        }
        for entry in &self.entries {
            writeln!(
                formatter,
                "{} ({}):",
                entry.unit_name(),
                entry.unit_id().value()
            )?;
            for card in entry.position_cards() {
                for (position, slots) in card {
                    let slot_list: Vec<&str> = slots.iter().map(|slot| slot.as_str()).collect();
                    let column = u8::from(position.column());
                    let row = u8::from(position.row());
                    writeln!(
                        formatter,
                        "  position ({column},{row}) {:?}  {}",
                        card.role(),
                        slot_list.join(", "),
                    )?;
                }
            }
            for card in entry.hotkey_cards() {
                for (_, entry) in card {
                    let slot_list: Vec<&str> =
                        entry.slots().iter().map(|slot| slot.as_str()).collect();
                    writeln!(
                        formatter,
                        "  hotkey {} {:?}  {}",
                        entry.token(),
                        card.role(),
                        slot_list.join(", "),
                    )?;
                }
            }
        }
        Ok(())
    }
}

impl ddd::ReadModel for UnitCollisionReport {}

#[cfg(test)]
mod tests;
