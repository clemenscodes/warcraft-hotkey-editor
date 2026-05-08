use std::collections::HashMap;

use warcraft_api::{UnitKind, WarcraftDatabase, WarcraftObjectId, WarcraftObjectMeta};

use warcraft_database::{BuildingTraits, CommandCatalog};

use crate::GridCoordinate;
use crate::slot::{CommandCard, GridSlotId};

const ROOTED_ONLY_ABILITY_CODES: &[WarcraftObjectId] =
    &[WarcraftObjectId::new("Apit"), WarcraftObjectId::new("Aall")];
const ROOTED_ONLY_ABILITY_IDS: &[WarcraftObjectId] = &[WarcraftObjectId::new("Anei")];

pub trait UnitCommandSlots {
    fn command_card(&self, unit_id: WarcraftObjectId) -> CommandCard;
    fn build_menu(&self, unit_id: WarcraftObjectId) -> Option<CommandCard>;
    fn research_menu(&self, unit_id: WarcraftObjectId) -> Option<CommandCard>;
    fn uprooted_menu(&self, unit_id: WarcraftObjectId) -> Option<CommandCard>;
    fn train_unit_upgrades(
        &self,
        unit_id: WarcraftObjectId,
    ) -> HashMap<WarcraftObjectId, WarcraftObjectId>;
    fn all_unit_ids(&self) -> impl Iterator<Item = WarcraftObjectId>;
}

fn ability_reverts_to_host(
    database: &WarcraftDatabase,
    ability_id: &str,
    host_unit_id: &str,
) -> bool {
    let ability_object = database.by_id(ability_id);
    let Some(target_id) = ability_object.and_then(|object| object.ability_morph_target_id()) else {
        return false;
    };
    if !target_id.eq_ignore_ascii_case(host_unit_id) {
        return false;
    }
    !BuildingTraits::ability_has_alt_state(ability_id)
}

fn ability_requires_rooted_form(database: &WarcraftDatabase, ability_id: &str) -> bool {
    if ROOTED_ONLY_ABILITY_IDS
        .iter()
        .any(|id| id.value().eq_ignore_ascii_case(ability_id))
    {
        return true;
    }
    let ability_object = database.by_id(ability_id);
    let Some(ability_code) = ability_object.and_then(|object| object.ability_code()) else {
        return false;
    };
    ROOTED_ONLY_ABILITY_CODES
        .iter()
        .any(|code| code.value().eq_ignore_ascii_case(ability_code))
}

fn slot_position_from_database(
    database: &WarcraftDatabase,
    object_id: &str,
) -> Option<GridCoordinate> {
    let database_object = database.by_id(object_id)?;
    database_object.default_button_position()
}

