use crate::collision::island_partition::SlotIslandPartition;
use crate::custom_keys::CustomKeys;
use crate::identity::slot::GridSlotId;
use crate::model::GridCoordinate;
use crate::unit::grids::{GridRole, UnitGrids};
use crate::unit::slots::UnitCommandSlots;
use std::collections::{HashMap, HashSet};
use std::fmt;
use warcraft_api::WarcraftObjectId;
use warcraft_database::WARCRAFT_DATABASE;

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
#[derive(Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct CrossUnitCollisionReport {
    position_groups: Vec<CrossUnitPositionGroup>,
}

/// One independent collision island: a connected component of mutually
/// conflicting abilities at a single grid position on one command card page.
///
/// Two abilities are in the same island only when some unit carries both of
/// them here.  Several groups can therefore share the same `(position,
/// grid_role)` — they are still reported separately, because moving an
/// ability in one island can never affect a different island (no carrier
/// unit bridges them).  This mirrors how the cascade's conflict graph
/// decomposes the problem; lumping a whole coordinate together would imply
/// conflicts that do not exist.
///
/// Abilities on different pages (e.g. MainCommand vs BuildMenu) likewise live
/// in separate groups and cannot collide — those pages are never displayed
/// simultaneously.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
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
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SharedAbilityEntry {
    slot_id: GridSlotId,
    /// Every unit whose command card includes this ability.
    unit_ids: Vec<WarcraftObjectId>,
}

/// A unit experiencing a button collision at this position.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AffectedUnitEntry {
    unit_id: WarcraftObjectId,
    unit_name: &'static str,
    /// All abilities this unit has at the collision position (≥2).
    colliding_slot_ids: Vec<GridSlotId>,
}

