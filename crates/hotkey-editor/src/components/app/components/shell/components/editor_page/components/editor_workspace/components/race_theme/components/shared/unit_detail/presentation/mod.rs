use super::model::UnitDetailInputs;
use super::state::{UnitDetailModel, UnitDetailView};
use crate::components::app::components::shell::components::shared::icons::IconUrl;
use crate::services::customkeys::context::use_loaded_keys;
use crate::services::editor_state::context::use_editor_state;
use crate::services::navigation::context::use_view_navigation;
use dioxus::prelude::*;
use std::collections::HashMap;
use std::rc::Rc;
use warcraft_api::WarcraftApi;
use warcraft_api::{HeroAttributes, UnitCombat, WarcraftObjectId, WarcraftObjectMeta};
use warcraft_keybinds::{CustomKeys, Evasion, GridSlotId, InspectorDetail, UnitSlotContainers};

/// The unit's command-grid slot containers: the always-present command card plus the
/// optional build, uprooted, and research menus, keyed to the host unit. Threaded down
/// the card body to the grids, which read the shared editor signals from context.
#[derive(Clone, PartialEq)]
pub(crate) struct UnitCommandGridSlots {
    pub(super) unit_id: WarcraftObjectId,
    pub(super) command_card_slots: Rc<[GridSlotId]>,
    pub(super) build_menu_slots: Option<Rc<[GridSlotId]>>,
    pub(super) uprooted_menu_slots: Option<Rc<[GridSlotId]>>,
    pub(super) research_menu_slots: Option<Rc<[GridSlotId]>>,
}

/// The hotkey-override target: the inspected slot's detail (absent when nothing is
/// selected) and the container slots the override card edits against.
#[derive(Clone, PartialEq)]
pub(crate) struct UnitOverrideTarget {
    pub(super) detail: Option<InspectorDetail>,
    pub(super) active_container_slots: Rc<[GridSlotId]>,
}

/// The selected unit resolved from the game database: its name, portrait, flavor
/// text, combat block, optional hero attributes, and evasion. Resolution fails with
/// the empty-state message the panel should show instead.
#[derive(Clone, PartialEq)]
pub(super) struct ResolvedUnit {
    pub(super) unit_name: &'static str,
    pub(super) portrait_url: Option<String>,
    pub(super) description_text: String,
    pub(super) combat: UnitCombat,
    pub(super) hero_attributes: Option<HeroAttributes>,
    pub(super) evasion: Evasion,
}

impl TryFrom<WarcraftObjectId> for ResolvedUnit {
    type Error = &'static str;

    fn try_from(unit_id: WarcraftObjectId) -> Result<Self, Self::Error> {
        let api = WarcraftApi::default();
        let Some(unit_object) = api.object(unit_id) else {
            return Err("Unit not found in database.");
        };
        let WarcraftObjectMeta::Unit(unit_meta) = unit_object.meta() else {
            return Err("Selected object is not a unit.");
        };
        let unit_name = unit_object.names().first().copied().unwrap_or("(unnamed)");
        let portrait_url = unit_object
            .icons()
            .first()
            .copied()
            .map(IconUrl::from_database_path)
            .map(|url| url.to_string());
        let description_text = unit_object.ubertip().unwrap_or_default().to_string();
        let combat = *unit_meta.combat();
        let hero_attributes = unit_meta.hero_attributes().copied();
        let evasion = Evasion::resolve(unit_meta);
        Ok(Self {
            unit_name,
            portrait_url,
            description_text,
            combat,
            hero_attributes,
            evasion,
        })
    }
}

/// The inspector panel for the currently-selected slot, resolved from its binding.
/// `None` when no slot is selected. The upgrade-form unit id is looked up from the
/// unit's train-upgrade map for ability slots.
#[derive(Clone, PartialEq)]
pub(super) struct InspectorPanel {
    pub(super) detail: Option<InspectorDetail>,
}

/// The inputs that shape an [`InspectorPanel`]: the selected slot, the live
/// document, the host unit, the research / uprooted flags, and the unit's
/// train-upgrade map.
pub(super) struct InspectorPanelInputs<'a> {
    pub(super) inspector_slot: &'a Option<GridSlotId>,
    pub(super) custom_keys: &'a Option<CustomKeys>,
    pub(super) host_unit_id: WarcraftObjectId,
    pub(super) from_uprooted: bool,
    pub(super) from_research: bool,
    pub(super) train_upgrades: &'a HashMap<WarcraftObjectId, WarcraftObjectId>,
}

impl From<InspectorPanelInputs<'_>> for InspectorPanel {
    fn from(inputs: InspectorPanelInputs<'_>) -> Self {
        let InspectorPanelInputs {
            inspector_slot,
            custom_keys,
            host_unit_id,
            from_uprooted,
            from_research,
            train_upgrades,
        } = inputs;
        let detail = inspector_slot.as_ref().map(|slot| {
            let upgrade_id = if let GridSlotId::Ability(id) = slot {
                train_upgrades.get(&id.object_id()).copied()
            } else {
                None
            };
            InspectorDetail::build(
                slot,
                custom_keys,
                host_unit_id,
                from_uprooted,
                from_research,
                upgrade_id,
            )
        });
        Self { detail }
    }
}

