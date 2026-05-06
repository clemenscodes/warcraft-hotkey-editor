use std::collections::HashSet;

use warcraft_api::{UnitKind, UnitMeta, WarcraftObjectMeta};
use warcraft_database::WARCRAFT_DATABASE;

use crate::building::BuildingTraits;
use crate::catalog::CommandCatalog;
use crate::lookup::ObjectLookup;
use crate::slot::GridSlotId;

const ROOTED_ONLY_ABILITY_CODES: &[&str] = &["Apit", "Aall"];
const ROOTED_ONLY_ABILITY_IDS: &[&str] = &["Anei"];

fn effective_kind(unit_meta: &UnitMeta) -> UnitKind {
    if unit_meta.is_special() && unit_meta.unit_kind() == UnitKind::Worker {
        return UnitKind::Soldier;
    }
    unit_meta.unit_kind()
}

pub struct UnitSlots;

impl UnitSlots {
    fn morphs_into_self(ability_id: &str, host_unit_id: &str) -> bool {
        let Some(target_id) = ObjectLookup::morph_target_unit(ability_id) else {
            return false;
        };
        if !target_id.eq_ignore_ascii_case(host_unit_id) {
            return false;
        }
        !BuildingTraits::ability_has_alt_state(ability_id)
    }

    fn is_rooted_only_mechanic(ability_id: &str) -> bool {
        if ROOTED_ONLY_ABILITY_IDS
            .iter()
            .any(|id| id.eq_ignore_ascii_case(ability_id))
        {
            return true;
        }
        let Some(ability_code) = ObjectLookup::ability_code(ability_id) else {
            return false;
        };
        ROOTED_ONLY_ABILITY_CODES
            .iter()
            .any(|code| code.eq_ignore_ascii_case(ability_code))
    }

