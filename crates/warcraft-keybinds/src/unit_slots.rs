use std::collections::HashSet;

use warcraft_api::{UnitKind, WarcraftObjectMeta};
use warcraft_database::WARCRAFT_DATABASE;

use crate::ButtonPosition;
use crate::building::BuildingTraits;
use crate::catalog::CommandCatalog;
use crate::slot::GridSlotId;

const ROOTED_ONLY_ABILITY_CODES: &[&str] = &["Apit", "Aall"];
const ROOTED_ONLY_ABILITY_IDS: &[&str] = &["Anei"];

pub struct UnitSlots;

impl UnitSlots {
    fn morphs_into_self(ability_id: &str, host_unit_id: &str) -> bool {
        let ability_object = WARCRAFT_DATABASE.by_id(ability_id);
        let Some(target_id) = ability_object.and_then(|object| object.ability_morph_target_id())
        else {
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
        let ability_object = WARCRAFT_DATABASE.by_id(ability_id);
        let Some(ability_code) = ability_object.and_then(|object| object.ability_code()) else {
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
        let Some(unit_object) = WARCRAFT_DATABASE.by_id(unit_id) else {
            return Vec::new();
        };
        let WarcraftObjectMeta::Unit(unit_meta) = unit_object.meta() else {
            return Vec::new();
        };
        let unit_race = unit_object.race();
        let primary_commands = CommandCatalog::primary_commands_for(unit_meta, unit_race, unit_id);
        let unit_kind = CommandCatalog::effective_kind(unit_meta);
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
            let command_database_object = WARCRAFT_DATABASE.by_id(command_name);
            let command_has_icon =
                command_database_object.is_some_and(|o| o.has_displayable_icon());
            if !command_has_icon {
                continue;
            }
            slots.push(GridSlotId::command(command_name));
        }

        let mut seen_train_positions: HashSet<ButtonPosition> = HashSet::new();
        for trained_id in primary_train_slots {
            let id_str = trained_id.value();
            let trained_database_object = WARCRAFT_DATABASE.by_id(id_str);
            let trained_has_icon =
                trained_database_object.is_some_and(|o| o.has_displayable_icon());
            if !trained_has_icon {
                continue;
            }
            let train_object = WARCRAFT_DATABASE.by_id(id_str);
            let api_position = train_object.and_then(|object| object.default_button_position());
            let default_train_position = api_position.map(|pos| {
                let column = pos.column();
                let row = pos.row();
                ButtonPosition::new(column, row)
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
            let research_id_str = research_id.value();
            let research_database_object = WARCRAFT_DATABASE.by_id(research_id_str);
            let research_has_icon =
                research_database_object.is_some_and(|o| o.has_displayable_icon());
            if !research_has_icon {
                continue;
            }
            slots.push(GridSlotId::ability(research_id_str));
        }
        for sell_item_id in sell_items {
            let sell_item_id_str = sell_item_id.value();
            let sell_item_database_object = WARCRAFT_DATABASE.by_id(sell_item_id_str);
            let sell_item_has_icon =
                sell_item_database_object.is_some_and(|o| o.has_displayable_icon());
            if !sell_item_has_icon {
                continue;
            }
            slots.push(GridSlotId::ability(sell_item_id_str));
        }
        for sell_unit_id in sell_units {
            let sell_unit_id_str = sell_unit_id.value();
            let sell_unit_database_object = WARCRAFT_DATABASE.by_id(sell_unit_id_str);
            let sell_unit_has_icon =
                sell_unit_database_object.is_some_and(|o| o.has_displayable_icon());
            if !sell_unit_has_icon {
                continue;
            }
            slots.push(GridSlotId::ability(sell_unit_id_str));
        }

        let is_uprootable = BuildingTraits::can_uproot(unit_id);
        let host_is_burrowed = BuildingTraits::is_burrowed_form(unit_id);
        let host_is_in_alt_state = BuildingTraits::unit_starts_in_toggle_alt_state(unit_id);

        for ability_id in regular_abilities.iter().chain(hero_abilities.iter()) {
            let ability_id_str = ability_id.value();
            if hero_abilities.contains(ability_id) {
                let levelable_object = WARCRAFT_DATABASE.by_id(ability_id_str);
                let is_levelable = levelable_object
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
            if is_uprootable && ability_id_str.eq_ignore_ascii_case("Aeat") {
                continue;
            }
            if host_is_burrowed && !BuildingTraits::ability_has_alt_state(ability_id_str) {
                continue;
            }
            if Self::morphs_into_self(ability_id_str, unit_id) {
                continue;
            }
            let ability_database_object = WARCRAFT_DATABASE.by_id(ability_id_str);
            let ability_has_icon =
                ability_database_object.is_some_and(|o| o.has_displayable_icon());
            if !ability_has_icon {
                continue;
            }
            let morph_target_object = WARCRAFT_DATABASE.by_id(ability_id_str);
            let morph_target_id =
                morph_target_object.and_then(|object| object.ability_morph_target_id());
            let is_morph_back =
                morph_target_id.is_some_and(|target| target.eq_ignore_ascii_case(unit_id));
            if is_morph_back
                || (host_is_in_alt_state && BuildingTraits::ability_has_alt_state(ability_id_str))
            {
                slots.push(GridSlotId::ability_off(ability_id_str));
            } else {
                slots.push(GridSlotId::ability(ability_id_str));
            }
        }

        if unit_kind == UnitKind::Hero
            && !hero_abilities.is_empty()
            && let Some(select_skill) = CommandCatalog::known_command("CmdSelectSkill")
        {
            let select_skill_database_object = WARCRAFT_DATABASE.by_id(select_skill);
            let select_skill_has_icon =
                select_skill_database_object.is_some_and(|o| o.has_displayable_icon());
            if select_skill_has_icon {
                slots.push(GridSlotId::command(select_skill));
            }
        }

        slots
    }

    pub fn build_menu_for(unit_id: &str) -> Option<Vec<GridSlotId>> {
        let unit_object = WARCRAFT_DATABASE.by_id(unit_id)?;
        let WarcraftObjectMeta::Unit(unit_meta) = unit_object.meta() else {
            return None;
        };
        if CommandCatalog::effective_kind(unit_meta) != UnitKind::Worker {
            return None;
        }
        if unit_meta.builds().is_empty() {
            return None;
        }
        let build_menu_commands = CommandCatalog::build_menu_commands_for(unit_meta);
        let mut slots: Vec<GridSlotId> = Vec::new();
        for command_name in build_menu_commands {
            let command_database_object = WARCRAFT_DATABASE.by_id(command_name);
            let command_has_icon =
                command_database_object.is_some_and(|o| o.has_displayable_icon());
            if !command_has_icon {
                continue;
            }
            slots.push(GridSlotId::command(command_name));
        }
        for production_id in unit_meta.builds() {
            let production_id_str = production_id.value();
            let production_database_object = WARCRAFT_DATABASE.by_id(production_id_str);
            let production_has_icon =
                production_database_object.is_some_and(|o| o.has_displayable_icon());
            if !production_has_icon {
                continue;
            }
            slots.push(GridSlotId::ability(production_id_str));
        }
        Some(slots)
    }

    pub fn research_menu_for(unit_id: &str) -> Option<Vec<GridSlotId>> {
        let unit_object = WARCRAFT_DATABASE.by_id(unit_id)?;
        let WarcraftObjectMeta::Unit(unit_meta) = unit_object.meta() else {
            return None;
        };
        if CommandCatalog::effective_kind(unit_meta) != UnitKind::Hero {
            return None;
        }
        let hero_abilities = unit_meta.hero_abilities();
        if hero_abilities.is_empty() {
            return None;
        }
        let mut slots: Vec<GridSlotId> = Vec::new();
        for ability_id in hero_abilities.iter() {
            let ability_id_str = ability_id.value();
            let ability_database_object = WARCRAFT_DATABASE.by_id(ability_id_str);
            let ability_has_icon =
                ability_database_object.is_some_and(|o| o.has_displayable_icon());
            if !ability_has_icon {
                continue;
            }
            slots.push(GridSlotId::ability(ability_id_str));
        }
        if let Some(back_command) = CommandCatalog::submenu_back_command() {
            let back_command_database_object = WARCRAFT_DATABASE.by_id(back_command);
            let back_command_has_icon =
                back_command_database_object.is_some_and(|o| o.has_displayable_icon());
            if back_command_has_icon {
                slots.push(GridSlotId::command(back_command));
            }
        }
        Some(slots)
    }

    pub fn uprooted_menu_for(unit_id: &str) -> Option<Vec<GridSlotId>> {
        let unit_object = WARCRAFT_DATABASE.by_id(unit_id)?;
        let WarcraftObjectMeta::Unit(unit_meta) = unit_object.meta() else {
            return None;
        };
        if CommandCatalog::effective_kind(unit_meta) != UnitKind::Building {
            return None;
        }
        if !BuildingTraits::can_uproot(unit_id) {
            return None;
        }
        let mut slots: Vec<GridSlotId> = Vec::new();
        for command_name in CommandCatalog::mobile_command_ids().iter().copied() {
            let command_database_object = WARCRAFT_DATABASE.by_id(command_name);
            let command_has_icon =
                command_database_object.is_some_and(|o| o.has_displayable_icon());
            if !command_has_icon {
                continue;
            }
            slots.push(GridSlotId::command(command_name));
        }
        for ability_id in unit_meta.abilities() {
            let ability_id_str = ability_id.value();
            let ability_database_object = WARCRAFT_DATABASE.by_id(ability_id_str);
            let ability_has_icon =
                ability_database_object.is_some_and(|o| o.has_displayable_icon());
            if !ability_has_icon {
                continue;
            }
            if Self::morphs_into_self(ability_id_str, unit_id) {
                continue;
            }
            if Self::is_rooted_only_mechanic(ability_id_str) {
                continue;
            }
            slots.push(GridSlotId::ability(ability_id_str));
        }
        Some(slots)
    }
}
