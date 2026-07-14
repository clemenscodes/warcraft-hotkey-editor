mod breadcrumbs;

pub(crate) use breadcrumbs::CollisionBreadcrumbsInputs;

use super::components::body::ContentModel;
use super::model::CollisionsPageModel;
use super::model::{
    CollisionEntry, CollisionList, HotkeysContent, PositionsContent, UnitPositionsContent,
};
use crate::components::app::components::shell::components::shared::breadcrumbs::BreadcrumbView;
use crate::components::app::components::shell::components::shared::icons::ResolvedIcon;
use crate::services::collision_selection::CollisionSelection;
use crate::services::collision_selection::context::use_collision_selection;
use crate::services::customkeys::context::use_custom_keys_service;
use crate::services::customkeys::context::use_loaded_keys;
use crate::services::grid_layout::context::use_grid_layout;
use crate::services::navigation::app_view::{AppView, CollisionKind};
use crate::services::navigation::context::use_view_navigation;
use crate::services::navigation::view_navigation::ViewNavigationContext;
use dioxus::prelude::*;
use std::collections::{HashMap, HashSet};
use warcraft_api::WarcraftObjectId;
use warcraft_keybinds::{
    CrossUnitCollisionReport, CrossUnitPositionGroup, GridCoordinate, GridSlotId,
    SharedAbilityEntry, UnitCollisionEntry, UnitCollisionReport,
};

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

    pub fn extra_count(&self) -> usize {
        self.carrier_count.saturating_sub(1)
    }
}

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

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct AbilityPairKey {
    mover_object_id: WarcraftObjectId,
    anchor_object_id: WarcraftObjectId,
}

#[derive(Clone, PartialEq)]
pub struct IslandView {
    key: String,
    coordinate: GridCoordinate,
    conflicts: Vec<ConflictView>,
    collision_count: usize,
}

