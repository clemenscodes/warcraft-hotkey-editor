use std::collections::HashMap;
use std::fmt;

use warcraft_api::{WarcraftObjectId, WarcraftObjectKind, WarcraftObjectMeta};
use warcraft_database::WARCRAFT_DATABASE;

use crate::unit::slots::UnitCommandSlots;

use crate::custom_keys::CustomKeys;
use crate::identity::slot::GridSlotId;
use crate::model::{AbilityBinding, AbilityBindingBuilder, GridCoordinate, Hotkey};

pub struct UnitKeyedCustomKeys {
    groups: Vec<UnitAbilityGroup>,
}

#[derive(Clone, Copy)]
pub struct UnitAbilityGroup {
    unit_id: WarcraftObjectId,
    unit_name: &'static str,
    slots: [Option<UnitAbilitySlot>; 12],
}

#[derive(Clone, Copy)]
pub struct UnitAbilitySlot {
    slot_id: GridSlotId,
    hotkey: Option<Hotkey>,
    position: Option<GridCoordinate>,
    research_hotkey: Option<Hotkey>,
}

impl UnitAbilitySlot {
    pub fn slot_id(&self) -> GridSlotId {
        self.slot_id
    }

    pub fn hotkey(&self) -> Option<Hotkey> {
        self.hotkey
    }

    pub fn position(&self) -> Option<GridCoordinate> {
        self.position
    }

    pub fn research_hotkey(&self) -> Option<Hotkey> {
        self.research_hotkey
    }
}

impl UnitAbilityGroup {
    pub fn unit_id(&self) -> WarcraftObjectId {
        self.unit_id
    }

    pub fn unit_name(&self) -> &'static str {
        self.unit_name
    }

    pub fn slots(&self) -> impl Iterator<Item = UnitAbilitySlot> + '_ {
        self.slots.iter().flatten().copied()
    }
}

impl UnitKeyedCustomKeys {
    pub fn groups(&self) -> &[UnitAbilityGroup] {
        &self.groups
    }

    pub fn for_unit(&self, unit_id: &str) -> Self {
        let groups = self
            .groups
            .iter()
            .filter(|group| group.unit_id().value().eq_ignore_ascii_case(unit_id))
            .cloned()
            .collect();
        Self { groups }
    }
}

impl From<&CustomKeys> for UnitKeyedCustomKeys {
    fn from(custom_keys: &CustomKeys) -> Self {
        let overridden_ids: HashMap<&'static str, &AbilityBinding> = custom_keys
            .bindings_in_order()
            .map(|entry| (entry.ability_id().value(), entry.binding()))
            .collect();

        let mut groups: Vec<UnitAbilityGroup> = WARCRAFT_DATABASE
            .iter()
            .filter_map(|(unit_id, warcraft_object)| {
                if warcraft_object.kind() != WarcraftObjectKind::Unit {
                    return None;
                }
                let WarcraftObjectMeta::Unit(_) = warcraft_object.meta() else {
                    return None;
                };
                let unit_name = warcraft_object.names().first().copied()?;
                if unit_name.is_empty() {
                    return None;
                }
                let command_card = WARCRAFT_DATABASE.command_card(*unit_id);
                let mut slots: [Option<UnitAbilitySlot>; 12] = [None; 12];
                let mut slot_count: usize = 0;
                for slot_id in command_card.filled_slots() {
                    let Some(binding) = overridden_ids.get(slot_id.as_str()) else {
                        continue;
                    };
                    let hotkey = binding.hotkey().cloned();
                    let position = binding.button_position().copied();
                    let research_hotkey = binding.research_hotkey().cloned();
                    let slot = UnitAbilitySlot {
                        slot_id,
                        hotkey,
                        position,
                        research_hotkey,
                    };
                    slots[slot_count] = Some(slot);
                    slot_count += 1;
                }
                if slot_count == 0 {
                    return None;
                }
                Some(UnitAbilityGroup {
                    unit_id: *unit_id,
                    unit_name,
                    slots,
                })
            })
            .collect();

        groups.sort_by(|left, right| {
            left.unit_name
                .cmp(right.unit_name)
                .then_with(|| left.unit_id.value().cmp(right.unit_id.value()))
        });

        Self { groups }
    }
}

impl From<&UnitKeyedCustomKeys> for CustomKeys {
    fn from(unit_keyed: &UnitKeyedCustomKeys) -> Self {
        let mut custom_keys = CustomKeys::from("");
        let mut seen: std::collections::HashSet<&'static str> = std::collections::HashSet::new();
        for group in unit_keyed.groups() {
            for slot in group.slots() {
                let id = slot.slot_id().as_str();
                if !seen.insert(id) {
                    continue;
                }
                let mut builder: AbilityBindingBuilder = AbilityBinding::builder();
                if let Some(hotkey) = slot.hotkey() {
                    builder = builder.hotkey(hotkey);
                }
                if let Some(position) = slot.position() {
                    builder = builder.button_position(position);
                }
                if let Some(research_hotkey) = slot.research_hotkey() {
                    builder = builder.research_hotkey(research_hotkey);
                }
                let binding = builder.build();
                custom_keys.put_ability(id, binding);
            }
        }
        custom_keys
    }
}

