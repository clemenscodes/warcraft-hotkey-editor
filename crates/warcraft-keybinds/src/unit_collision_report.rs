use std::fmt;

use warcraft_api::WarcraftObjectId;
use warcraft_database::WARCRAFT_DATABASE;

use crate::custom_keys::CustomKeys;
use crate::grid_layout::GridLayout;
use crate::unit_grids::{HotkeyCollisionCard, PositionCollisionCard, UnitGrids};
use crate::unit_slots::UnitCommandSlots;

pub struct UnitCollisionReport {
    entries: Vec<UnitCollisionEntry>,
}

pub struct UnitCollisionEntry {
    unit_id: WarcraftObjectId,
    unit_name: &'static str,
    position_cards: Box<[PositionCollisionCard]>,
    hotkey_cards: Box<[HotkeyCollisionCard]>,
}

impl UnitCollisionEntry {
    pub fn unit_id(&self) -> WarcraftObjectId {
        self.unit_id
    }

    pub fn unit_name(&self) -> &'static str {
        self.unit_name
    }

    pub fn position_cards(&self) -> &[PositionCollisionCard] {
        &self.position_cards
    }

    pub fn hotkey_cards(&self) -> &[HotkeyCollisionCard] {
        &self.hotkey_cards
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
                if position_cards.is_empty() && hotkey_cards.is_empty() {
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
            .map(|entry| UnitCollisionEntry {
                unit_id: entry.unit_id,
                unit_name: entry.unit_name,
                position_cards: entry.position_cards.clone(),
                hotkey_cards: entry.hotkey_cards.clone(),
            })
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
                for (position, slots) in card.iter() {
                    let slot_list: Vec<&str> = slots.iter().map(|slot| slot.as_str()).collect();
                    writeln!(
                        formatter,
                        "  position ({},{}) {:?}  {}",
                        position.column().as_u8(),
                        position.row().as_u8(),
                        card.role(),
                        slot_list.join(", ")
                    )?;
                }
            }
            for card in entry.hotkey_cards() {
                for (_, entry) in card.iter() {
                    let slot_list: Vec<&str> =
                        entry.slots().iter().map(|slot| slot.as_str()).collect();
                    writeln!(
                        formatter,
                        "  hotkey {} {:?}  {}",
                        entry.token(),
                        card.role(),
                        slot_list.join(", ")
                    )?;
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod unit_collision_report_tests {
    use super::*;
    use crate::model::{AbilityBinding, ColumnIndex, GridCoordinate, Hotkey, RowIndex};

    fn paladin_id() -> WarcraftObjectId {
        WarcraftObjectId::new("Hpal")
    }

    #[test]
    fn paladin_has_no_collisions_in_normalized_default() {
        let custom_keys = CustomKeys::from("").normalize();
        let layout = GridLayout::qwerty_grid();
        let report = UnitCollisionReport::compute(&custom_keys, layout);
        let paladin_filtered = report.for_unit("Hpal");
        assert!(
            paladin_filtered.is_empty(),
            "Paladin must have no collisions in the normalized default on QWERTY"
        );
    }

    #[test]
    fn detects_position_collision_across_all_units() {
        let shared_position = GridCoordinate::new(ColumnIndex::Zero, RowIndex::Two);
        let holy_light_binding = AbilityBinding::builder()
            .button_position(shared_position)
            .build();
        let divine_shield_binding = AbilityBinding::builder()
            .button_position(shared_position)
            .build();
        let mut custom_keys = CustomKeys::from("").normalize();
        custom_keys.put_ability("AHhb", holy_light_binding);
        custom_keys.put_ability("AHds", divine_shield_binding);
        let layout = GridLayout::qwerty_grid();
        let report = UnitCollisionReport::compute(&custom_keys, layout);
        let paladin_entry = report
            .entries()
            .iter()
            .find(|entry| entry.unit_id() == paladin_id());
        assert!(
            paladin_entry.is_some(),
            "Paladin must appear in collision report when two abilities share a position"
        );
        let entry = paladin_entry.unwrap();
        assert!(
            !entry.position_cards().is_empty(),
            "position collision card must be present for Paladin"
        );
        let has_collision_at_position = entry
            .position_cards()
            .iter()
            .any(|card| card.collision_at(shared_position).is_some());
        assert!(
            has_collision_at_position,
            "collision must be reported at the shared position"
        );
    }

    #[test]
    fn detects_hotkey_collision_across_all_units() {
        let hotkey_q = Hotkey::Letter('Q');
        let holy_light_binding = AbilityBinding::builder().hotkey(hotkey_q.clone()).build();
        let divine_shield_binding = AbilityBinding::builder().hotkey(hotkey_q).build();
        let mut custom_keys = CustomKeys::from("").normalize();
        custom_keys.put_ability("AHhb", holy_light_binding);
        custom_keys.put_ability("AHds", divine_shield_binding);
        let layout = GridLayout::qwerty_grid();
        let report = UnitCollisionReport::compute(&custom_keys, layout);
        let paladin_entry = report
            .entries()
            .iter()
            .find(|entry| entry.unit_id() == paladin_id());
        assert!(
            paladin_entry.is_some(),
            "Paladin must appear in collision report when two abilities share a hotkey"
        );
        let entry = paladin_entry.unwrap();
        assert!(
            !entry.hotkey_cards().is_empty(),
            "hotkey collision card must be present for Paladin"
        );
    }

    #[test]
    fn for_unit_filters_to_matching_unit() {
        let shared_position = GridCoordinate::new(ColumnIndex::Zero, RowIndex::Two);
        let holy_light_binding = AbilityBinding::builder()
            .button_position(shared_position)
            .build();
        let divine_shield_binding = AbilityBinding::builder()
            .button_position(shared_position)
            .build();
        let mut custom_keys = CustomKeys::from("").normalize();
        custom_keys.put_ability("AHhb", holy_light_binding);
        custom_keys.put_ability("AHds", divine_shield_binding);
        let layout = GridLayout::qwerty_grid();
        let report = UnitCollisionReport::compute(&custom_keys, layout);
        let filtered = report.for_unit("Hpal");
        assert!(
            filtered
                .entries()
                .iter()
                .all(|entry| entry.unit_id() == paladin_id()),
            "for_unit must return only entries for the requested unit"
        );
    }

    #[test]
    fn for_unit_returns_empty_for_unknown_unit() {
        let custom_keys = CustomKeys::from("").normalize();
        let layout = GridLayout::qwerty_grid();
        let report = UnitCollisionReport::compute(&custom_keys, layout);
        let filtered = report.for_unit("ZZZZ");
        assert!(
            filtered.is_empty(),
            "unknown unit id must yield empty report"
        );
    }

    #[test]
    fn entries_are_sorted_by_unit_name() {
        let custom_keys = CustomKeys::from("").normalize();
        let layout = GridLayout::qwerty_grid();
        let report = UnitCollisionReport::compute(&custom_keys, layout);
        let names: Vec<&str> = report.entries().iter().map(|e| e.unit_name()).collect();
        let mut sorted_names = names.clone();
        sorted_names.sort();
        assert_eq!(names, sorted_names, "entries must be sorted by unit name");
    }
}
