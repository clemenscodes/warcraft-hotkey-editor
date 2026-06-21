mod detail;
mod mini_grid;
mod sidebar;

use std::collections::HashMap;

use dioxus::prelude::*;
use warcraft_database::ObjectLookup;
use warcraft_keybinds::{
    CrossUnitCollisionReport, CrossUnitPositionGroup, CustomKeys, GridSlotId, SharedAbilityEntry,
};

use crate::components::shared::icons::ICON_COLLISIONS_CLEAR;
use crate::model::icons::IconUrl;
use crate::services::navigation::app_view::CollisionKind;
use crate::services::navigation::view_navigation::ViewNavigationContext;

use detail::IslandDetail;
use sidebar::IslandSidebar;

/// One ability resolved to an icon, display name, and object id. Two abilities
/// can share an icon and name yet be distinct objects, so the id is shown too.
#[derive(Clone, PartialEq)]
pub(super) struct AbilityIconView {
    object_id: String,
    icon_url: Option<String>,
    name: String,
}

impl AbilityIconView {
    fn resolve(slot_id: GridSlotId) -> Self {
        let resolution = AbilityResolution::resolve(slot_id);
        let object_id = slot_id.as_str().to_owned();
        Self {
            object_id,
            icon_url: resolution.icon_url,
            name: resolution.name,
        }
    }

    pub(super) fn object_id(&self) -> &str {
        &self.object_id
    }

    pub(super) fn icon_url(&self) -> Option<&str> {
        self.icon_url.as_deref()
    }

    pub(super) fn name(&self) -> &str {
        &self.name
    }
}

/// One unit resolved to its id, display name, and icon.  The id is kept so an
/// icon click can deep-link into the editor focused on that unit.
#[derive(Clone, PartialEq)]
pub(super) struct UnitIconView {
    unit_id: String,
    name: String,
    icon_url: Option<String>,
}

impl UnitIconView {
    pub(super) fn resolve(unit_id_value: &str) -> Self {
        let object_option = ObjectLookup::by_id(unit_id_value);
        let icon_url = object_option
            .and_then(|object| object.icons().first().copied())
            .map(IconUrl::from_database_path)
            .map(|icon| icon.to_string());
        let name_option = object_option.and_then(|object| object.names().first().copied());
        let name = name_option.unwrap_or(unit_id_value).to_owned();
        let unit_id = unit_id_value.to_owned();
        Self {
            unit_id,
            name,
            icon_url,
        }
    }

    pub(super) fn unit_id(&self) -> &str {
        &self.unit_id
    }

    pub(super) fn name(&self) -> &str {
        &self.name
    }

    pub(super) fn icon_url(&self) -> Option<&str> {
        self.icon_url.as_deref()
    }
}

/// One side of a conflict: an ability, the unit shown as its header, and the
/// full list of units that carry it (resolved lazily for the carriers dialog).
#[derive(Clone, PartialEq)]
pub(super) struct ConflictSide {
    unit: UnitIconView,
    ability: AbilityIconView,
    carrier_unit_ids: Vec<String>,
    carrier_count: usize,
}

impl ConflictSide {
    pub(super) fn unit(&self) -> &UnitIconView {
        &self.unit
    }

    pub(super) fn ability(&self) -> &AbilityIconView {
        &self.ability
    }

    pub(super) fn carrier_unit_ids(&self) -> &[String] {
        &self.carrier_unit_ids
    }

    /// Other carriers beyond the one shown as this side's header.
    pub(super) fn extra_count(&self) -> usize {
        self.carrier_count.saturating_sub(1)
    }
}

/// One conflict: two symmetric sides whose abilities land on the same cell.
/// Left is the affected unit's own ability; right is the shared ability it
/// clashes with (with a sample carrier as that side's header).
#[derive(Clone, PartialEq)]
pub(super) struct ConflictView {
    left: ConflictSide,
    right: ConflictSide,
}

impl ConflictView {
    pub(super) fn left(&self) -> &ConflictSide {
        &self.left
    }

    pub(super) fn right(&self) -> &ConflictSide {
        &self.right
    }
}

/// The data backing the carriers dialog: the shared ability's name and every
/// unit that carries it, resolved to icons and names when the dialog opens.
#[derive(Clone, PartialEq)]
pub(super) struct CarrierDialogData {
    ability_name: String,
    carriers: Vec<UnitIconView>,
}

