use std::collections::{HashMap, HashSet};
use std::fmt;

use warcraft_api::WarcraftObjectId;
use warcraft_database::WARCRAFT_DATABASE;

use crate::custom_keys::CustomKeys;
use crate::identity::slot::GridSlotId;
use crate::model::GridCoordinate;
use crate::unit::grids::{GridRole, UnitGrids};
use crate::unit::slots::UnitCommandSlots;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct PositionContext {
    position: GridCoordinate,
    grid_role: GridRole,
}

/// The subset of per-unit collisions where at least one colliding ability is shared
/// across multiple units.
///
/// A "cross-unit" collision is one where fixing the intra-unit problem by moving
/// an ability has ripple effects: because that ability is on N other units, moving
/// it may create or shift collisions there too.  Pure intra-unit collisions (all
/// colliding abilities exclusive to one unit) are omitted — they belong only in
/// `UnitCollisionReport`.
pub struct CrossUnitCollisionReport {
    position_groups: Vec<CrossUnitPositionGroup>,
}

/// One grid position, on one specific command card page, where at least one shared
/// ability is involved in a collision.  Abilities on different pages (e.g.
/// MainCommand vs BuildMenu) live in separate groups and cannot collide — those
/// pages are never displayed simultaneously.
pub struct CrossUnitPositionGroup {
    position: GridCoordinate,
    grid_role: GridRole,
    /// The abilities at this position that span multiple units and participate in
    /// at least one actual collision here.  Moving any of these affects every unit
    /// in its `unit_ids` list — that is the cross-unit cost.
    shared_abilities: Vec<SharedAbilityEntry>,
    /// Units that have two or more abilities at this position.  Each entry mirrors
    /// what the per-unit collision report shows for that unit at this position.
    affected_units: Vec<AffectedUnitEntry>,
}

/// An ability assigned to a colliding position that appears on two or more units.
pub struct SharedAbilityEntry {
    slot_id: GridSlotId,
    /// Every unit whose command card includes this ability.
    unit_ids: Vec<WarcraftObjectId>,
}

/// A unit experiencing a button collision at this position.
pub struct AffectedUnitEntry {
    unit_id: WarcraftObjectId,
    unit_name: &'static str,
    /// All abilities this unit has at the collision position (≥2).
    colliding_slot_ids: Vec<GridSlotId>,
}