impl UnitCommandSlots for WarcraftDatabase {
    fn command_card(&self, unit_id: WarcraftObjectId) -> CommandCard {
        let unit_id_str = unit_id.value();
        let Some(unit_object) = self.by_id(unit_id_str) else {
            return CommandCard::empty();
        };
        let WarcraftObjectMeta::Unit(unit_meta) = unit_object.meta() else {
            return CommandCard::empty();
        };
        let unit_race = unit_object.race();
        let primary_commands =
            CommandCatalog::primary_commands_for(unit_meta, unit_race, unit_id_str);
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

        let mut card = CommandCard::empty();

        for command_name in primary_commands {
            let command_object = self.by_id(command_name);
            let command_has_icon =
                command_object.is_some_and(|object| object.has_displayable_icon());
            if !command_has_icon {
                continue;
            }
            let Some(slot_position) = slot_position_from_database(self, command_name) else {
                continue;
            };
            let command_slot = GridSlotId::command(command_name);
            card.place(slot_position, command_slot);
        }

        for trained_id in primary_train_slots {
            let trained_str = trained_id.value();
            let trained_object = self.by_id(trained_str);
            let trained_has_icon =
                trained_object.is_some_and(|object| object.has_displayable_icon());
            if !trained_has_icon {
                continue;
            }
            let Some(slot_position) = slot_position_from_database(self, trained_str) else {
                continue;
            };
            let train_slot = GridSlotId::ability(trained_str);
            card.place(slot_position, train_slot);
        }

        for research_id in primary_research_slots {
            let research_str = research_id.value();
            let research_object = self.by_id(research_str);
            let research_has_icon =
                research_object.is_some_and(|object| object.has_displayable_icon());
            if !research_has_icon {
                continue;
            }
            let Some(slot_position) = slot_position_from_database(self, research_str) else {
                continue;
            };
            let research_slot = GridSlotId::ability(research_str);
            card.place(slot_position, research_slot);
        }

        let mut unplaced_sell_item_slots: Vec<GridSlotId> = Vec::new();
        for sell_item_id in sell_items {
            let sell_item_str = sell_item_id.value();
            let sell_item_object = self.by_id(sell_item_str);
            let sell_item_has_icon =
                sell_item_object.is_some_and(|object| object.has_displayable_icon());
            if !sell_item_has_icon {
                continue;
            }
            let sell_item_slot = GridSlotId::ability(sell_item_str);
            match slot_position_from_database(self, sell_item_str) {
                Some(sell_item_position) => {
                    if !card.place(sell_item_position, sell_item_slot) {
                        unplaced_sell_item_slots.push(sell_item_slot);
                    }
                }
                None => {
                    unplaced_sell_item_slots.push(sell_item_slot);
                }
            }
        }
        for unplaced_slot in unplaced_sell_item_slots {
            card.place_at_next_empty(unplaced_slot);
        }

        let mut unplaced_sell_unit_slots: Vec<GridSlotId> = Vec::new();
        for sell_unit_id in sell_units {
            let sell_unit_str = sell_unit_id.value();
            let sell_unit_object = self.by_id(sell_unit_str);
            let sell_unit_has_icon =
                sell_unit_object.is_some_and(|object| object.has_displayable_icon());
            if !sell_unit_has_icon {
                continue;
            }
            let sell_unit_slot = GridSlotId::ability(sell_unit_str);
            match slot_position_from_database(self, sell_unit_str) {
                Some(sell_unit_position) => {
                    if !card.place(sell_unit_position, sell_unit_slot) {
                        unplaced_sell_unit_slots.push(sell_unit_slot);
                    }
                }
                None => {
                    unplaced_sell_unit_slots.push(sell_unit_slot);
                }
            }
        }
        for unplaced_slot in unplaced_sell_unit_slots {
            card.place_at_next_empty(unplaced_slot);
        }

        let is_uprootable = BuildingTraits::can_uproot(unit_id_str);
        let host_is_burrowed = BuildingTraits::is_burrowed_form(unit_id_str);
        let host_is_in_alt_state = BuildingTraits::unit_starts_in_toggle_alt_state(unit_id_str);

        let mut unplaced_ability_slots: Vec<GridSlotId> = Vec::new();
        for ability_id in regular_abilities.iter().chain(hero_abilities.iter()) {
            let ability_str = ability_id.value();
            if hero_abilities.contains(ability_id) {
                let levelable_object = self.by_id(ability_str);
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
            if is_uprootable && ability_str.eq_ignore_ascii_case("Aeat") {
                continue;
            }
            if host_is_burrowed && !BuildingTraits::ability_has_alt_state(ability_str) {
                continue;
            }
            if ability_reverts_to_host(self, ability_str, unit_id_str) {
                continue;
            }
            let ability_database_object = self.by_id(ability_str);
            let ability_has_icon =
                ability_database_object.is_some_and(|object| object.has_displayable_icon());
            if !ability_has_icon {
                continue;
            }
            let morph_target_object = self.by_id(ability_str);
            let morph_target_id =
                morph_target_object.and_then(|object| object.ability_morph_target_id());
            let is_morph_back =
                morph_target_id.is_some_and(|target| target.eq_ignore_ascii_case(unit_id_str));
            let use_off_state = is_morph_back
                || (host_is_in_alt_state && BuildingTraits::ability_has_alt_state(ability_str));
            let ability_slot = if use_off_state {
                GridSlotId::ability_off(ability_str)
            } else {
                GridSlotId::ability(ability_str)
            };
            match slot_position_from_database(self, ability_str) {
                Some(ability_position) => {
                    if !card.place(ability_position, ability_slot) {
                        unplaced_ability_slots.push(ability_slot);
                    }
                }
                None => {
                    unplaced_ability_slots.push(ability_slot);
                }
            }
        }
        for unplaced_slot in unplaced_ability_slots {
            card.place_at_next_empty(unplaced_slot);
        }

        if unit_kind == UnitKind::Hero
            && !hero_abilities.is_empty()
            && let Some(select_skill) = CommandCatalog::known_command("CmdSelectSkill")
        {
            let select_skill_object = self.by_id(select_skill);
            let select_skill_has_icon =
                select_skill_object.is_some_and(|object| object.has_displayable_icon());
            if select_skill_has_icon {
                let position_option = slot_position_from_database(self, select_skill);
                if let Some(slot_position) = position_option {
                    let select_skill_slot = GridSlotId::command(select_skill);
                    card.place(slot_position, select_skill_slot);
                }
            }
        }

        card
    }

    fn build_menu(&self, unit_id: WarcraftObjectId) -> Option<CommandCard> {
        let unit_id_str = unit_id.value();
        let unit_object = self.by_id(unit_id_str)?;
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
        let mut card = CommandCard::empty();
        for command_name in build_menu_commands {
            let command_object = self.by_id(command_name);
            let command_has_icon =
                command_object.is_some_and(|object| object.has_displayable_icon());
            if !command_has_icon {
                continue;
            }
            let Some(slot_position) = slot_position_from_database(self, command_name) else {
                continue;
            };
            let command_slot = GridSlotId::command(command_name);
            card.place(slot_position, command_slot);
        }
        for production_id in unit_meta.builds() {
            let production_str = production_id.value();
            let production_object = self.by_id(production_str);
            let production_has_icon =
                production_object.is_some_and(|object| object.has_displayable_icon());
            if !production_has_icon {
                continue;
            }
            let Some(slot_position) = slot_position_from_database(self, production_str) else {
                continue;
            };
            let production_slot = GridSlotId::ability(production_str);
            card.place(slot_position, production_slot);
        }
        Some(card)
    }

    fn research_menu(&self, unit_id: WarcraftObjectId) -> Option<CommandCard> {
        let unit_id_str = unit_id.value();
        let unit_object = self.by_id(unit_id_str)?;
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
        let mut card = CommandCard::empty();
        for ability_id in hero_abilities.iter() {
            let ability_str = ability_id.value();
            let ability_object = self.by_id(ability_str);
            let ability_has_icon =
                ability_object.is_some_and(|object| object.has_displayable_icon());
            if !ability_has_icon {
                continue;
            }
            let Some(slot_position) = slot_position_from_database(self, ability_str) else {
                continue;
            };
            let ability_slot = GridSlotId::ability(ability_str);
            card.place(slot_position, ability_slot);
        }
        if let Some(back_command) = CommandCatalog::submenu_back_command() {
            let back_command_object = self.by_id(back_command);
            let back_command_has_icon =
                back_command_object.is_some_and(|object| object.has_displayable_icon());
            if back_command_has_icon {
                let position_option = slot_position_from_database(self, back_command);
                if let Some(slot_position) = position_option {
                    let back_slot = GridSlotId::command(back_command);
                    card.place(slot_position, back_slot);
                }
            }
        }
        Some(card)
    }

    fn uprooted_menu(&self, unit_id: WarcraftObjectId) -> Option<CommandCard> {
        let unit_id_str = unit_id.value();
        let unit_object = self.by_id(unit_id_str)?;
        let WarcraftObjectMeta::Unit(unit_meta) = unit_object.meta() else {
            return None;
        };
        if CommandCatalog::effective_kind(unit_meta) != UnitKind::Building {
            return None;
        }
        if !BuildingTraits::can_uproot(unit_id_str) {
            return None;
        }
        let mut card = CommandCard::empty();
        for command_name in CommandCatalog::mobile_command_ids().iter().copied() {
            let command_object = self.by_id(command_name);
            let command_has_icon =
                command_object.is_some_and(|object| object.has_displayable_icon());
            if !command_has_icon {
                continue;
            }
            let Some(slot_position) = slot_position_from_database(self, command_name) else {
                continue;
            };
            let command_slot = GridSlotId::command(command_name);
            card.place(slot_position, command_slot);
        }
        for ability_id in unit_meta.abilities() {
            let ability_str = ability_id.value();
            let ability_object = self.by_id(ability_str);
            let ability_has_icon =
                ability_object.is_some_and(|object| object.has_displayable_icon());
            if !ability_has_icon {
                continue;
            }
            if ability_reverts_to_host(self, ability_str, unit_id_str) {
                continue;
            }
            if ability_requires_rooted_form(self, ability_str) {
                continue;
            }
            let Some(slot_position) = slot_position_from_database(self, ability_str) else {
                continue;
            };
            let ability_slot = GridSlotId::ability(ability_str);
            card.place(slot_position, ability_slot);
        }
        Some(card)
    }

    fn train_unit_upgrades(
        &self,
        unit_id: WarcraftObjectId,
    ) -> HashMap<WarcraftObjectId, WarcraftObjectId> {
        let unit_id_str = unit_id.value();
        let Some(unit_object) = self.by_id(unit_id_str) else {
            return HashMap::new();
        };
        let WarcraftObjectMeta::Unit(unit_meta) = unit_object.meta() else {
            return HashMap::new();
        };
        let primary_train_slots = unit_meta.trains();
        let mut seen_positions: HashMap<crate::GridCoordinate, WarcraftObjectId> = HashMap::new();
        let mut upgrades: HashMap<WarcraftObjectId, WarcraftObjectId> = HashMap::new();
        for trained_id in primary_train_slots {
            let trained_str = trained_id.value();
            let trained_object = self.by_id(trained_str);
            let has_icon = trained_object.is_some_and(|object| object.has_displayable_icon());
            if !has_icon {
                continue;
            }
            let position_option =
                trained_object.and_then(|object| object.default_button_position());
            let Some(position) = position_option else {
                continue;
            };
            if let Some(existing_id) = seen_positions.get(&position).copied() {
                upgrades.entry(existing_id).or_insert(*trained_id);
            } else {
                seen_positions.insert(position, *trained_id);
            }
        }
        upgrades
    }

    fn all_unit_ids(&self) -> impl Iterator<Item = WarcraftObjectId> {
        self.iter().filter_map(|(database_id, warcraft_object)| {
            if matches!(warcraft_object.meta(), WarcraftObjectMeta::Unit(_)) {
                Some(*database_id)
            } else {
                None
            }
        })
    }
}

#[cfg(test)]
mod unit_slots_tests {
    use super::*;
    use warcraft_database::WARCRAFT_DATABASE;