impl fmt::Display for UnitKeyedCustomKeys {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        for group in &self.groups {
            writeln!(
                formatter,
                "{} ({}):",
                group.unit_name(),
                group.unit_id().value()
            )?;
            for slot in group.slots() {
                let id = slot.slot_id().as_str();
                let hotkey = slot
                    .hotkey()
                    .map(|hotkey| format!("{hotkey}"))
                    .unwrap_or_default();
                let position = slot
                    .position()
                    .map(|coordinate| format!("{coordinate}"))
                    .unwrap_or_default();
                let research = slot
                    .research_hotkey()
                    .map(|hotkey| format!("  research={hotkey}"))
                    .unwrap_or_default();
                writeln!(
                    formatter,
                    "  {id:<12}  hotkey={hotkey:<4}  pos={position:<6}{research}"
                )?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod unit_keyed_tests {
    use super::*;
    use crate::model::{AbilityBinding, GridCoordinate, Hotkey};
    use crate::model::{ColumnIndex, RowIndex};

    #[test]
    fn round_trip_preserves_hotkey_and_position() {
        let position = GridCoordinate::new(ColumnIndex::Zero, RowIndex::One);
        let hotkey = Hotkey::Letter('Q');
        let binding = AbilityBinding::builder()
            .hotkey(hotkey)
            .button_position(position)
            .build();
        let mut original = CustomKeys::from("");
        original.put_ability("AHhb", binding);

        let unit_keyed = UnitKeyedCustomKeys::from(&original);
        let reconstructed = CustomKeys::from(&unit_keyed);

        let reconstructed_binding = reconstructed
            .binding("AHhb")
            .expect("AHhb must survive round trip");
        assert_eq!(reconstructed_binding.hotkey(), Some(&hotkey));
        assert_eq!(reconstructed_binding.button_position(), Some(&position));
    }

    #[test]
    fn round_trip_preserves_research_hotkey() {
        let research_hotkey = Hotkey::Letter('E');
        let binding = AbilityBinding::builder()
            .research_hotkey(research_hotkey)
            .build();
        let mut original = CustomKeys::from("");
        original.put_ability("AHds", binding);

        let unit_keyed = UnitKeyedCustomKeys::from(&original);
        let reconstructed = CustomKeys::from(&unit_keyed);

        let reconstructed_binding = reconstructed
            .binding("AHds")
            .expect("AHds must survive round trip");
        assert_eq!(
            reconstructed_binding.research_hotkey(),
            Some(&research_hotkey)
        );
    }

    #[test]
    fn for_unit_returns_only_matching_groups() {
        let custom_keys = CustomKeys::from("").normalize();
        let unit_keyed = UnitKeyedCustomKeys::from(&custom_keys);
        let paladin_only = unit_keyed.for_unit("Hpal");
        assert!(
            paladin_only
                .groups()
                .iter()
                .all(|g| g.unit_id().value().eq_ignore_ascii_case("Hpal")),
            "for_unit must return only groups whose unit_id matches"
        );
    }

    #[test]
    fn for_unit_is_case_insensitive() {
        let custom_keys = CustomKeys::from("").normalize();
        let unit_keyed = UnitKeyedCustomKeys::from(&custom_keys);
        let upper = unit_keyed.for_unit("HPAL");
        let lower = unit_keyed.for_unit("hpal");
        assert_eq!(
            upper.groups().len(),
            lower.groups().len(),
            "for_unit must match regardless of case"
        );
    }

    #[test]
    fn for_unit_returns_empty_when_no_match() {
        let custom_keys = CustomKeys::from("").normalize();
        let unit_keyed = UnitKeyedCustomKeys::from(&custom_keys);
        let result = unit_keyed.for_unit("ZZZZ");
        assert!(
            result.groups().is_empty(),
            "unknown unit id must yield empty result"
        );
    }

    #[test]
    fn abilities_not_in_any_unit_command_card_are_dropped() {
        let binding = AbilityBinding::builder()
            .hotkey(Hotkey::Letter('X'))
            .build();
        let mut original = CustomKeys::from("");
        original.put_ability("ZZZZ", binding);

        let unit_keyed = UnitKeyedCustomKeys::from(&original);
        let reconstructed = CustomKeys::from(&unit_keyed);

        assert!(
            reconstructed.binding("ZZZZ").is_none(),
            "ability not in any command card must not appear in reconstructed CustomKeys"
        );
    }
}