impl CrossUnitCollisionReport {
    pub fn compute(custom_keys: &CustomKeys) -> Self {
        // Build: (position, grid_role) -> slot_id -> set of unit_ids.
        // Grid role is the key — MainCommand, BuildMenu, and UprootedForm are
        // separate pages that are never visible at the same time, so slots on
        // different pages of the same unit cannot collide.
        let mut entries_by_context: HashMap<
            PositionContext,
            HashMap<GridSlotId, HashSet<WarcraftObjectId>>,
        > = HashMap::new();

        for unit_id in WARCRAFT_DATABASE.all_unit_ids() {
            let unit_grids = UnitGrids::for_unit(unit_id);
            for named_grid in unit_grids.grids() {
                let grid_role = named_grid.role();
                let is_research_context = grid_role.is_research_context();
                for slot in named_grid.card().filled_slots() {
                    let Some(position) = custom_keys.position_for_slot(&slot, is_research_context)
                    else {
                        continue;
                    };
                    let context_key = PositionContext {
                        position,
                        grid_role,
                    };
                    entries_by_context
                        .entry(context_key)
                        .or_default()
                        .entry(slot)
                        .or_default()
                        .insert(unit_id);
                }
            }
        }

        let mut position_groups: Vec<CrossUnitPositionGroup> = Vec::new();

        for (context, slot_to_unit_set) in entries_by_context {
            if slot_to_unit_set.len() < 2 {
                continue;
            }

            // Build per-unit slot lists, deduplicating by as_str() so that
            // Ability(X) and AbilityOff(X) at the same position are not counted
            // as two separate buttons (they are two states of one button).
            let mut unit_to_slot_ids: HashMap<WarcraftObjectId, Vec<GridSlotId>> = HashMap::new();
            for (slot_id, unit_id_set) in &slot_to_unit_set {
                for unit_id in unit_id_set {
                    let slots = unit_to_slot_ids.entry(*unit_id).or_default();
                    let already_present = slots
                        .iter()
                        .any(|existing| existing.as_str() == slot_id.as_str());
                    if !already_present {
                        slots.push(*slot_id);
                    }
                }
            }

            // Shared abilities at this position (on 2+ units).
            let shared_str_set: HashSet<&str> = slot_to_unit_set
                .iter()
                .filter(|(_, unit_id_set)| unit_id_set.len() >= 2)
                .map(|(slot_id, _)| slot_id.as_str())
                .collect();

            let mut affected_units: Vec<AffectedUnitEntry> = unit_to_slot_ids
                .into_iter()
                .filter(|(_, slot_ids)| {
                    // 2+ distinct abilities at this position on this unit.
                    slot_ids.len() >= 2
                })
                .filter(|(_, slot_ids)| {
                    // At least one of the colliding abilities must be shared across
                    // multiple units; otherwise this is a pure intra-unit collision.
                    slot_ids
                        .iter()
                        .any(|slot_id| shared_str_set.contains(slot_id.as_str()))
                })
                .filter_map(|(unit_id, mut colliding_slot_ids)| {
                    let unit_name = WARCRAFT_DATABASE
                        .by_id(unit_id.value())
                        .and_then(|object| object.names().first().copied())
                        .filter(|name| !name.is_empty())?;
                    colliding_slot_ids.sort_by(|left, right| left.as_str().cmp(right.as_str()));
                    Some(AffectedUnitEntry {
                        unit_id,
                        unit_name,
                        colliding_slot_ids,
                    })
                })
                .collect();

            if affected_units.is_empty() {
                continue;
            }

            // Collect every slot_id that actually participates in at least one collision.
            let mut slots_in_collisions: HashSet<GridSlotId> = HashSet::new();
            for entry in &affected_units {
                for slot_id in &entry.colliding_slot_ids {
                    slots_in_collisions.insert(*slot_id);
                }
            }

            // The "cross-unit" abilities: shared across 2+ units AND part of an actual collision.
            let mut shared_abilities: Vec<SharedAbilityEntry> = slot_to_unit_set
                .iter()
                .filter(|(slot_id, unit_id_set)| {
                    unit_id_set.len() >= 2 && slots_in_collisions.contains(*slot_id)
                })
                .map(|(slot_id, unit_id_set)| {
                    let mut unit_ids: Vec<WarcraftObjectId> = unit_id_set.iter().copied().collect();
                    unit_ids.sort_by(|left, right| left.value().cmp(right.value()));
                    SharedAbilityEntry {
                        slot_id: *slot_id,
                        unit_ids,
                    }
                })
                .collect();

            // Skip groups where every collision is purely intra-unit (no shared ability involved).
            if shared_abilities.is_empty() {
                continue;
            }

            shared_abilities
                .sort_by(|left, right| left.slot_id.as_str().cmp(right.slot_id.as_str()));
            affected_units.sort_by(|left, right| left.unit_name.cmp(right.unit_name));

            position_groups.push(CrossUnitPositionGroup {
                position: context.position,
                grid_role: context.grid_role,
                shared_abilities,
                affected_units,
            });
        }

        position_groups.sort_by(|left, right| {
            let left_row = u8::from(left.position.row());
            let left_col = u8::from(left.position.column());
            let right_row = u8::from(right.position.row());
            let right_col = u8::from(right.position.column());
            left_row
                .cmp(&right_row)
                .then_with(|| left_col.cmp(&right_col))
        });

        Self { position_groups }
    }

    pub fn position_groups(&self) -> &[CrossUnitPositionGroup] {
        &self.position_groups
    }

    pub fn is_empty(&self) -> bool {
        self.position_groups.is_empty()
    }

    pub fn total_affected_unit_count(&self) -> usize {
        let mut seen: HashSet<WarcraftObjectId> = HashSet::new();
        for group in &self.position_groups {
            for entry in &group.affected_units {
                seen.insert(entry.unit_id);
            }
        }
        seen.len()
    }
}

impl CrossUnitPositionGroup {
    pub fn position(&self) -> GridCoordinate {
        self.position
    }

    pub fn grid_role(&self) -> GridRole {
        self.grid_role
    }

    pub fn shared_abilities(&self) -> &[SharedAbilityEntry] {
        &self.shared_abilities
    }

    pub fn affected_units(&self) -> &[AffectedUnitEntry] {
        &self.affected_units
    }
}

impl SharedAbilityEntry {
    pub fn slot_id(&self) -> GridSlotId {
        self.slot_id
    }

    pub fn unit_ids(&self) -> &[WarcraftObjectId] {
        &self.unit_ids
    }

    pub fn unit_count(&self) -> usize {
        self.unit_ids.len()
    }
}

impl AffectedUnitEntry {
    pub fn unit_id(&self) -> WarcraftObjectId {
        self.unit_id
    }