impl CarrierDialogData {
    pub(super) fn new(ability_name: String, carrier_unit_ids: &[String]) -> Self {
        let mut carriers: Vec<UnitIconView> = Vec::with_capacity(carrier_unit_ids.len());
        for carrier_unit_id in carrier_unit_ids {
            let carrier = UnitIconView::resolve(carrier_unit_id);
            carriers.push(carrier);
        }
        Self {
            ability_name,
            carriers,
        }
    }

    pub(super) fn ability_name(&self) -> &str {
        &self.ability_name
    }

    pub(super) fn carriers(&self) -> &[UnitIconView] {
        &self.carriers
    }
}

/// A single cross-unit collision island, flattened into display-ready data.
/// All collision facts come from the domain crate; this only formats them.
#[derive(Clone, PartialEq)]
pub(super) struct IslandView {
    key: String,
    position_column: u8,
    position_row: u8,
    conflicts: Vec<ConflictView>,
    collision_count: usize,
}

impl IslandView {
    fn build(group: &CrossUnitPositionGroup) -> Self {
        let position = group.position();
        let position_column = u8::from(position.column());
        let position_row = u8::from(position.row());

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

            let own_ability = AbilityIconView::resolve(own_slot);
            let shared_ability = AbilityIconView::resolve(shared_slot);

            let affected_unit_id_value = affected.unit_id().value();
            let affected_unit = UnitIconView::resolve(affected_unit_id_value);

            // The shared (right) side: its carriers come from the domain entry.
            let mut shared_carrier_unit_ids: Vec<String> =
                Vec::with_capacity(shared_entry.unit_ids().len());
            for carrier_object in shared_entry.unit_ids() {
                let carrier_value = carrier_object.value().to_owned();
                shared_carrier_unit_ids.push(carrier_value);
            }
            let shared_carrier_count = shared_entry.unit_count();
            let sample_carrier_id_option = shared_carrier_unit_ids
                .iter()
                .find(|carrier_id| carrier_id.as_str() != affected_unit_id_value)
                .or_else(|| shared_carrier_unit_ids.first());
            let sample_carrier = sample_carrier_id_option
                .map(|carrier_id| UnitIconView::resolve(carrier_id.as_str()))
                .unwrap_or_else(|| affected_unit.clone());
            let right = ConflictSide {
                unit: sample_carrier,
                ability: shared_ability,
                carrier_unit_ids: shared_carrier_unit_ids,
                carrier_count: shared_carrier_count,
            };

            // The own (left) side: the affected unit heads it. If the own
            // ability is itself shared, list all its carriers; otherwise it is
            // carried only by this unit at this position.
            let own_slot_key = own_slot.as_str();
            let own_shared_entry_option = shared_map.get(own_slot_key).copied();
            let own_carrier_unit_ids: Vec<String> = match own_shared_entry_option {
                Some(entry) => entry
                    .unit_ids()
                    .iter()
                    .map(|carrier_object| carrier_object.value().to_owned())
                    .collect(),
                None => {
                    let single = affected_unit_id_value.to_owned();
                    vec![single]
                }
            };
            let own_carrier_count = match own_shared_entry_option {
                Some(entry) => entry.unit_count(),
                None => 1,
            };
            let left = ConflictSide {
                unit: affected_unit,
                ability: own_ability,
                carrier_unit_ids: own_carrier_unit_ids,
                carrier_count: own_carrier_count,
            };

            let conflict = ConflictView { left, right };
            conflicts.push(conflict);
        }
        let collision_count = affected_entries.len();

        let first_shared_str = shared_entries
            .first()
            .map(|shared| shared.slot_id().as_str())
            .unwrap_or("");
        let key = format!("{position_column}:{position_row}:{first_shared_str}");

        Self {
            key,
            position_column,
            position_row,
            conflicts,
            collision_count,
        }
    }

    pub(super) fn key(&self) -> &str {
        &self.key
    }

    pub(super) fn position_column(&self) -> u8 {
        self.position_column
    }

    pub(super) fn position_row(&self) -> u8 {
        self.position_row
    }

    pub(super) fn conflicts(&self) -> &[ConflictView] {
        &self.conflicts
    }

    pub(super) fn collision_count(&self) -> usize {
        self.collision_count
    }
}

/// Resolves a `GridSlotId` to its icon URL and display name via the database.
struct AbilityResolution {
    icon_url: Option<String>,
    name: String,
}

