use std::collections::HashMap;

use warcraft_api::{UnitKind, WarcraftObjectMeta};
use warcraft_database::WARCRAFT_DATABASE;

use crate::domain::building_traits::BuildingTraits;
use crate::domain::command_catalog::CommandCatalog;
use crate::domain::grid_slot::GridSlotId;
use crate::domain::object_lookup::ObjectLookup;
use crate::domain::unit_kind::UnitKindHelpers;

const ROOTED_ONLY_ABILITY_CODES: &[&str] = &["Apit", "Aall"];
const ROOTED_ONLY_ABILITY_IDS: &[&str] = &["Anei"];

pub(crate) struct UnitSlots;

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

    pub(crate) fn all_unit_ids() -> impl Iterator<Item = &'static str> {
        WARCRAFT_DATABASE.iter().filter_map(|(id, obj)| {
            if matches!(obj.meta(), WarcraftObjectMeta::Unit(_)) {
                Some(id.value())
            } else {
                None
            }
        })
    }

    pub(crate) fn command_card_for(unit_id: &str) -> Vec<GridSlotId> {
        let Some(unit_object) = ObjectLookup::by_id(unit_id) else {
            return Vec::new();
        };
        let WarcraftObjectMeta::Unit(unit_meta) = unit_object.meta() else {
            return Vec::new();
        };
        let primary_commands =
            CommandCatalog::primary_commands_for(unit_meta, unit_object.race(), unit_id);
        let unit_kind = UnitKindHelpers::effective_kind(unit_meta);
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

        // Deduplicate train slots that share a default button position — the
        // second entry at the same position is an upgrade of the first and must
        // not be included as an independent slot (same logic as unit_detail.rs).
        let mut seen_train_positions: HashMap<(u8, u8), ()> = HashMap::new();
        for trained_id in primary_train_slots {
            let id_str = trained_id.value();
            if !ObjectLookup::has_icon(id_str) {
                continue;
            }
            let default_pos = ObjectLookup::by_id(id_str)
                .and_then(|obj| obj.default_button_position())
                .map(|p| (p.column(), p.row()));
            if let Some(pos) = default_pos {
                if seen_train_positions.contains_key(&pos) {
                    continue;
                }
                seen_train_positions.insert(pos, ());
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
                    .map(|o| match o.meta() {
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
            if ObjectLookup::ability_belongs_to_alt_form(ability_id.value(), unit_id) {
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

    pub(crate) fn build_menu_for(unit_id: &str) -> Option<Vec<GridSlotId>> {
        let unit_object = ObjectLookup::by_id(unit_id)?;
        let WarcraftObjectMeta::Unit(unit_meta) = unit_object.meta() else {
            return None;
        };
        if UnitKindHelpers::effective_kind(unit_meta) != UnitKind::Worker {
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

    pub(crate) fn uprooted_menu_for(unit_id: &str) -> Option<Vec<GridSlotId>> {
        let unit_object = ObjectLookup::by_id(unit_id)?;
        let WarcraftObjectMeta::Unit(unit_meta) = unit_object.meta() else {
            return None;
        };
        if UnitKindHelpers::effective_kind(unit_meta) != UnitKind::Building {
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
            if ObjectLookup::ability_belongs_to_alt_form(ability_id.value(), unit_id) {
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