impl CrossUnitCollisionReport {
    pub fn compute(custom_keys: &CustomKeys) -> Self {
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
            let shared_str_set: HashSet<&str> = slot_to_unit_set
                .iter()
                .filter(|(_, unit_id_set)| unit_id_set.len() >= 2)
                .map(|(slot_id, _)| slot_id.as_str())
                .collect();
            let context_affected_units: Vec<AffectedUnitEntry> = unit_to_slot_ids
                .into_iter()
                .filter(|(_, slot_ids)| slot_ids.len() >= 2)
                .filter(|(_, slot_ids)| {
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
            if context_affected_units.is_empty() {
                continue;
            }
            let mut island_partition = SlotIslandPartition::new();
            for entry in &context_affected_units {
                let Some(first_slot) = entry.colliding_slot_ids.first() else {
                    continue;
                };
                let first_key = first_slot.as_str();
                island_partition.register(first_key);
                for slot_id in entry.colliding_slot_ids.iter().skip(1) {
                    let slot_key = slot_id.as_str();
                    island_partition.union(first_key, slot_key);
                }
            }
            let mut units_by_island: HashMap<String, Vec<AffectedUnitEntry>> = HashMap::new();
            for entry in context_affected_units {
                let Some(first_slot) = entry.colliding_slot_ids.first() else {
                    continue;
                };
                let island_key = island_partition.root(first_slot.as_str());
                units_by_island.entry(island_key).or_default().push(entry);
            }
            for (_island_key, mut island_affected_units) in units_by_island {
                let mut slots_in_collisions: HashSet<GridSlotId> = HashSet::new();
                for entry in &island_affected_units {
                    for slot_id in &entry.colliding_slot_ids {
                        slots_in_collisions.insert(*slot_id);
                    }
                }
                let mut shared_abilities: Vec<SharedAbilityEntry> = slot_to_unit_set
                    .iter()
                    .filter(|(slot_id, unit_id_set)| {
                        unit_id_set.len() >= 2 && slots_in_collisions.contains(*slot_id)
                    })
                    .map(|(slot_id, unit_id_set)| {
                        let mut unit_ids: Vec<WarcraftObjectId> =
                            unit_id_set.iter().copied().collect();
                        unit_ids.sort_by(|left, right| left.value().cmp(right.value()));
                        SharedAbilityEntry {
                            slot_id: *slot_id,
                            unit_ids,
                        }
                    })
                    .collect();
                if shared_abilities.is_empty() {
                    continue;
                }
                shared_abilities
                    .sort_by(|left, right| left.slot_id.as_str().cmp(right.slot_id.as_str()));
                island_affected_units.sort_by(|left, right| left.unit_name.cmp(right.unit_name));
                let island = CrossUnitPositionGroup {
                    position: context.position,
                    grid_role: context.grid_role,
                    shared_abilities,
                    affected_units: island_affected_units,
                };
                position_groups.push(island);
            }
        }
        position_groups.sort_by(|left, right| {
            let left_row = u8::from(left.position.row());
            let left_col = u8::from(left.position.column());
            let right_row = u8::from(right.position.row());
            let right_col = u8::from(right.position.column());
            let left_role = left.grid_role.sort_index();
            let right_role = right.grid_role.sort_index();
            let left_anchor = left.sort_anchor();
            let right_anchor = right.sort_anchor();
            left_row
                .cmp(&right_row)
                .then_with(|| left_col.cmp(&right_col))
                .then_with(|| left_role.cmp(&right_role))
                .then_with(|| left_anchor.cmp(&right_anchor))
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

    /// Deterministic tiebreaker for ordering islands that sit on the same
    /// `(position, grid_role)`: the lexically-first shared ability, then the
    /// lexically-first affected unit name.  Both lists are already sorted
    /// when the island is built, so `.first()` is stable.
    fn sort_anchor(&self) -> String {
        let first_ability = self
            .shared_abilities
            .first()
            .map(|entry| entry.slot_id.as_str())
            .unwrap_or("");
        let first_unit = self
            .affected_units
            .first()
            .map(|entry| entry.unit_name)
            .unwrap_or("");
        format!("{first_ability}\u{1f}{first_unit}")
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
            "Position ({column},{row}) [{context}] — {unit_count} unit(s) affected:",
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
                parts.join("  ×  "),
            )?;
        }
        Ok(())
    }
}

impl ddd::ReadModel for CrossUnitCollisionReport {}

#[cfg(test)]
mod cross_unit_collision_tests {
    use super::*;
    use crate::model::{AbilityBinding, ColumnIndex, GridCoordinate, RowIndex};

    fn paladin_id() -> WarcraftObjectId {
        WarcraftObjectId::new("Hpal")
    }

    #[test]
    fn normalized_default_has_cross_unit_collisions() {
        let custom_keys = CustomKeys::from_text("");
        let report = CrossUnitCollisionReport::compute(&custom_keys);
        assert!(
            !report.is_empty(),
            "normalized default CustomKeys has known cross-unit collisions \
             (e.g. CmdMove shares position (0,0) with unit-specific abilities)",
        );
    }

    #[test]
    fn normalized_default_collision_count_is_stable() {
        let custom_keys = CustomKeys::from_text("");
        let report = CrossUnitCollisionReport::compute(&custom_keys);
        assert_eq!(
            report.position_groups().len(),
            19,
            "normalized default decomposes into exactly 19 cross-unit collision \
             islands (the raw colliding cells split into 19 independent components; \
             the Halls of the Dead / Black Citadel Backpack research now shares \
             cell (3,0) with the Attack command)",
        );
    }

    #[test]
    fn islands_at_the_same_cell_never_share_an_affected_unit() {
        let custom_keys = CustomKeys::from_text("");
        let report = CrossUnitCollisionReport::compute(&custom_keys);
        let groups = report.position_groups();
        for outer_index in 0..groups.len() {
            for inner_index in (outer_index + 1)..groups.len() {
                let outer = &groups[outer_index];
                let inner = &groups[inner_index];
                if outer.position() != inner.position() || outer.grid_role() != inner.grid_role() {
                    continue;
                }
                let outer_unit_ids: HashSet<WarcraftObjectId> = outer
                    .affected_units()
                    .iter()
                    .map(AffectedUnitEntry::unit_id)
                    .collect();
                let shares_a_unit = inner
                    .affected_units()
                    .iter()
                    .any(|entry| outer_unit_ids.contains(&entry.unit_id()));
                assert!(
                    !shares_a_unit,
                    "two islands at the same cell must not share an affected unit",
                );
            }
        }
    }

    #[test]
    fn at_least_one_cell_decomposes_into_multiple_islands() {
        let custom_keys = CustomKeys::from_text("");
        let report = CrossUnitCollisionReport::compute(&custom_keys);
        let mut island_counts: HashMap<PositionContext, usize> = HashMap::new();
        for group in report.position_groups() {
            let context = PositionContext {
                position: group.position(),
                grid_role: group.grid_role(),
            };
            *island_counts.entry(context).or_insert(0) += 1;
        }
        let has_multi_island_cell = island_counts.values().any(|count| *count > 1);
        assert!(
            has_multi_island_cell,
            "the normalized default has cells whose abilities form several \
             independent islands; the report must split them",
        );
    }

    #[test]
    fn demon_hunter_has_collision_at_two_zero_in_normalized_default() {
        let custom_keys = CustomKeys::from_text("");
        let report = CrossUnitCollisionReport::compute(&custom_keys);
        let two_zero = GridCoordinate::new(ColumnIndex::Two, RowIndex::Zero);
        let demon_hunter_id = WarcraftObjectId::new("Eevi");
        let demon_hunter_affected = report.position_groups().iter().any(|group| {
            group.position() == two_zero
                && group.grid_role() == GridRole::MainCommand
                && group
                    .affected_units()
                    .iter()
                    .any(|entry| entry.unit_id() == demon_hunter_id)
        });
        assert!(
            demon_hunter_affected,
            "Demon Hunter (Eevi) must appear in an island at (2,0) main command",
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
        let mut custom_keys = CustomKeys::from_text("");
        custom_keys.put_ability("AHhb", holy_light_binding);
        custom_keys.put_ability("AHds", divine_shield_binding);
        let report = CrossUnitCollisionReport::compute(&custom_keys);
        assert!(
            !report.is_empty(),
            "two Paladin abilities at (0,0) must produce a cross-unit collision",
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
        let mut custom_keys = CustomKeys::from_text("");
        custom_keys.put_ability("AHhb", holy_light_binding);
        custom_keys.put_ability("AHds", divine_shield_binding);
        let report = CrossUnitCollisionReport::compute(&custom_keys);
        let paladin_id = paladin_id();
        let group = report
            .position_groups()
            .iter()
            .find(|group| {
                group.position() == shared_position
                    && group
                        .affected_units()
                        .iter()
                        .any(|entry| entry.unit_id() == paladin_id)
            })
            .expect("the Paladin's island at the shared position must exist");
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
        let mut custom_keys = CustomKeys::from_text("");
        custom_keys.put_ability("AHhb", holy_light_binding);
        custom_keys.put_ability("AHds", divine_shield_binding);
        let report = CrossUnitCollisionReport::compute(&custom_keys);
        let paladin_id = paladin_id();
        let paladin_affected = report.position_groups().iter().any(|group| {
            group.position() == shared_position
                && group
                    .affected_units()
                    .iter()
                    .any(|entry| entry.unit_id() == paladin_id)
        });
        assert!(
            paladin_affected,
            "Paladin must appear in an island when its abilities collide",
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
        let mut custom_keys = CustomKeys::from_text("");
        custom_keys.put_ability("AHhb", holy_light_binding);
        custom_keys.put_ability("AHds", divine_shield_binding);
        let report = CrossUnitCollisionReport::compute(&custom_keys);
        let paladin_id = paladin_id();
        let group = report
            .position_groups()
            .iter()
            .find(|group| {
                group.position() == shared_position
                    && group
                        .affected_units()
                        .iter()
                        .any(|entry| entry.unit_id() == paladin_id)
            })
            .expect("the Paladin's island at (1,0) must exist");
        let shared_ids: Vec<&str> = group
            .shared_abilities()
            .iter()
            .map(|entry| entry.slot_id().as_str())
            .collect();
        assert!(
            shared_ids.contains(&"AHhb") || shared_ids.contains(&"AHds"),
            "at least one of AHhb/AHds must appear in shared_abilities \
             (the one that is on the Paladin and potentially other units)",
        );
    }

    #[test]
    fn pure_intra_unit_collisions_are_excluded() {
        let custom_keys = CustomKeys::from_text("");
        let report = CrossUnitCollisionReport::compute(&custom_keys);
        for group in report.position_groups() {
            assert!(
                !group.shared_abilities().is_empty(),
                "every cross-unit group must contain at least one shared ability",
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
        let mut custom_keys = CustomKeys::from_text("");
        custom_keys.put_ability("AHhb", holy_light_binding);
        custom_keys.put_ability("AHds", divine_shield_binding);
        let report = CrossUnitCollisionReport::compute(&custom_keys);
        let false_collision = report.position_groups().iter().any(|group| {
            let ids: Vec<&str> = group
                .shared_abilities()
                .iter()
                .map(|entry| entry.slot_id().as_str())
                .collect();
            ids.contains(&"AHhb") && ids.contains(&"AHds")
        });
        assert!(
            !false_collision,
            "abilities at distinct positions must not produce a cross-unit collision",
        );
    }
}