    #[test]
    fn command_card_for_unknown_unit_is_empty() {
        let unit_id = WarcraftObjectId::new("ZZZUnknown");
        let card = WARCRAFT_DATABASE.command_card(unit_id);
        assert!(card.is_empty());
    }

    #[test]
    fn command_card_for_peasant_is_non_empty() {
        let unit_id = WarcraftObjectId::new("hpea");
        let card = WARCRAFT_DATABASE.command_card(unit_id);
        assert!(!card.is_empty());
    }

    #[test]
    fn command_card_for_peasant_contains_attack() {
        let unit_id = WarcraftObjectId::new("hpea");
        let card = WARCRAFT_DATABASE.command_card(unit_id);
        let has_attack = card
            .filled_slots()
            .any(|slot| slot.id().value().eq_ignore_ascii_case("CmdAttack"));
        assert!(has_attack, "peasant command card must contain CmdAttack");
    }

    #[test]
    fn command_card_for_paladin_is_non_empty() {
        let unit_id = WarcraftObjectId::new("Hpal");
        let card = WARCRAFT_DATABASE.command_card(unit_id);
        assert!(!card.is_empty());
    }

    #[test]
    fn command_card_for_paladin_contains_hero_abilities() {
        let unit_id = WarcraftObjectId::new("Hpal");
        let card = WARCRAFT_DATABASE.command_card(unit_id);
        let ability_count = card
            .filled_slots()
            .filter(|slot| matches!(slot, GridSlotId::Ability(_)))
            .count();
        assert!(
            ability_count > 0,
            "paladin must have at least one ability slot"
        );
    }

