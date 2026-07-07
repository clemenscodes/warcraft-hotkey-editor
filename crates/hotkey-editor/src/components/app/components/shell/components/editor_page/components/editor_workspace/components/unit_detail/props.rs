use super::components::unit_description::UnitDescriptionProps;
use super::components::unit_detail_body::UnitDetailBodyProps;
use super::components::unit_detail_body::components::unit_detail_row::UnitDetailRowProps;
use super::components::unit_detail_body::components::unit_detail_row::components::unit_command_grids::UnitCommandGridsProps;
use super::components::unit_detail_body::components::unit_detail_row::components::unit_tile_override::UnitTileOverrideProps;
use super::components::unit_detail_header::UnitDetailHeaderProps;
use super::components::unit_stats_panel::UnitStatsPanelProps;
use super::logic::ResolvedUnit;
use super::state::UnitDetailModel;
use dioxus::prelude::*;
use std::rc::Rc;
use warcraft_api::Race;
use warcraft_keybinds::{GridSlotId, InspectorDetail};

/// The unit detail panel's inputs: the active race and the selected unit. The editor
/// signals its grids and override card drive are sourced from context by those
/// children's own hooks, so the panel is fed no god-bag of signals as props.
#[derive(Props, Clone, PartialEq)]
pub struct UnitDetailPanelProps {
    pub active_race: Signal<Race>,
    pub selected_unit_id: Signal<Option<String>>,
}

/// Every computed intermediate the loaded panel's props tree is built from. The hook
/// resolves the selected unit, its slot containers, and the inspector detail into one
/// of these; the whole child props tree then derives itself through the `From` impl
/// below, so the hook never assembles a props struct by hand.
pub(super) struct UnitDetailInputs {
    pub(super) race: Race,
    pub(super) unit_id: String,
    pub(super) resolved_unit: ResolvedUnit,
    pub(super) selected_hero_level: Signal<u32>,
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
            race,
            unit_id,
            resolved_unit,
            selected_hero_level,
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
        let header_unit_id = unit_id.clone();
        let header = UnitDetailHeaderProps {
            unit_name,
            unit_id: header_unit_id,
            portrait_url,
            has_hero_attributes,
        };
        let description = UnitDescriptionProps {
            text: description_text,
        };
        let stats = UnitStatsPanelProps {
            combat,
            hero_attributes,
            selected_hero_level,
            evasion,
        };
        let grids = UnitCommandGridsProps {
            unit_id,
            race,
            command_card_slots,
            build_menu_slots,
            uprooted_menu_slots,
            research_menu_slots,
        };
        let tile_override = UnitTileOverrideProps {
            detail,
            active_container_slots,
        };
        let row = UnitDetailRowProps {
            grids,
            tile_override,
        };
        let body = UnitDetailBodyProps { row };
        Self {
            header,
            description,
            stats,
            body,
        }
    }
}