/// Which of the unit's containers the override panel edits against, selected from the
/// research / uprooted flags and, in the default case, whether the inspected slot is
/// a build-menu entry rather than a command-card one.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct ActiveContainer {
    pub(super) slots: Rc<[GridSlotId]>,
}

/// The inputs that select an [`ActiveContainer`]: the unit's containers, the
/// inspected slot, and the research / uprooted flags.
pub(super) struct ActiveContainerInputs<'a> {
    pub(super) containers: &'a UnitSlotContainers,
    pub(super) inspector_slot: &'a Option<GridSlotId>,
    pub(super) from_uprooted: bool,
    pub(super) from_research: bool,
}

impl From<ActiveContainerInputs<'_>> for ActiveContainer {
    fn from(inputs: ActiveContainerInputs<'_>) -> Self {
        let ActiveContainerInputs {
            containers,
            inspector_slot,
            from_uprooted,
            from_research,
        } = inputs;
        let empty_slot_list: Rc<[GridSlotId]> = Rc::from(Vec::<GridSlotId>::new());
        let slots: Rc<[GridSlotId]> = if from_uprooted {
            containers
                .uprooted()
                .unwrap_or_else(|| empty_slot_list.clone())
        } else if from_research {
            containers
                .research()
                .unwrap_or_else(|| empty_slot_list.clone())
        } else {
            let in_build_menu = inspector_slot
                .as_ref()
                .is_some_and(|slot| containers.build_menu_contains(slot));
            if in_build_menu {
                containers
                    .build_menu()
                    .unwrap_or_else(|| empty_slot_list.clone())
            } else {
                containers.command_card()
            }
        };
        Self { slots }
    }
}

/// Resets the selected hero level to its default whenever the selected unit changes.
/// The picker owns its own open state, and the stats panel reads the chosen level from
/// context, so this hook only owns the reset effect.
fn use_hero_level_reset(selected_unit_id: Signal<Option<WarcraftObjectId>>) {
    let mut selected_hero_level = use_editor_state().selected_hero_level();
    use_effect(move || {
        let _ = selected_unit_id.read();
        selected_hero_level.set(1);
    });
}

/// Resolves the selected unit and shapes every child's props. The domain work is
/// grouped into the [`ResolvedUnit`], [`InspectorPanel`], and [`ActiveContainer`]
/// derivations plus the memoized [`UnitSlotContainers`]; this hook only orchestrates
/// them, gathers the [`UnitDetailInputs`], and lets the props tree derive itself.
pub(super) fn use_unit_detail_panel() -> UnitDetailView {
    let navigation = use_view_navigation();
    let selected_unit_id = navigation.selected_unit_id();
    let editor = use_editor_state();
    let selected_slot = editor.selected_slot();
    let selected_from_research = editor.selected_from_research();
    let selected_from_uprooted = editor.selected_from_uprooted();
    let loaded_keys = use_loaded_keys();
    use_hero_level_reset(selected_unit_id);
    let slot_data_memo = use_memo(move || {
        let unit_id_option = *selected_unit_id.read();
        let unit_id = unit_id_option.unwrap_or_default();
        UnitSlotContainers::resolve(unit_id)
    });
    let unit_id_option = *selected_unit_id.read();
    let Some(unit_id) = unit_id_option else {
        return UnitDetailView::Empty("Select a unit to view its command card.");
    };
    let resolved_unit = match ResolvedUnit::try_from(unit_id) {
        Ok(resolved) => resolved,
        Err(message) => return UnitDetailView::Empty(message),
    };
    let slot_containers = slot_data_memo.read();
    let command_card_slots = slot_containers.command_card();
    let build_menu_slots = slot_containers.build_menu();
    let uprooted_menu_slots = slot_containers.uprooted();
    let research_menu_slots = slot_containers.research();
    let inspector_slot = *selected_slot.read();
    let inspector_from_uprooted = *selected_from_uprooted.read();
    let inspector_from_research = *selected_from_research.read();
    let keys_guard = loaded_keys.read();
    let train_upgrades = slot_containers.train_upgrades();
    let custom_keys_ref: &Option<CustomKeys> = &keys_guard;
    let inspector_inputs = InspectorPanelInputs {
        inspector_slot: &inspector_slot,
        custom_keys: custom_keys_ref,
        host_unit_id: unit_id,
        from_uprooted: inspector_from_uprooted,
        from_research: inspector_from_research,
        train_upgrades,
    };
    let inspector_panel = InspectorPanel::from(inspector_inputs);
    drop(keys_guard);
    let containers_ref: &UnitSlotContainers = &slot_containers;
    let active_container_inputs = ActiveContainerInputs {
        containers: containers_ref,
        inspector_slot: &inspector_slot,
        from_uprooted: inspector_from_uprooted,
        from_research: inspector_from_research,
    };
    let active_container = ActiveContainer::from(active_container_inputs);
    let active_container_slots = active_container.slots;
    let detail = inspector_panel.detail;
    let inputs = UnitDetailInputs {
        unit_id,
        resolved_unit,
        command_card_slots,
        build_menu_slots,
        uprooted_menu_slots,
        research_menu_slots,
        detail,
        active_container_slots,
    };
    let model = UnitDetailModel::from(inputs);
    let boxed_model = Box::new(model);
    UnitDetailView::Loaded(boxed_model)
}