    #[test]
    fn build_menu_for_non_worker_returns_none() {
        let unit_id = WarcraftObjectId::new("Hpal");
        let result = WARCRAFT_DATABASE.build_menu(unit_id);
        assert!(result.is_none());
    }

    #[test]
    fn build_menu_for_peasant_returns_some() {
        let unit_id = WarcraftObjectId::new("hpea");
        let result = WARCRAFT_DATABASE.build_menu(unit_id);
        assert!(result.is_some());
    }

    #[test]
    fn build_menu_for_peasant_is_non_empty() {
        let unit_id = WarcraftObjectId::new("hpea");
        let card = WARCRAFT_DATABASE.build_menu(unit_id).unwrap();
        assert!(!card.is_empty());
    }

    #[test]
    fn research_menu_for_non_hero_returns_none() {
        let unit_id = WarcraftObjectId::new("hpea");
        let result = WARCRAFT_DATABASE.research_menu(unit_id);
        assert!(result.is_none());
    }

    #[test]
    fn research_menu_for_paladin_returns_some() {
        let unit_id = WarcraftObjectId::new("Hpal");
        let result = WARCRAFT_DATABASE.research_menu(unit_id);
        assert!(result.is_some());
    }

    #[test]
    fn uprooted_menu_for_non_uprootable_building_returns_none() {
        let unit_id = WarcraftObjectId::new("htow");
        let result = WARCRAFT_DATABASE.uprooted_menu(unit_id);
        assert!(result.is_none());
    }

