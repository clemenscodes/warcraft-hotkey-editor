use crate::GridCoordinate;
use crate::identity::slot::{CommandCard, GridSlotId};
use crate::unit::ability_rules::{
    AbilityInDatabase, AbilityOnUnit, FormUpgradeSwap, HiddenAbility, MorphAgainstHost,
    RevertsToHost, RootedOnlyAbility, UnitPair,
};
use ddd::Specification;
use std::collections::HashMap;
use warcraft_api::{UnitKind, WarcraftDatabase, WarcraftObjectId, WarcraftObjectMeta};
use warcraft_database::{BuildingTraits, CommandCatalog};

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

fn slot_position_from_database(
    database: &WarcraftDatabase,
    object_id: &str,
) -> Option<GridCoordinate> {
    let database_object = database.by_id(object_id)?;
    database_object.default_button_position()
}

fn research_slot_position_from_database(
    database: &WarcraftDatabase,
    object_id: &str,
) -> Option<GridCoordinate> {
    let database_object = database.by_id(object_id)?;
    database_object.default_research_button_position()
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
        let mut unplaced_train_slots: Vec<GridSlotId> = Vec::new();
        for trained_id in primary_train_slots {
            let trained_str = trained_id.value();
            let trained_object = self.by_id(trained_str);
            let trained_has_icon =
                trained_object.is_some_and(|object| object.has_displayable_icon());
            if !trained_has_icon {
                continue;
            }
            let train_slot = GridSlotId::ability(trained_str);
            match slot_position_from_database(self, trained_str) {
                Some(slot_position) => {
                    if !card.place(slot_position, train_slot) {
                        let occupant = card.slot_at(slot_position);
                        let collapses_into_swap = occupant.is_some_and(|existing| {
                            let existing_id = existing.id();
                            let existing_str = existing_id.value();
                            let pair = UnitPair::new(trained_str, existing_str);
                            FormUpgradeSwap.is_satisfied_by(&pair)
                        });
                        if !collapses_into_swap {
                            unplaced_train_slots.push(train_slot);
                        }
                    }
                }
                None => {
                    unplaced_train_slots.push(train_slot);
                }
            }
        }
        for unplaced_slot in unplaced_train_slots {
            card.place_at_next_empty(unplaced_slot);
        }
        let mut unplaced_research_slots: Vec<GridSlotId> = Vec::new();
        for research_id in primary_research_slots {
            let research_str = research_id.value();
            let research_object = self.by_id(research_str);
            let research_has_icon =
                research_object.is_some_and(|object| object.has_displayable_icon());
            if !research_has_icon {
                continue;
            }
            let research_slot = GridSlotId::ability(research_str);
            match slot_position_from_database(self, research_str) {
                Some(slot_position) => {
                    if !card.place(slot_position, research_slot) {
                        unplaced_research_slots.push(research_slot);
                    }
                }
                None => {
                    unplaced_research_slots.push(research_slot);
                }
            }
        }
        for unplaced_slot in unplaced_research_slots {
            card.place_at_next_empty(unplaced_slot);
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
        let mut occupied_on_positions: Vec<GridCoordinate> = Vec::new();
        for ability_id in regular_abilities.iter().chain(hero_abilities.iter()) {
            if let Some(on_position) = slot_position_from_database(self, ability_id.value()) {
                occupied_on_positions.push(on_position);
            }
        }
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
            let hidden_candidate = AbilityOnUnit::new(unit_id_str, ability_str);
            if HiddenAbility.is_satisfied_by(&hidden_candidate) {
                continue;
            }
            if is_uprootable && ability_str.eq_ignore_ascii_case("Aeat") {
                continue;
            }
            if host_is_burrowed && !BuildingTraits::ability_has_alt_state(ability_str) {
                continue;
            }
            let morph_candidate = MorphAgainstHost::new(self, ability_str, unit_id_str);
            if RevertsToHost.is_satisfied_by(&morph_candidate) {
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
            let off_state_object = self.by_id(ability_str);
            let off_state_position = off_state_object.and_then(|object| match object.meta() {
                WarcraftObjectMeta::Ability(ability_meta) => ability_meta.off_button_position(),
                _ => None,
            });
            match slot_position_from_database(self, ability_str) {
                Some(ability_position) => {
                    if !card.place(ability_position, ability_slot) {
                        unplaced_ability_slots.push(ability_slot);
                    }
                    if !use_off_state
                        && unit_kind == UnitKind::Building
                        && let Some(off_position) = off_state_position
                        && off_position != ability_position
                        && !occupied_on_positions.contains(&off_position)
                    {
                        let off_state_slot = GridSlotId::ability_off(ability_str);
                        if !card.place(off_position, off_state_slot) {
                            unplaced_ability_slots.push(off_state_slot);
                        }
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
            let Some(slot_position) = research_slot_position_from_database(self, ability_str)
            else {
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
            let morph_candidate = MorphAgainstHost::new(self, ability_str, unit_id_str);
            if RevertsToHost.is_satisfied_by(&morph_candidate) {
                continue;
            }
            let rooted_candidate = AbilityInDatabase::new(self, ability_str);
            if RootedOnlyAbility.is_satisfied_by(&rooted_candidate) {
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
                let existing_str = existing_id.value();
                let pair = UnitPair::new(existing_str, trained_str);
                if FormUpgradeSwap.is_satisfied_by(&pair) {
                    upgrades.entry(existing_id).or_insert(*trained_id);
                }
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
mod tests;
