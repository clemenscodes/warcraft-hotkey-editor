mod breadcrumbs;

pub(crate) use breadcrumbs::CollisionBreadcrumbsInputs;

use crate::components::app::components::shell::components::shared::icons::ResolvedIcon;
use std::collections::{HashMap, HashSet};
use warcraft_api::WarcraftObjectId;
use warcraft_keybinds::{
    CrossUnitCollisionReport, CrossUnitPositionGroup, GridCoordinate, GridSlotId,
    SharedAbilityEntry, UnitCollisionEntry, UnitCollisionReport,
};

/// One ability resolved to an icon, display name, and object id. Two abilities
/// can share an icon and name yet be distinct objects, so the id is shown too.
#[derive(Clone, PartialEq)]
pub struct AbilityIconView {
    object_id: WarcraftObjectId,
    icon_url: Option<String>,
    name: String,
}

impl From<GridSlotId> for AbilityIconView {
    fn from(slot_id: GridSlotId) -> Self {
        let resolution = AbilityResolution::from(slot_id);
        let object_id = slot_id.id();
        Self {
            object_id,
            icon_url: resolution.icon_url,
            name: resolution.name,
        }
    }
}

impl AbilityIconView {
    pub fn object_id(&self) -> WarcraftObjectId {
        self.object_id
    }

