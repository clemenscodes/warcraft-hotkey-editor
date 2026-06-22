mod detail;
mod hotkey_detail;
mod hotkey_sidebar;
mod mini_grid;
mod sidebar;
mod unit_position_detail;
mod unit_position_sidebar;

use std::collections::HashMap;

use dioxus::prelude::*;
use warcraft_database::ObjectLookup;
use warcraft_keybinds::{
    CrossUnitCollisionReport, CrossUnitPositionGroup, CustomKeys, GridSlotId, SharedAbilityEntry,
    UnitCollisionReport,
};

use crate::components::shared::icons::ICON_COLLISIONS_CLEAR;
use crate::model::grid::GridLayout;
use crate::model::icons::IconUrl;
use crate::services::navigation::app_view::{AppView, CollisionKind};
use crate::services::navigation::view_navigation::ViewNavigationContext;

use detail::IslandDetail;
use hotkey_detail::HotkeyUnitDetail;
use hotkey_sidebar::HotkeyUnitSidebar;
use sidebar::IslandSidebar;
use unit_position_detail::UnitPositionDetail;
use unit_position_sidebar::UnitPositionSidebar;

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

/// One ability participating in a conflict: the ability resolved to an icon,
/// plus the full list of units that carry it (for the carriers dialog) and how
/// many, used for the "+N more" hint.
#[derive(Clone, PartialEq)]
pub(super) struct ConflictAbilityView {
    ability: AbilityIconView,
    carrier_unit_ids: Vec<String>,
    carrier_count: usize,
}

impl ConflictAbilityView {
    pub(super) fn ability(&self) -> &AbilityIconView {
        &self.ability
    }

    pub(super) fn carrier_unit_ids(&self) -> &[String] {
        &self.carrier_unit_ids
    }

    /// Carriers beyond the affected unit already shown on the conflict card.
    pub(super) fn extra_count(&self) -> usize {
        self.carrier_count.saturating_sub(1)
    }
}

/// One conflict: a single affected unit whose two abilities land on the same
/// cell — its own ability and the shared ability it clashes with.
#[derive(Clone, PartialEq)]
pub(super) struct ConflictView {
    unit: UnitIconView,
    own_ability: ConflictAbilityView,
    shared_ability: ConflictAbilityView,
}

impl ConflictView {
    pub(super) fn unit(&self) -> &UnitIconView {
        &self.unit
    }

    pub(super) fn own_ability(&self) -> &ConflictAbilityView {
        &self.own_ability
    }