    #[test]
    fn uprooted_menu_for_tree_of_life_returns_some() {
        let unit_id = WarcraftObjectId::new("etol");
        let result = WARCRAFT_DATABASE.uprooted_menu(unit_id);
        assert!(result.is_some());
    }

    #[test]
    fn uprooted_menu_for_tree_of_life_contains_movement_commands() {
        let unit_id = WarcraftObjectId::new("etol");
        let card = WARCRAFT_DATABASE.uprooted_menu(unit_id).unwrap();
        let has_move = card
            .filled_slots()
            .any(|slot| slot.id().value().eq_ignore_ascii_case("CmdMove"));
        assert!(has_move, "uprooted Tree of Life must have CmdMove");
    }

    #[test]
    fn all_unit_ids_is_non_empty() {
        let count = WARCRAFT_DATABASE.all_unit_ids().count();
        assert!(count > 0);
    }

    #[test]
    fn all_unit_ids_contains_peasant() {
        let has_peasant = WARCRAFT_DATABASE
            .all_unit_ids()
            .any(|id| id.value().eq_ignore_ascii_case("hpea"));
        assert!(has_peasant);
    }

    #[test]
    fn goblin_lab_command_card_shows_all_three_sell_units() {
        let unit_id = WarcraftObjectId::new("ngad");
        let card = WARCRAFT_DATABASE.command_card(unit_id);
        let has_sapper = card
            .filled_slots()
            .any(|slot| slot.id().value().eq_ignore_ascii_case("ngsp"));
        let has_zeppelin = card
            .filled_slots()
            .any(|slot| slot.id().value().eq_ignore_ascii_case("nzep"));
        let has_shredder = card
            .filled_slots()
            .any(|slot| slot.id().value().eq_ignore_ascii_case("ngir"));
        assert!(
            has_sapper,
            "Goblin Lab command card must contain Goblin Sapper (ngsp)"
        );
        assert!(
            has_zeppelin,
            "Goblin Lab command card must contain Goblin Zeppelin (nzep)"
        );
        assert!(
            has_shredder,
            "Goblin Lab command card must contain Goblin Shredder (ngir)"
        );
    }

    #[test]
    fn goblin_merchant_command_card_shows_all_eleven_sell_items() {
        let unit_id = WarcraftObjectId::new("ngme");
        let card = WARCRAFT_DATABASE.command_card(unit_id);
        let sell_item_ids = [
            "stwp", "bspd", "dust", "tret", "prvt", "cnob", "stel", "pnvl", "shea", "spro", "pinv",
        ];
        for sell_item_id in sell_item_ids {
            let present = card
                .filled_slots()
                .any(|slot| slot.id().value().eq_ignore_ascii_case(sell_item_id));
            assert!(
                present,
                "Goblin Merchant command card must contain sell item {sell_item_id}"
            );
        }
    }

    #[test]
    fn gargoyle_command_card_contains_prioritize() {
        let unit_id = WarcraftObjectId::new("ugar");
        let card = WARCRAFT_DATABASE.command_card(unit_id);
        let has_prioritize = card
            .filled_slots()
            .any(|slot| slot.id().value().eq_ignore_ascii_case("Aatp"));
        assert!(
            has_prioritize,
            "Gargoyle command card must contain Prioritize (Aatp)"
        );
    }
}