    pub fn unit_name(&self) -> &'static str {
        self.unit_name
    }

    pub fn colliding_slot_ids(&self) -> &[GridSlotId] {
        &self.colliding_slot_ids
    }
}

impl fmt::Display for CrossUnitCollisionReport {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.position_groups.is_empty() {
            return writeln!(formatter, "No cross-unit collisions.");
        }
        for group in &self.position_groups {
            write!(formatter, "{group}")?;
        }
        Ok(())
    }
}

impl fmt::Display for CrossUnitPositionGroup {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let column = u8::from(self.position.column());
        let row = u8::from(self.position.row());
        let context = match self.grid_role {
            GridRole::HeroSkillTree => "research",
            GridRole::MainCommand => "main command",
            GridRole::BuildMenu => "build menu",
            GridRole::UprootedForm => "uprooted",
        };
        let unit_count = self.affected_units.len();
        writeln!(
            formatter,
            "Position ({column},{row}) [{context}] — {unit_count} unit(s) affected:"
        )?;

        let unit_count_for_slot: HashMap<&str, usize> = self
            .shared_abilities
            .iter()
            .map(|entry| (entry.slot_id.as_str(), entry.unit_ids.len()))
            .collect();

        for affected in &self.affected_units {
            let parts: Vec<String> = affected
                .colliding_slot_ids
                .iter()
                .map(|slot_id| {
                    let name = slot_id.display_name(None, None);
                    let count = unit_count_for_slot
                        .get(slot_id.as_str())
                        .copied()
                        .unwrap_or(1);
                    let noun = if count == 1 { "unit" } else { "units" };
                    format!("{name} [{count} {noun}]")
                })
                .collect();
            writeln!(
                formatter,
                "  {} ({}):  {}",
                affected.unit_name,
                affected.unit_id.value(),
                parts.join("  ×  ")
            )?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod cross_unit_collision_tests {
    use super::*;
    use crate::model::{AbilityBinding, ColumnIndex, GridCoordinate, RowIndex};

    fn paladin_id() -> WarcraftObjectId {
        WarcraftObjectId::new("Hpal")
    }

    #[test]
    fn normalized_default_has_cross_unit_collisions() {
        let custom_keys = CustomKeys::from("").normalize();
        let report = CrossUnitCollisionReport::compute(&custom_keys);
        assert!(
            !report.is_empty(),
            "normalized default CustomKeys has known cross-unit collisions \
             (e.g. CmdMove shares position (0,0) with unit-specific abilities)"
        );
    }

    #[test]
    fn normalized_default_collision_count_is_stable() {
        let custom_keys = CustomKeys::from("").normalize();
        let report = CrossUnitCollisionReport::compute(&custom_keys);
        assert_eq!(
            report.position_groups().len(),
            5,
            "normalized default must produce exactly 5 cross-unit collision groups"
        );
    }

    #[test]
    fn demon_hunter_has_collision_at_two_zero_in_normalized_default() {
        let custom_keys = CustomKeys::from("").normalize();
        let report = CrossUnitCollisionReport::compute(&custom_keys);
        let two_zero = GridCoordinate::new(ColumnIndex::Two, RowIndex::Zero);
        let group = report
            .position_groups()
            .iter()
            .find(|group| {
                group.position() == two_zero && group.grid_role() == GridRole::MainCommand
            })
            .expect("collision group at (2,0) main command must exist in default");
        let demon_hunter_id = WarcraftObjectId::new("Eevi");
        let demon_hunter_affected = group
            .affected_units()
            .iter()
            .any(|entry| entry.unit_id() == demon_hunter_id);
        assert!(
            demon_hunter_affected,
            "Demon Hunter (Eevi) must appear in affected_units at (2,0)"
        );
    }

    #[test]
    fn detects_collision_when_two_paladin_abilities_share_position() {
        let shared_position = GridCoordinate::new(ColumnIndex::Zero, RowIndex::Zero);
        let holy_light_binding = AbilityBinding::builder()
            .button_position(shared_position)
            .build();
        let divine_shield_binding = AbilityBinding::builder()
            .button_position(shared_position)
            .build();
        let mut custom_keys = CustomKeys::from("").normalize();
        custom_keys.put_ability("AHhb", holy_light_binding);
        custom_keys.put_ability("AHds", divine_shield_binding);
        let report = CrossUnitCollisionReport::compute(&custom_keys);
        assert!(
            !report.is_empty(),
            "two Paladin abilities at (0,0) must produce a cross-unit collision"
        );
    }

    #[test]
    fn collision_group_position_matches_shared_position() {
        let shared_position = GridCoordinate::new(ColumnIndex::Two, RowIndex::One);
        let holy_light_binding = AbilityBinding::builder()
            .button_position(shared_position)
            .build();
        let divine_shield_binding = AbilityBinding::builder()
            .button_position(shared_position)
            .build();
        let mut custom_keys = CustomKeys::from("").normalize();
        custom_keys.put_ability("AHhb", holy_light_binding);
        custom_keys.put_ability("AHds", divine_shield_binding);
        let report = CrossUnitCollisionReport::compute(&custom_keys);
        let group = report
            .position_groups()
            .iter()
            .find(|group| group.position() == shared_position)
            .expect("must find a group at the shared position");
        assert_eq!(group.position(), shared_position);
        assert_eq!(group.grid_role(), GridRole::MainCommand);
    }

    #[test]
    fn affected_units_includes_paladin_for_paladin_ability_collision() {
        let shared_position = GridCoordinate::new(ColumnIndex::Zero, RowIndex::Zero);
        let holy_light_binding = AbilityBinding::builder()
            .button_position(shared_position)
            .build();
        let divine_shield_binding = AbilityBinding::builder()
            .button_position(shared_position)
            .build();
        let mut custom_keys = CustomKeys::from("").normalize();
        custom_keys.put_ability("AHhb", holy_light_binding);
        custom_keys.put_ability("AHds", divine_shield_binding);
        let report = CrossUnitCollisionReport::compute(&custom_keys);
        let paladin_id = paladin_id();
        let group = report
            .position_groups()
            .iter()
            .find(|group| group.position() == shared_position)
            .expect("collision group at (0,0) must exist");
        let paladin_affected = group
            .affected_units()
            .iter()
            .any(|entry| entry.unit_id() == paladin_id);
        assert!(
            paladin_affected,
            "Paladin must appear in affected_units when its abilities collide"
        );
    }

    #[test]
    fn shared_abilities_covers_colliding_abilities() {
        let shared_position = GridCoordinate::new(ColumnIndex::One, RowIndex::Zero);
        let holy_light_binding = AbilityBinding::builder()
            .button_position(shared_position)
            .build();
        let divine_shield_binding = AbilityBinding::builder()
            .button_position(shared_position)
            .build();
        let mut custom_keys = CustomKeys::from("").normalize();
        custom_keys.put_ability("AHhb", holy_light_binding);
        custom_keys.put_ability("AHds", divine_shield_binding);
        let report = CrossUnitCollisionReport::compute(&custom_keys);
        let group = report
            .position_groups()
            .iter()
            .find(|group| group.position() == shared_position)
            .expect("collision group at (1,0) must exist");
        let shared_ids: Vec<&str> = group
            .shared_abilities()
            .iter()
            .map(|entry| entry.slot_id().as_str())
            .collect();
        assert!(
            shared_ids.contains(&"AHhb") || shared_ids.contains(&"AHds"),
            "at least one of AHhb/AHds must appear in shared_abilities \
             (the one that is on the Paladin and potentially other units)"
        );
    }

    #[test]
    fn pure_intra_unit_collisions_are_excluded() {
        // If two abilities each appear on exactly one unit and that unit is the same,
        // the collision is purely intra-unit and must not appear in the cross-unit report.
        // We verify this by checking that groups with no shared ability are absent.
        let custom_keys = CustomKeys::from("").normalize();
        let report = CrossUnitCollisionReport::compute(&custom_keys);
        for group in report.position_groups() {
            assert!(
                !group.shared_abilities().is_empty(),
                "every cross-unit group must contain at least one shared ability"
            );
        }
    }

    #[test]
    fn no_false_positives_when_abilities_are_at_different_positions() {
        let position_a = GridCoordinate::new(ColumnIndex::Zero, RowIndex::Zero);
        let position_b = GridCoordinate::new(ColumnIndex::One, RowIndex::Zero);
        let holy_light_binding = AbilityBinding::builder()
            .button_position(position_a)
            .build();
        let divine_shield_binding = AbilityBinding::builder()
            .button_position(position_b)
            .build();
        let mut custom_keys = CustomKeys::from("").normalize();
        custom_keys.put_ability("AHhb", holy_light_binding);
        custom_keys.put_ability("AHds", divine_shield_binding);
        let report = CrossUnitCollisionReport::compute(&custom_keys);
        let false_collision = report.position_groups().iter().any(|group| {
            let ids: Vec<&str> = group
                .shared_abilities()
                .iter()
                .map(|e| e.slot_id().as_str())
                .collect();
            ids.contains(&"AHhb") && ids.contains(&"AHds")
        });
        assert!(
            !false_collision,
            "abilities at distinct positions must not produce a cross-unit collision"
        );
    }
}