    pub fn icon_url(&self) -> Option<&str> {
        self.icon_url.as_deref()
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

/// One unit resolved to its id, display name, and icon.  The id is kept so an
/// icon click can deep-link into the editor focused on that unit.
#[derive(Clone, PartialEq)]
pub struct UnitIconView {
    unit_id: WarcraftObjectId,
    name: String,
    icon_url: Option<String>,
}

impl From<WarcraftObjectId> for UnitIconView {
    fn from(unit_id: WarcraftObjectId) -> Self {
        let resolved = ResolvedIcon::lookup(unit_id);
        let icon_url = resolved.icon_url().map(str::to_owned);
        let name = resolved.name_or(unit_id);
        Self {
            unit_id,
            name,
            icon_url,
        }
    }
}

impl UnitIconView {
    pub fn unit_id(&self) -> WarcraftObjectId {
        self.unit_id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn icon_url(&self) -> Option<&str> {
        self.icon_url.as_deref()
    }
}

/// One ability participating in a conflict: the ability resolved to an icon,
/// plus the full list of units that carry it (for the carriers dialog) and how
/// many, used for the "+N more" hint.
#[derive(Clone, PartialEq)]
pub struct ConflictAbilityView {
    ability: AbilityIconView,
    carrier_unit_ids: Vec<WarcraftObjectId>,
    carrier_count: usize,
}

impl ConflictAbilityView {
    pub fn ability(&self) -> &AbilityIconView {
        &self.ability
    }

    pub fn carrier_unit_ids(&self) -> &[WarcraftObjectId] {
        &self.carrier_unit_ids
    }

    /// Carriers beyond the affected unit already shown on the conflict card.
    pub fn extra_count(&self) -> usize {
        self.carrier_count.saturating_sub(1)
    }
}

/// One conflict: a single affected unit whose two abilities land on the same
/// cell — its own ability and the shared ability it clashes with.
#[derive(Clone, PartialEq)]
pub struct ConflictView {
    unit: UnitIconView,
    own_ability: ConflictAbilityView,
    shared_ability: ConflictAbilityView,
}

impl ConflictView {
    pub fn unit(&self) -> &UnitIconView {
        &self.unit
    }

    pub fn own_ability(&self) -> &ConflictAbilityView {
        &self.own_ability
    }

    pub fn shared_ability(&self) -> &ConflictAbilityView {
        &self.shared_ability
    }
}

/// Identity of a conflict by the two abilities that clash. Within one island
/// the same mover/anchor pair can appear on many units (every unit that
/// carries both abilities at the cell); they are the same collision with the
/// same fix, so the display collapses them to a single card keyed by this.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct AbilityPairKey {
    mover_object_id: WarcraftObjectId,
    anchor_object_id: WarcraftObjectId,
}

/// A single cross-unit collision island, flattened into display-ready data.
/// All collision facts come from the domain crate; this only formats them.
#[derive(Clone, PartialEq)]
pub struct IslandView {
    key: String,
    coordinate: GridCoordinate,
    conflicts: Vec<ConflictView>,
    collision_count: usize,
}

impl IslandView {
    fn build(group: &CrossUnitPositionGroup) -> Self {
        let coordinate = group.position();
        let key_column = u8::from(coordinate.column());
        let key_row = u8::from(coordinate.row());
        let shared_entries = group.shared_abilities();
        let mut shared_map: HashMap<&'static str, &SharedAbilityEntry> =
            HashMap::with_capacity(shared_entries.len());
        for shared in shared_entries {
            let slot_key = shared.slot_id().as_str();
            shared_map.insert(slot_key, shared);
        }
        let affected_entries = group.affected_units();
        let mut conflicts: Vec<ConflictView> = Vec::with_capacity(affected_entries.len());
        for affected in affected_entries {
            let colliding_slot_ids = affected.colliding_slot_ids();
            let mut shared_slot_option: Option<GridSlotId> = None;
            let mut shared_entry_option: Option<&SharedAbilityEntry> = None;
            for slot_id in colliding_slot_ids {
                let slot_key = slot_id.as_str();
                let Some(candidate) = shared_map.get(slot_key).copied() else {
                    continue;
                };
                let candidate_count = candidate.unit_count();
                let is_better = match shared_entry_option {
                    Some(current) => candidate_count > current.unit_count(),
                    None => true,
                };
                if is_better {
                    shared_slot_option = Some(*slot_id);
                    shared_entry_option = Some(candidate);
                }
            }
            let Some(shared_slot) = shared_slot_option else {
                continue;
            };
            let Some(shared_entry) = shared_entry_option else {
                continue;
            };
            let own_slot = colliding_slot_ids
                .iter()
                .copied()
                .find(|slot_id| *slot_id != shared_slot)
                .unwrap_or(shared_slot);
            let own_ability_icon = AbilityIconView::from(own_slot);
            let shared_ability_icon = AbilityIconView::from(shared_slot);
            let affected_unit_object_id = affected.unit_id();
            let affected_unit = UnitIconView::from(affected_unit_object_id);
            let mut shared_carrier_unit_ids: Vec<WarcraftObjectId> =
                Vec::with_capacity(shared_entry.unit_ids().len());
            for carrier_object in shared_entry.unit_ids() {
                let carrier_value = *carrier_object;
                shared_carrier_unit_ids.push(carrier_value);
            }
            let shared_carrier_count = shared_entry.unit_count();
            let shared_ability = ConflictAbilityView {
                ability: shared_ability_icon,
                carrier_unit_ids: shared_carrier_unit_ids,
                carrier_count: shared_carrier_count,
            };
            let own_slot_key = own_slot.as_str();
            let own_shared_entry_option = shared_map.get(own_slot_key).copied();
            let own_carrier_unit_ids: Vec<WarcraftObjectId> = match own_shared_entry_option {
                Some(entry) => entry.unit_ids().to_vec(),
                None => {
                    let single = affected_unit_object_id;
                    vec![single]
                }
            };
            let own_carrier_count = match own_shared_entry_option {
                Some(entry) => entry.unit_count(),
                None => 1,
            };
            let own_ability = ConflictAbilityView {
                ability: own_ability_icon,
                carrier_unit_ids: own_carrier_unit_ids,
                carrier_count: own_carrier_count,
            };
            let conflict = ConflictView {
                unit: affected_unit,
                own_ability,
                shared_ability,
            };
            conflicts.push(conflict);
        }
        conflicts.sort_by_key(|conflict| {
            let mover_carrier_count = conflict.own_ability.carrier_count;
            let anchor_carrier_count = conflict.shared_ability.carrier_count;
            let combined_carrier_weight = mover_carrier_count + anchor_carrier_count;
            std::cmp::Reverse(combined_carrier_weight)
        });
        let mut seen_ability_pairs: HashSet<AbilityPairKey> = HashSet::new();
        conflicts.retain(|conflict| {
            let mover_object_id = conflict.own_ability.ability.object_id;
            let anchor_object_id = conflict.shared_ability.ability.object_id;
            let pair_key = AbilityPairKey {
                mover_object_id,
                anchor_object_id,
            };
            seen_ability_pairs.insert(pair_key)
        });
        let collision_count = conflicts.len();
        let first_shared_str = shared_entries
            .first()
            .map(|shared| shared.slot_id().as_str())
            .unwrap_or("");
        let key = format!("{key_column}:{key_row}:{first_shared_str}");
        Self {
            key,
            coordinate,
            conflicts,
            collision_count,
        }
    }

    pub fn key(&self) -> &str {
        &self.key
    }

    pub fn coordinate(&self) -> GridCoordinate {
        self.coordinate
    }

    pub fn conflicts(&self) -> &[ConflictView] {
        &self.conflicts
    }

    pub fn collision_count(&self) -> usize {
        self.collision_count
    }
}

/// Resolves a `GridSlotId` to its icon URL and display name via the database.
struct AbilityResolution {
    icon_url: Option<String>,
    name: String,
}

impl From<GridSlotId> for AbilityResolution {
    fn from(slot_id: GridSlotId) -> Self {
        let resolved = ResolvedIcon::lookup(slot_id.id());
        let icon_url = resolved.icon_url().map(str::to_owned);
        let name = match resolved.name() {
            Some(name) => name.to_owned(),
            None => slot_id.display_name(None, None),
        };
        Self { icon_url, name }
    }
}

/// Turns the domain's `CrossUnitCollisionReport` into a flat island list,
/// in the report's stable row/column/role order.
pub(crate) struct CollisionPageModel;

impl CollisionPageModel {
    pub(crate) fn compute(report: &CrossUnitCollisionReport) -> Vec<IslandView> {
        let groups = report.position_groups();
        let mut islands: Vec<IslandView> = Vec::with_capacity(groups.len());
        for group in groups {
            let island = IslandView::build(group);
            islands.push(island);
        }
        islands.sort_by_key(|island| std::cmp::Reverse(island.collision_count));
        islands
    }
}

/// One hotkey conflict on a unit's command card: a hotkey letter shared by two
/// or more abilities on the same card, resolved to display-ready icons.
#[derive(Clone, PartialEq)]
pub struct HotkeyConflictView {
    hotkey_label: String,
    role_label: String,
    abilities: Vec<AbilityIconView>,
}

impl HotkeyConflictView {
    pub fn hotkey_label(&self) -> &str {
        &self.hotkey_label
    }

    pub fn role_label(&self) -> &str {
        &self.role_label
    }

    pub fn abilities(&self) -> &[AbilityIconView] {
        &self.abilities
    }
}

/// One unit that has collisions, flattened into display-ready data: the unit
/// header, how many conflicts it holds, and the conflicts themselves. Generic over
/// the conflict shape — shared-hotkey-letter clashes ([`HotkeyUnitView`]) or
/// per-cell position clashes ([`UnitPositionUnitView`]).
#[derive(Clone, PartialEq)]
pub struct CollisionUnitView<Conflict> {
    key: String,
    unit: UnitIconView,
    collision_count: usize,
    conflicts: Vec<Conflict>,
}

impl<Conflict> CollisionUnitView<Conflict> {
    pub fn key(&self) -> &str {
        &self.key
    }

    pub fn unit(&self) -> &UnitIconView {
        &self.unit
    }

    pub fn collision_count(&self) -> usize {
        self.collision_count
    }

    pub fn conflicts(&self) -> &[Conflict] {
        &self.conflicts
    }

    /// Builds the sorted per-unit collision list from a report: every entry whose
    /// `shape` yields conflicts becomes a view keyed by that unit, and the list is
    /// sorted by collision count descending. This is the shared derivation both
    /// collision kinds run; each kind supplies only how a unit's conflicts are shaped.
    fn from_report(
        report: &UnitCollisionReport,
        shape: impl Fn(&UnitCollisionEntry) -> Vec<Conflict>,
    ) -> Vec<Self> {
        let entries = report.entries();
        let mut units: Vec<Self> = Vec::with_capacity(entries.len());
        for entry in entries {
            let conflicts = shape(entry);
            if conflicts.is_empty() {
                continue;
            }
            let collision_count = conflicts.len();
            let unit_object_id = entry.unit_id();
            let unit_id_value = unit_object_id.value();
            let unit = UnitIconView::from(unit_object_id);
            let key = unit_id_value.to_owned();
            let unit_view = Self {
                key,
                unit,
                collision_count,
                conflicts,
            };
            units.push(unit_view);
        }
        units.sort_by_key(|unit_view| std::cmp::Reverse(unit_view.collision_count));
        units
    }
}

/// A unit's shared-hotkey-letter collisions across its command cards.
pub type HotkeyUnitView = CollisionUnitView<HotkeyConflictView>;

/// A unit's per-cell position collisions (its own abilities clashing on a slot).
pub type UnitPositionUnitView = CollisionUnitView<UnitPositionConflictView>;

/// Turns the domain's `UnitCollisionReport` into a flat list of units with
/// hotkey collisions, each carrying its shared-letter conflicts. Sorted by
/// collision count descending to mirror the position-collision sidebar.
pub(crate) struct HotkeyCollisionPageModel;

impl HotkeyCollisionPageModel {
    pub(crate) fn compute(report: &UnitCollisionReport) -> Vec<HotkeyUnitView> {
        HotkeyUnitView::from_report(report, Self::shape_conflicts)
    }

    /// One unit's shared-hotkey-letter conflicts across its command cards, ordered
    /// by how many abilities clash on each letter (the widest clash first).
    fn shape_conflicts(entry: &UnitCollisionEntry) -> Vec<HotkeyConflictView> {
        let hotkey_cards = entry.hotkey_cards();
        let mut conflicts: Vec<HotkeyConflictView> = Vec::new();
        for card in hotkey_cards {
            if card.is_empty() {
                continue;
            }
            let role = card.role();
            let role_label = role.label().to_owned();
            for (_grid_coordinate, collision_cell) in card {
                let token = collision_cell.token();
                let hotkey_label = token.display_label();
                let collision_slots = collision_cell.slots();
                let mut abilities: Vec<AbilityIconView> = Vec::with_capacity(collision_slots.len());
                for slot_id in collision_slots.iter() {
                    let ability = AbilityIconView::from(slot_id);
                    abilities.push(ability);
                }
                let conflict = HotkeyConflictView {
                    hotkey_label,
                    role_label: role_label.clone(),
                    abilities,
                };
                conflicts.push(conflict);
            }
        }
        conflicts.sort_by_key(|conflict| std::cmp::Reverse(conflict.abilities.len()));
        conflicts
    }
}

/// One per-unit position conflict: a command-card cell where two or more of a
/// single unit's own abilities land on the same slot, resolved to icons.
#[derive(Clone, PartialEq)]
pub struct UnitPositionConflictView {
    coordinate: GridCoordinate,
    role_label: String,
    abilities: Vec<AbilityIconView>,
}

impl UnitPositionConflictView {
    pub fn coordinate(&self) -> GridCoordinate {
        self.coordinate
    }

    pub fn role_label(&self) -> &str {
        &self.role_label
    }

    pub fn abilities(&self) -> &[AbilityIconView] {
        &self.abilities
    }
}

/// Turns the domain's `UnitCollisionReport` into a flat list of units whose own
/// abilities collide on a command-card cell, each carrying its per-cell
/// conflicts. Sorted by collision count descending.
pub(crate) struct UnitPositionPageModel;

impl UnitPositionPageModel {
    pub(crate) fn compute(report: &UnitCollisionReport) -> Vec<UnitPositionUnitView> {
        UnitPositionUnitView::from_report(report, Self::shape_conflicts)
    }

    /// One unit's per-cell position conflicts across its command cards, ordered by
    /// how many of the unit's own abilities clash on each cell (widest clash first).
    fn shape_conflicts(entry: &UnitCollisionEntry) -> Vec<UnitPositionConflictView> {
        let position_cards = entry.position_cards();
        let mut conflicts: Vec<UnitPositionConflictView> = Vec::new();
        for card in position_cards {
            if card.is_empty() {
                continue;
            }
            let role = card.role();
            let role_label = role.label().to_owned();
            for (coordinate, collision_slots) in card {
                let mut abilities: Vec<AbilityIconView> = Vec::with_capacity(collision_slots.len());
                for slot_id in collision_slots.iter() {
                    let ability = AbilityIconView::from(slot_id);
                    abilities.push(ability);
                }
                let conflict = UnitPositionConflictView {
                    coordinate,
                    role_label: role_label.clone(),
                    abilities,
                };
                conflicts.push(conflict);
            }
        }
        conflicts.sort_by_key(|conflict| std::cmp::Reverse(conflict.abilities.len()));
        conflicts
    }
}