impl From<&CrossUnitPositionGroup> for IslandView {
    fn from(group: &CrossUnitPositionGroup) -> Self {
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
}

impl IslandView {
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

impl From<&CrossUnitCollisionReport> for CollisionList<IslandView> {
    fn from(report: &CrossUnitCollisionReport) -> Self {
        let groups = report.position_groups();
        let mut islands: Vec<IslandView> = Vec::with_capacity(groups.len());
        for group in groups {
            let island = IslandView::from(group);
            islands.push(island);
        }
        islands.sort_by_key(|island| std::cmp::Reverse(island.collision_count));
        Self::from(islands)
    }
}

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
}

pub type HotkeyUnitView = CollisionUnitView<HotkeyConflictView>;

pub type UnitPositionUnitView = CollisionUnitView<UnitPositionConflictView>;

#[derive(Clone, PartialEq)]
struct HotkeyConflicts {
    conflicts: Vec<HotkeyConflictView>,
}

impl From<&UnitCollisionEntry> for HotkeyConflicts {
    fn from(entry: &UnitCollisionEntry) -> Self {
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
        Self { conflicts }
    }
}

impl From<&UnitCollisionReport> for CollisionList<HotkeyUnitView> {
    fn from(report: &UnitCollisionReport) -> Self {
        let entries = report.entries();
        let mut units: Vec<HotkeyUnitView> = Vec::with_capacity(entries.len());
        for entry in entries {
            let hotkey_conflicts = HotkeyConflicts::from(entry);
            let conflicts = hotkey_conflicts.conflicts;
            if conflicts.is_empty() {
                continue;
            }
            let collision_count = conflicts.len();
            let unit_object_id = entry.unit_id();
            let unit_id_value = unit_object_id.value();
            let unit = UnitIconView::from(unit_object_id);
            let key = unit_id_value.to_owned();
            let unit_view = HotkeyUnitView {
                key,
                unit,
                collision_count,
                conflicts,
            };
            units.push(unit_view);
        }
        units.sort_by_key(|unit_view| std::cmp::Reverse(unit_view.collision_count));
        Self::from(units)
    }
}

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

#[derive(Clone, PartialEq)]
struct UnitPositionConflicts {
    conflicts: Vec<UnitPositionConflictView>,
}

impl From<&UnitCollisionEntry> for UnitPositionConflicts {
    fn from(entry: &UnitCollisionEntry) -> Self {
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
        Self { conflicts }
    }
}

impl From<&UnitCollisionReport> for CollisionList<UnitPositionUnitView> {
    fn from(report: &UnitCollisionReport) -> Self {
        let entries = report.entries();
        let mut units: Vec<UnitPositionUnitView> = Vec::with_capacity(entries.len());
        for entry in entries {
            let position_conflicts = UnitPositionConflicts::from(entry);
            let conflicts = position_conflicts.conflicts;
            if conflicts.is_empty() {
                continue;
            }
            let collision_count = conflicts.len();
            let unit_object_id = entry.unit_id();
            let unit_id_value = unit_object_id.value();
            let unit = UnitIconView::from(unit_object_id);
            let key = unit_id_value.to_owned();
            let unit_view = UnitPositionUnitView {
                key,
                unit,
                collision_count,
                conflicts,
            };
            units.push(unit_view);
        }
        units.sort_by_key(|unit_view| std::cmp::Reverse(unit_view.collision_count));
        Self::from(units)
    }
}

pub(super) struct CollisionsPagePresentation {
    pub(super) breadcrumbs: Vec<BreadcrumbView>,
    pub(super) content: ContentModel,
}

fn use_route_reconcile(
    kind: CollisionKind,
    entry: Option<String>,
    view_navigation: ViewNavigationContext,
    selection: CollisionSelection,
) {
    use_effect(use_reactive!(|(kind, entry)| {
        let view = AppView::Collisions { kind };
        view_navigation.restore_view(view);
        let mut selected = match kind {
            CollisionKind::Positions => selection.selected_island(),
            CollisionKind::Hotkeys => selection.selected_hotkey_unit(),
            CollisionKind::UnitPositions => selection.selected_unit_position(),
        };
        if *selected.peek() != entry {
            selected.set(entry.clone());
        }
    }));
}

fn use_valid_selection<View>(memo: Memo<CollisionList<View>>, selected: Signal<Option<String>>)
where
    View: CollisionEntry + PartialEq + 'static,
{
    let mut selected = selected;
    use_effect(move || {
        let list = memo.read();
        let views = &list.views;
        if views.is_empty() {
            return;
        }
        let current = selected.read().clone();
        let still_valid = match current {
            Some(ref key) => views.iter().any(|view| view.key() == key),
            None => false,
        };
        if !still_valid {
            let first_key = views.first().map(|view| view.key().to_owned());
            if let Some(key) = first_key {
                selected.set(Some(key));
            }
        }
    });
}

pub(super) fn use_collisions_page(props: &CollisionsPageModel) -> CollisionsPagePresentation {
    let view_navigation = use_view_navigation();
    let selection = use_collision_selection();
    let custom_keys_service = use_custom_keys_service();
    let loaded_keys = use_loaded_keys();
    let grid_layout = use_grid_layout();
    let kind = CollisionKind::from_query_param(props.kind.as_deref());
    let entry = props.entry.clone().filter(|value| !value.is_empty());
    use_route_reconcile(kind, entry, view_navigation, selection);

    let islands_memo = use_memo(move || {
        let report = custom_keys_service.cross_unit_collisions();
        CollisionList::<IslandView>::from(&report)
    });
    let hotkey_units_memo = use_memo(move || {
        let layout = *grid_layout.read();
        let report = custom_keys_service.unit_collisions(layout);
        CollisionList::<HotkeyUnitView>::from(&report)
    });
    let unit_positions_memo = use_memo(move || {
        let layout = *grid_layout.read();
        let report = custom_keys_service.unit_collisions(layout);
        CollisionList::<UnitPositionUnitView>::from(&report)
    });

    let selected_island = selection.selected_island();
    let selected_hotkey_unit = selection.selected_hotkey_unit();
    let selected_unit_position = selection.selected_unit_position();
    use_valid_selection(islands_memo, selected_island);
    use_valid_selection(hotkey_units_memo, selected_hotkey_unit);
    use_valid_selection(unit_positions_memo, selected_unit_position);

    let islands = islands_memo();
    let hotkey_units = hotkey_units_memo();
    let unit_positions = unit_positions_memo();
    let has_file = loaded_keys.read().is_some();

    let breadcrumb_inputs = CollisionBreadcrumbsInputs {
        active_kind: kind,
        position_count: islands.unit_count,
        unit_position_count: unit_positions.collision_count,
        hotkey_count: hotkey_units.collision_count,
        view_navigation,
    };
    let breadcrumbs: Vec<BreadcrumbView> = breadcrumb_inputs.into();
    let content = match kind {
        CollisionKind::Hotkeys => {
            let inputs = HotkeysContent {
                has_file,
                list: hotkey_units,
            };
            ContentModel::from(inputs)
        }
        CollisionKind::UnitPositions => {
            let inputs = UnitPositionsContent {
                has_file,
                list: unit_positions,
            };
            ContentModel::from(inputs)
        }
        CollisionKind::Positions => {
            let inputs = PositionsContent {
                has_file,
                list: islands,
            };
            ContentModel::from(inputs)
        }
    };
    CollisionsPagePresentation {
        breadcrumbs,
        content,
    }
}

impl ddd::Presentation for CollisionsPagePresentation {
    type Model = CollisionsPageModel;
}