    pub fn all_unit_ids() -> impl Iterator<Item = &'static str> {
        WARCRAFT_DATABASE
            .iter()
            .filter_map(|(database_id, warcraft_object)| {
                if matches!(warcraft_object.meta(), WarcraftObjectMeta::Unit(_)) {
                    Some(database_id.value())
                } else {
                    None
                }
            })
    }

    pub fn command_card_for(unit_id: &str) -> Vec<GridSlotId> {
        let Some(unit_object) = ObjectLookup::by_id(unit_id) else {
            return Vec::new();
        };
        let WarcraftObjectMeta::Unit(unit_meta) = unit_object.meta() else {
            return Vec::new();
        };
        let primary_commands =
            CommandCatalog::primary_commands_for(unit_meta, unit_object.race(), unit_id);
        let unit_kind = effective_kind(unit_meta);
        let regular_abilities = unit_meta.abilities();
        let hero_abilities = unit_meta.hero_abilities();

        let primary_train_slots = if unit_kind == UnitKind::Building {
            unit_meta.trains()
        } else {
            &[]
        };
        let primary_research_slots = if unit_kind == UnitKind::Building {
            unit_meta.researches()
        } else {
            &[]
        };
        let sell_items = if unit_kind == UnitKind::Building {
            unit_meta.sell_items()
        } else {
            &[]
        };
        let sell_units = if unit_kind == UnitKind::Building {
            unit_meta.sell_units()
        } else {
            &[]
        };

        let mut slots: Vec<GridSlotId> = Vec::new();

        for command_name in primary_commands {
            if !ObjectLookup::has_icon(command_name) {
                continue;
            }
            slots.push(GridSlotId::command(command_name));
        }

        let mut seen_train_positions: HashSet<crate::ButtonPosition> = HashSet::new();
        for trained_id in primary_train_slots {
            let id_str = trained_id.value();
            if !ObjectLookup::has_icon(id_str) {
                continue;
            }
            let default_train_position: Option<crate::ButtonPosition> = ObjectLookup::by_id(id_str)
                .and_then(|warcraft_object| warcraft_object.default_button_position())
                .map(|api_position| {
                    let column = api_position.column();
                    let row = api_position.row();
                    crate::ButtonPosition::new(column, row)
                });
            if let Some(train_position) = default_train_position {
                if seen_train_positions.contains(&train_position) {
                    continue;
                }
                seen_train_positions.insert(train_position);
            }
            slots.push(GridSlotId::ability(id_str));
        }

        for research_id in primary_research_slots {
            if !ObjectLookup::has_icon(research_id.value()) {
                continue;
            }
            slots.push(GridSlotId::ability(research_id.value()));
        }
        for sell_item_id in sell_items {
            if !ObjectLookup::has_icon(sell_item_id.value()) {
                continue;
            }
            slots.push(GridSlotId::ability(sell_item_id.value()));
        }
        for sell_unit_id in sell_units {
            if !ObjectLookup::has_icon(sell_unit_id.value()) {
                continue;
            }
            slots.push(GridSlotId::ability(sell_unit_id.value()));
        }

        let is_uprootable = BuildingTraits::can_uproot(unit_id);
        let host_is_burrowed = BuildingTraits::is_burrowed_form(unit_id);
        let host_is_in_alt_state = BuildingTraits::unit_starts_in_toggle_alt_state(unit_id);

        for ability_id in regular_abilities.iter().chain(hero_abilities.iter()) {
            if hero_abilities.contains(ability_id) {
                let is_levelable = ObjectLookup::by_id(ability_id.value())
                    .map(|object| match object.meta() {
                        WarcraftObjectMeta::Ability(meta) => {
                            meta.max_level() > 1 || meta.is_ultimate()
                        }
                        _ => true,
                    })
                    .unwrap_or(true);
                if !is_levelable {
                    continue;
                }
            }
            if is_uprootable && ability_id.value().eq_ignore_ascii_case("Aeat") {
                continue;
            }
            if host_is_burrowed && !BuildingTraits::ability_has_alt_state(ability_id.value()) {
                continue;
            }
            if Self::morphs_into_self(ability_id.value(), unit_id) {
                continue;
            }
            if !ObjectLookup::has_icon(ability_id.value()) {
                continue;
            }
            let is_morph_back = ObjectLookup::morph_target_unit(ability_id.value())
                .is_some_and(|target| target.eq_ignore_ascii_case(unit_id));
            if is_morph_back
                || (host_is_in_alt_state
                    && BuildingTraits::ability_has_alt_state(ability_id.value()))
            {
                slots.push(GridSlotId::ability_off(ability_id.value()));
            } else {
                slots.push(GridSlotId::ability(ability_id.value()));
            }
        }

        if unit_kind == UnitKind::Hero
            && !hero_abilities.is_empty()
            && let Some(select_skill) = CommandCatalog::known_command("CmdSelectSkill")
            && ObjectLookup::has_icon(select_skill)
        {
            slots.push(GridSlotId::command(select_skill));
        }

        slots
    }

    pub fn build_menu_for(unit_id: &str) -> Option<Vec<GridSlotId>> {
        let unit_object = ObjectLookup::by_id(unit_id)?;
        let WarcraftObjectMeta::Unit(unit_meta) = unit_object.meta() else {
            return None;
        };
        if effective_kind(unit_meta) != UnitKind::Worker {
            return None;
        }
        if unit_meta.builds().is_empty() {
            return None;
        }
        let build_menu_commands = CommandCatalog::build_menu_commands_for(unit_meta);
        let mut slots: Vec<GridSlotId> = Vec::new();
        for command_name in build_menu_commands {
            if !ObjectLookup::has_icon(command_name) {
                continue;
            }
            slots.push(GridSlotId::command(command_name));
        }
        for production_id in unit_meta.builds() {
            if !ObjectLookup::has_icon(production_id.value()) {
                continue;
            }
            slots.push(GridSlotId::ability(production_id.value()));
        }
        Some(slots)
    }

    pub fn research_menu_for(unit_id: &str) -> Option<Vec<GridSlotId>> {
        let unit_object = ObjectLookup::by_id(unit_id)?;
        let WarcraftObjectMeta::Unit(unit_meta) = unit_object.meta() else {
            return None;
        };
        if effective_kind(unit_meta) != UnitKind::Hero {
            return None;
        }
        let hero_abilities = unit_meta.hero_abilities();
        if hero_abilities.is_empty() {
            return None;
        }
        let mut slots: Vec<GridSlotId> = Vec::new();
        for ability_id in hero_abilities.iter() {
            if !ObjectLookup::has_icon(ability_id.value()) {
                continue;
            }
            slots.push(GridSlotId::ability(ability_id.value()));
        }
        if let Some(back_command) = CommandCatalog::submenu_back_command()
            && ObjectLookup::has_icon(back_command)
        {
            slots.push(GridSlotId::command(back_command));
        }
        Some(slots)
    }

    pub fn uprooted_menu_for(unit_id: &str) -> Option<Vec<GridSlotId>> {
        let unit_object = ObjectLookup::by_id(unit_id)?;
        let WarcraftObjectMeta::Unit(unit_meta) = unit_object.meta() else {
            return None;
        };
        if effective_kind(unit_meta) != UnitKind::Building {
            return None;
        }
        if !BuildingTraits::can_uproot(unit_id) {
            return None;
        }
        let mut slots: Vec<GridSlotId> = Vec::new();
        for command_name in CommandCatalog::mobile_command_ids().iter().copied() {
            if !ObjectLookup::has_icon(command_name) {
                continue;
            }
            slots.push(GridSlotId::command(command_name));
        }
        for ability_id in unit_meta.abilities() {
            if !ObjectLookup::has_icon(ability_id.value()) {
                continue;
            }
            if Self::morphs_into_self(ability_id.value(), unit_id) {
                continue;
            }
            if Self::is_rooted_only_mechanic(ability_id.value()) {
                continue;
            }
            slots.push(GridSlotId::ability(ability_id.value()));
        }
        Some(slots)
    }
}