impl AbilityResolution {
    fn resolve(slot_id: GridSlotId) -> Self {
        let id_value = slot_id.id().value();
        let object_option = ObjectLookup::by_id(id_value);
        let icon_url = object_option
            .and_then(|object| object.icons().first().copied())
            .map(IconUrl::from_database_path)
            .map(|icon| icon.to_string());
        let name_option = object_option.and_then(|object| object.names().first().copied());
        let name = match name_option {
            Some(name) => name.to_owned(),
            None => slot_id.display_name(None, None),
        };
        Self { icon_url, name }
    }
}

/// Turns the domain's `CrossUnitCollisionReport` into a flat island list,
/// in the report's stable row/column/role order.
struct CollisionPageModel;

impl CollisionPageModel {
    fn compute(custom_keys: &CustomKeys) -> Vec<IslandView> {
        let report = CrossUnitCollisionReport::compute(custom_keys);
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

const CENTERED_STATE_CLASS: &str = "collisions-page flex flex-col items-center justify-center \
    gap-[1.25rem] [flex:1_1_0] [min-height:0] text-center p-[2rem] text-warcraft-text-secondary";

#[derive(Props, Clone, PartialEq)]
pub(crate) struct CollisionsPageProps {
    pub(crate) kind: CollisionKind,
    pub(crate) loaded_keys: Signal<Option<CustomKeys>>,
    pub(crate) view_navigation: ViewNavigationContext,
}

/// Top-level Collisions page. For `Positions` it renders an island sidebar
/// (each island a mini command grid with its conflicting button flagged) and
/// a detail pane that lists every conflict. `Hotkeys` lands in a later slice.
#[component]
pub(crate) fn CollisionsPage(props: CollisionsPageProps) -> Element {
    let kind = props.kind;
    let loaded_keys = props.loaded_keys;
    let view_navigation = props.view_navigation;

    let islands_memo = use_memo(move || {
        let guard = loaded_keys.read();
        let Some(custom_keys) = guard.as_ref() else {
            return Vec::new();
        };
        CollisionPageModel::compute(custom_keys)
    });

    let mut selected_island = use_signal(|| None::<String>);

    use_effect(move || {
        let islands = islands_memo.read();
        if islands.is_empty() {
            return;
        }
        let current = selected_island.read().clone();
        let still_valid = match current {
            Some(ref key) => islands.iter().any(|island| island.key() == key),
            None => false,
        };
        if !still_valid {
            let first_key = islands.first().map(|island| island.key().to_owned());
            if let Some(key) = first_key {
                selected_island.set(Some(key));
            }
        }
    });

    let islands = islands_memo();
    let island_count = islands.len();
    let has_file = loaded_keys.read().is_some();
    let sidebar_islands = islands.clone();

    rsx! {
        match kind {
            CollisionKind::Hotkeys => rsx! {
                section {
                    class: CENTERED_STATE_CLASS,
                    "data-collision-kind": "hotkeys",
                    p {
                        class: "m-0 font-friz-quadrata uppercase tracking-[0.12em] text-warcraft-gold [text-shadow:1px_1px_0_#000]",
                        "Hotkey collisions arrive in a later slice."
                    }
                }
            },
            CollisionKind::Positions => rsx! {
                if !has_file {
                    section {
                        class: CENTERED_STATE_CLASS,
                        "data-collision-kind": "positions",
                        "data-island-count": "0",
                        p { class: "m-0", "Upload your CustomKeys.txt to inspect position collisions." }
                    }
                } else if island_count == 0 {
                    section {
                        class: CENTERED_STATE_CLASS,
                        "data-collision-kind": "positions",
                        "data-island-count": "0",
                        span {
                            class: "inline-flex w-[3.5rem] h-[3.5rem] text-warcraft-gold \
                                    [&_svg]:w-full [&_svg]:h-full [filter:drop-shadow(0_0_10px_rgba(255,206,99,0.45))]",
                            aria_hidden: "true",
                            dangerous_inner_html: ICON_COLLISIONS_CLEAR,
                        }
                        p {
                            class: "m-0 font-friz-quadrata uppercase tracking-[0.12em] text-warcraft-gold [text-shadow:1px_1px_0_#000]",
                            "All clear."
                        }
                    }
                } else {
                    div {
                        class: "main-content collisions-content collisions-page",
                        "data-collision-kind": "positions",
                        "data-island-count": "{island_count}",
                        IslandSidebar {
                            islands: sidebar_islands,
                            selected_island,
                        }
                        IslandDetail {
                            islands,
                            selected_island,
                            view_navigation,
                        }
                    }
                }
            },
        }
    }
}
