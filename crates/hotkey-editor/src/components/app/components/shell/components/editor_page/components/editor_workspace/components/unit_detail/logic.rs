use crate::components::app::components::shell::components::shared::icons::IconUrl;
use std::collections::HashMap;
use std::rc::Rc;
use warcraft_api::{HeroAttributes, UnitCombat, WarcraftObjectId, WarcraftObjectMeta};
use warcraft_database::ObjectLookup;
use warcraft_keybinds::{CustomKeys, Evasion, GridSlotId, InspectorDetail, UnitSlotContainers};

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

impl ResolvedUnit {
    pub(super) fn resolve(unit_id: &str) -> Result<Self, &'static str> {
        let Some(unit_object) = ObjectLookup::by_id(unit_id) else {
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

impl InspectorPanel {
    pub(super) fn resolve(
        inspector_slot: &Option<GridSlotId>,
        custom_keys: &Option<CustomKeys>,
        host_unit_id: &str,
        from_uprooted: bool,
        from_research: bool,
        train_upgrades: &HashMap<WarcraftObjectId, WarcraftObjectId>,
    ) -> Self {
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

impl ActiveContainer {
    pub(super) fn resolve(
        containers: &UnitSlotContainers,
        inspector_slot: &Option<GridSlotId>,
        from_uprooted: bool,
        from_research: bool,
    ) -> Self {
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
            let inspector_slot_id = inspector_slot
                .as_ref()
                .map(|slot| slot.as_str().to_string());
            let build_menu_slots = containers.build_menu();
            let in_build_menu = inspector_slot_id.as_deref().is_some_and(|id_value| {
                build_menu_slots.as_ref().is_some_and(|list| {
                    list.iter()
                        .any(|candidate| candidate.as_str().eq_ignore_ascii_case(id_value))
                })
            });
            if in_build_menu {
                build_menu_slots.unwrap_or_else(|| empty_slot_list.clone())
            } else {
                containers.command_card()
            }
        };
        Self { slots }
    }
}
