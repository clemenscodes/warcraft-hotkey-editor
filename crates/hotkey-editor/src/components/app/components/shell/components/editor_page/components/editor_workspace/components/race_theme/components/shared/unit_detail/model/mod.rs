use super::presentation::{ResolvedUnit, UnitCommandGridSlots, UnitOverrideTarget};
use super::state::UnitDetailModel;
use std::rc::Rc;
use warcraft_api::WarcraftObjectId;
use warcraft_keybinds::{GridSlotId, InspectorDetail};

pub(super) struct UnitDetailInputs {
    pub(super) unit_id: WarcraftObjectId,
    pub(super) resolved_unit: ResolvedUnit,
    pub(super) command_card_slots: Rc<[GridSlotId]>,
    pub(super) build_menu_slots: Option<Rc<[GridSlotId]>>,
    pub(super) uprooted_menu_slots: Option<Rc<[GridSlotId]>>,
    pub(super) research_menu_slots: Option<Rc<[GridSlotId]>>,
    pub(super) detail: Option<InspectorDetail>,
    pub(super) active_container_slots: Rc<[GridSlotId]>,
}

impl From<UnitDetailInputs> for UnitDetailModel {
    fn from(inputs: UnitDetailInputs) -> Self {
        let UnitDetailInputs {
            unit_id,
            resolved_unit,
            command_card_slots,
            build_menu_slots,
            uprooted_menu_slots,
            research_menu_slots,
            detail,
            active_container_slots,
        } = inputs;
        let ResolvedUnit {
            unit_name,
            portrait_url,
            description_text,
            combat,
            hero_attributes,
            evasion,
        } = resolved_unit;
        let has_hero_attributes = hero_attributes.is_some();
        let grid_slots = UnitCommandGridSlots {
            unit_id,
            command_card_slots,
            build_menu_slots,
            uprooted_menu_slots,
            research_menu_slots,
        };
        let override_target = UnitOverrideTarget {
            detail,
            active_container_slots,
        };
        Self {
            unit_name,
            unit_id,
            portrait_url,
            has_hero_attributes,
            description_text,
            combat,
            hero_attributes,
            evasion,
            grid_slots,
            override_target,
        }
    }
}