    pub(super) fn shared_ability(&self) -> &ConflictAbilityView {
        &self.shared_ability
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

            let own_ability_icon = AbilityIconView::resolve(own_slot);
            let shared_ability_icon = AbilityIconView::resolve(shared_slot);

            let affected_unit_id_value = affected.unit_id().value();
            let affected_unit = UnitIconView::resolve(affected_unit_id_value);

            // The shared ability: its carriers come straight from the domain
            // entry. Clicking it lists every unit that carries the ability.
            let mut shared_carrier_unit_ids: Vec<String> =
                Vec::with_capacity(shared_entry.unit_ids().len());
            for carrier_object in shared_entry.unit_ids() {
                let carrier_value = carrier_object.value().to_owned();
                shared_carrier_unit_ids.push(carrier_value);
            }
            let shared_carrier_count = shared_entry.unit_count();
            let shared_ability = ConflictAbilityView {
                ability: shared_ability_icon,
                carrier_unit_ids: shared_carrier_unit_ids,
                carrier_count: shared_carrier_count,
            };

            // The unit's own ability. If it is itself shared, list all its
            // carriers; otherwise it is carried only by this unit at this cell.
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

/// Scrolls the selected sidebar entry into view, matched by its `data-…-key`
/// attribute. `nearest` alignment means an already-visible entry is left in
/// place — only an off-screen restore (e.g. returning from the editor) actually
/// scrolls. No-op on native builds.
#[cfg(target_arch = "wasm32")]
pub(super) fn scroll_entry_into_view(attribute: &str, key: &str) {
    let Some(window) = web_sys::window() else {
        return;
    };
    let Some(document) = window.document() else {
        return;
    };
    let selector = format!("[{attribute}=\"{key}\"]");
    let Ok(Some(element)) = document.query_selector(&selector) else {
        return;
    };
    let options = web_sys::ScrollIntoViewOptions::new();
    options.set_block(web_sys::ScrollLogicalPosition::Center);
    options.set_inline(web_sys::ScrollLogicalPosition::Nearest);
    element.scroll_into_view_with_scroll_into_view_options(&options);
}

#[cfg(not(target_arch = "wasm32"))]
pub(super) fn scroll_entry_into_view(_attribute: &str, _key: &str) {}

/// One hotkey conflict on a unit's command card: a hotkey letter shared by two
/// or more abilities on the same card, resolved to display-ready icons.
#[derive(Clone, PartialEq)]
pub(super) struct HotkeyConflictView {
    hotkey_label: String,
    role_label: String,
    abilities: Vec<AbilityIconView>,
}

impl HotkeyConflictView {
    pub(super) fn hotkey_label(&self) -> &str {
        &self.hotkey_label
    }

    pub(super) fn role_label(&self) -> &str {
        &self.role_label
    }

    pub(super) fn abilities(&self) -> &[AbilityIconView] {
        &self.abilities
    }
}

/// One unit that has hotkey collisions, flattened into display-ready data: the
/// unit header plus every shared-letter conflict across its command cards.
#[derive(Clone, PartialEq)]
pub(super) struct HotkeyUnitView {
    key: String,
    unit: UnitIconView,
    collision_count: usize,
    conflicts: Vec<HotkeyConflictView>,
}

impl HotkeyUnitView {
    pub(super) fn key(&self) -> &str {
        &self.key
    }

    pub(super) fn unit(&self) -> &UnitIconView {
        &self.unit
    }

    pub(super) fn collision_count(&self) -> usize {
        self.collision_count
    }

    pub(super) fn conflicts(&self) -> &[HotkeyConflictView] {
        &self.conflicts
    }
}

/// Turns the domain's `UnitCollisionReport` into a flat list of units with
/// hotkey collisions, each carrying its shared-letter conflicts. Sorted by
/// collision count descending to mirror the position-collision sidebar.
struct HotkeyCollisionPageModel;

impl HotkeyCollisionPageModel {
    fn compute(custom_keys: &CustomKeys, layout: GridLayout) -> Vec<HotkeyUnitView> {
        let report = UnitCollisionReport::compute(custom_keys, layout);
        let entries = report.entries();
        let mut units: Vec<HotkeyUnitView> = Vec::with_capacity(entries.len());
        for entry in entries {
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
                    let mut abilities: Vec<AbilityIconView> =
                        Vec::with_capacity(collision_slots.len());
                    for slot_id in collision_slots.iter() {
                        let ability = AbilityIconView::resolve(slot_id);
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
            if conflicts.is_empty() {
                continue;
            }
            let collision_count = conflicts.len();
            let unit_object_id = entry.unit_id();
            let unit_id_value = unit_object_id.value();
            let unit = UnitIconView::resolve(unit_id_value);
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
        units
    }
}

/// One per-unit position conflict: a command-card cell where two or more of a
/// single unit's own abilities land on the same slot, resolved to icons.
#[derive(Clone, PartialEq)]
pub(super) struct UnitPositionConflictView {
    position_column: u8,
    position_row: u8,
    role_label: String,
    abilities: Vec<AbilityIconView>,
}

impl UnitPositionConflictView {
    pub(super) fn position_column(&self) -> u8 {
        self.position_column
    }

    pub(super) fn position_row(&self) -> u8 {
        self.position_row
    }

    pub(super) fn role_label(&self) -> &str {
        &self.role_label
    }

    pub(super) fn abilities(&self) -> &[AbilityIconView] {
        &self.abilities
    }
}

/// One unit with per-unit position collisions: the unit header plus each cell
/// where its own abilities clash on the same slot.
#[derive(Clone, PartialEq)]
pub(super) struct UnitPositionUnitView {
    key: String,
    unit: UnitIconView,
    collision_count: usize,
    conflicts: Vec<UnitPositionConflictView>,
}

impl UnitPositionUnitView {
    pub(super) fn key(&self) -> &str {
        &self.key
    }

    pub(super) fn unit(&self) -> &UnitIconView {
        &self.unit
    }

    pub(super) fn collision_count(&self) -> usize {
        self.collision_count
    }

    pub(super) fn conflicts(&self) -> &[UnitPositionConflictView] {
        &self.conflicts
    }
}

/// Turns the domain's `UnitCollisionReport` into a flat list of units whose own
/// abilities collide on a command-card cell, each carrying its per-cell
/// conflicts. Sorted by collision count descending.
struct UnitPositionPageModel;

impl UnitPositionPageModel {
    fn compute(custom_keys: &CustomKeys, layout: GridLayout) -> Vec<UnitPositionUnitView> {
        let report = UnitCollisionReport::compute(custom_keys, layout);
        let entries = report.entries();
        let mut units: Vec<UnitPositionUnitView> = Vec::with_capacity(entries.len());
        for entry in entries {
            let position_cards = entry.position_cards();
            let mut conflicts: Vec<UnitPositionConflictView> = Vec::new();
            for card in position_cards {
                if card.is_empty() {
                    continue;
                }
                let role = card.role();
                let role_label = role.label().to_owned();
                for (grid_coordinate, collision_slots) in card {
                    let position_column = u8::from(grid_coordinate.column());
                    let position_row = u8::from(grid_coordinate.row());
                    let mut abilities: Vec<AbilityIconView> =
                        Vec::with_capacity(collision_slots.len());
                    for slot_id in collision_slots.iter() {
                        let ability = AbilityIconView::resolve(slot_id);
                        abilities.push(ability);
                    }
                    let conflict = UnitPositionConflictView {
                        position_column,
                        position_row,
                        role_label: role_label.clone(),
                        abilities,
                    };
                    conflicts.push(conflict);
                }
            }
            if conflicts.is_empty() {
                continue;
            }
            let collision_count = conflicts.len();
            let unit_object_id = entry.unit_id();
            let unit_id_value = unit_object_id.value();
            let unit = UnitIconView::resolve(unit_id_value);
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
        units
    }
}

const CENTERED_STATE_CLASS: &str = "collisions-page flex flex-col items-center justify-center \
    gap-[1.25rem] [flex:1_1_0] [min-height:0] text-center p-[2rem] text-warcraft-text-secondary";

#[derive(Props, Clone, PartialEq)]
struct CollisionBreadcrumbsProps {
    kind: CollisionKind,
    position_count: usize,
    unit_position_count: usize,
    hotkey_count: usize,
    view_navigation: ViewNavigationContext,
}

#[derive(Props, Clone, PartialEq)]
struct CollisionBreadcrumbProps {
    label: &'static str,
    count: usize,
    target_kind: CollisionKind,
    data_breadcrumb: &'static str,
    active: bool,
    view_navigation: ViewNavigationContext,
}

/// A single breadcrumb tab: a label with its live collision count that, when
/// clicked, navigates the collisions view to its kind. Highlighted when active.
#[component]
fn CollisionBreadcrumb(props: CollisionBreadcrumbProps) -> Element {
    let label = props.label;
    let count = props.count;
    let target_kind = props.target_kind;
    let data_breadcrumb = props.data_breadcrumb;
    let active = props.active;
    let view_navigation = props.view_navigation;

    let class_name = if active {
        "collision-breadcrumb active"
    } else {
        "collision-breadcrumb"
    };
    let aria_current = if active { "page" } else { "false" };

    let go_to_kind = move |_| {
        let target = AppView::Collisions { kind: target_kind };
        view_navigation.apply(target);
    };

    rsx! {
        button {
            class: class_name,
            r#type: "button",
            "aria-current": aria_current,
            "data-breadcrumb": data_breadcrumb,
            onclick: go_to_kind,
            span { class: "collision-breadcrumb-label", "{label}" }
            span { class: "collision-breadcrumb-count", "{count}" }
        }
    }
}

/// The breadcrumb bar under the header: one tab per collision kind — cross-unit
/// position, per-unit position, and hotkey — each with its live collision count
/// (counted the same way as the header badge, so the three sum to its total).
/// Clicking a tab swaps the view below.
#[component]
fn CollisionBreadcrumbs(props: CollisionBreadcrumbsProps) -> Element {
    let kind = props.kind;
    let position_count = props.position_count;
    let unit_position_count = props.unit_position_count;
    let hotkey_count = props.hotkey_count;
    let view_navigation = props.view_navigation;

    let positions_active = matches!(kind, CollisionKind::Positions);
    let unit_positions_active = matches!(kind, CollisionKind::UnitPositions);
    let hotkeys_active = matches!(kind, CollisionKind::Hotkeys);

    rsx! {
        nav { class: "collision-breadcrumbs", aria_label: "Collision categories",
            CollisionBreadcrumb {
                label: "Position Collisions",
                count: position_count,
                target_kind: CollisionKind::Positions,
                data_breadcrumb: "positions",
                active: positions_active,
                view_navigation,
            }
            span { class: "collision-breadcrumb-separator", aria_hidden: "true", "|" }
            CollisionBreadcrumb {
                label: "Unit Position Collisions",
                count: unit_position_count,
                target_kind: CollisionKind::UnitPositions,
                data_breadcrumb: "unit-positions",
                active: unit_positions_active,
                view_navigation,
            }
            span { class: "collision-breadcrumb-separator", aria_hidden: "true", "|" }
            CollisionBreadcrumb {
                label: "Hotkey Collisions",
                count: hotkey_count,
                target_kind: CollisionKind::Hotkeys,
                data_breadcrumb: "hotkeys",
                active: hotkeys_active,
                view_navigation,
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub(crate) struct CollisionsPageProps {
    pub(crate) kind: CollisionKind,
    pub(crate) loaded_keys: Signal<Option<CustomKeys>>,
    pub(crate) grid_layout: Signal<GridLayout>,
    pub(crate) view_navigation: ViewNavigationContext,
    /// Selection signals live in `app.rs` so they survive leaving the page (a
    /// unit click → editor) and ride in the `?entry=` URL param. One per kind,
    /// for per-tab memory.
    pub(crate) selected_island: Signal<Option<String>>,
    pub(crate) selected_hotkey_unit: Signal<Option<String>>,
    pub(crate) selected_unit_position: Signal<Option<String>>,
}

/// Top-level Collisions page. `Positions` renders an island sidebar (each
/// island a mini command grid with its conflicting button flagged) and a detail
/// pane listing every cross-unit conflict. `Hotkeys` renders a sidebar of units
/// whose command cards share a hotkey letter and a detail pane of those clashes.
#[component]
pub(crate) fn CollisionsPage(props: CollisionsPageProps) -> Element {
    let kind = props.kind;
    let loaded_keys = props.loaded_keys;
    let grid_layout = props.grid_layout;
    let view_navigation = props.view_navigation;

    let islands_memo = use_memo(move || {
        let guard = loaded_keys.read();
        let Some(custom_keys) = guard.as_ref() else {
            return Vec::new();
        };
        CollisionPageModel::compute(custom_keys)
    });

    let hotkey_units_memo = use_memo(move || {
        let guard = loaded_keys.read();
        let Some(custom_keys) = guard.as_ref() else {
            return Vec::new();
        };
        let layout = *grid_layout.read();
        HotkeyCollisionPageModel::compute(custom_keys, layout)
    });

    let unit_positions_memo = use_memo(move || {
        let guard = loaded_keys.read();
        let Some(custom_keys) = guard.as_ref() else {
            return Vec::new();
        };
        let layout = *grid_layout.read();
        UnitPositionPageModel::compute(custom_keys, layout)
    });

    let mut selected_island = props.selected_island;
    let mut selected_hotkey_unit = props.selected_hotkey_unit;
    let mut selected_unit_position = props.selected_unit_position;

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

    use_effect(move || {
        let hotkey_units = hotkey_units_memo.read();
        if hotkey_units.is_empty() {
            return;
        }
        let current = selected_hotkey_unit.read().clone();
        let still_valid = match current {
            Some(ref key) => hotkey_units.iter().any(|unit| unit.key() == key),
            None => false,
        };
        if !still_valid {
            let first_key = hotkey_units.first().map(|unit| unit.key().to_owned());
            if let Some(key) = first_key {
                selected_hotkey_unit.set(Some(key));
            }
        }
    });

    use_effect(move || {
        let unit_positions = unit_positions_memo.read();
        if unit_positions.is_empty() {
            return;
        }
        let current = selected_unit_position.read().clone();
        let still_valid = match current {
            Some(ref key) => unit_positions.iter().any(|unit| unit.key() == key),
            None => false,
        };
        if !still_valid {
            let first_key = unit_positions.first().map(|unit| unit.key().to_owned());
            if let Some(key) = first_key {
                selected_unit_position.set(Some(key));
            }
        }
    });

    let islands = islands_memo();
    let island_count = islands.len();
    let has_file = loaded_keys.read().is_some();
    let sidebar_islands = islands.clone();

    let hotkey_units = hotkey_units_memo();
    let hotkey_unit_count = hotkey_units.len();
    let hotkey_collision_count = hotkey_units
        .iter()
        .map(|unit_view| unit_view.collision_count())
        .sum::<usize>();
    let sidebar_hotkey_units = hotkey_units.clone();

    let unit_positions = unit_positions_memo();
    let unit_position_unit_count = unit_positions.len();
    let unit_position_collision_count = unit_positions
        .iter()
        .map(|unit_view| unit_view.collision_count())
        .sum::<usize>();
    let sidebar_unit_positions = unit_positions.clone();

    rsx! {
        div { class: "collisions-shell",
            CollisionBreadcrumbs {
                kind,
                position_count: island_count,
                unit_position_count: unit_position_collision_count,
                hotkey_count: hotkey_collision_count,
                view_navigation,
            }
            match kind {
            CollisionKind::Hotkeys => rsx! {
                if !has_file {
                    section {
                        class: CENTERED_STATE_CLASS,
                        "data-collision-kind": "hotkeys",
                        "data-unit-count": "0",
                        p { class: "m-0", "Upload your CustomKeys.txt to inspect hotkey collisions." }
                    }
                } else if hotkey_unit_count == 0 {
                    section {
                        class: CENTERED_STATE_CLASS,
                        "data-collision-kind": "hotkeys",
                        "data-unit-count": "0",
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
                        "data-collision-kind": "hotkeys",
                        "data-unit-count": "{hotkey_unit_count}",
                        HotkeyUnitSidebar {
                            units: sidebar_hotkey_units,
                            selected_unit: selected_hotkey_unit,
                        }
                        HotkeyUnitDetail {
                            units: hotkey_units,
                            selected_unit: selected_hotkey_unit,
                            view_navigation,
                        }
                    }
                }
            },
            CollisionKind::UnitPositions => rsx! {
                if !has_file {
                    section {
                        class: CENTERED_STATE_CLASS,
                        "data-collision-kind": "unit-positions",
                        "data-unit-count": "0",
                        p { class: "m-0", "Upload your CustomKeys.txt to inspect unit position collisions." }
                    }
                } else if unit_position_unit_count == 0 {
                    section {
                        class: CENTERED_STATE_CLASS,
                        "data-collision-kind": "unit-positions",
                        "data-unit-count": "0",
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
                        "data-collision-kind": "unit-positions",
                        "data-unit-count": "{unit_position_unit_count}",
                        UnitPositionSidebar {
                            units: sidebar_unit_positions,
                            selected_unit: selected_unit_position,
                        }
                        UnitPositionDetail {
                            units: unit_positions,
                            selected_unit: selected_unit_position,
                            view_navigation,
                        }
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
}
