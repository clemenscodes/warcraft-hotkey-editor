use std::collections::HashMap;
use std::rc::Rc;

use warcraft_api::{GridCoordinate, UnitKind, WarcraftObjectId, WarcraftObjectMeta};
use warcraft_database::{BuildingTraits, CommandCatalog};
use warcraft_database::{ObjectLookup, UnitKindHelpers};

use crate::grid_slot::GridSlotId;

#[derive(Clone, PartialEq)]
pub(crate) struct UnitSlotData {
    command_card_slots: Rc<[GridSlotId]>,
    build_menu_slots: Option<Rc<[GridSlotId]>>,
    uprooted_menu_slots: Option<Rc<[GridSlotId]>>,
    research_menu_slots: Option<Rc<[GridSlotId]>>,
    train_unit_upgrades: HashMap<&'static str, &'static str>,
}

impl UnitSlotData {
    pub(crate) fn empty() -> Self {
        Self {
            command_card_slots: Rc::from(Vec::<GridSlotId>::new()),
            build_menu_slots: None,
            uprooted_menu_slots: None,
            research_menu_slots: None,
            train_unit_upgrades: HashMap::new(),
        }
    }

    pub(crate) fn command_card_slots(&self) -> &Rc<[GridSlotId]> {
        &self.command_card_slots
    }

    pub(crate) fn build_menu_slots(&self) -> Option<&Rc<[GridSlotId]>> {
        self.build_menu_slots.as_ref()
    }

    pub(crate) fn uprooted_menu_slots(&self) -> Option<&Rc<[GridSlotId]>> {
        self.uprooted_menu_slots.as_ref()
    }

    pub(crate) fn research_menu_slots(&self) -> Option<&Rc<[GridSlotId]>> {
        self.research_menu_slots.as_ref()
    }

    pub(crate) fn train_unit_upgrades(&self) -> &HashMap<&'static str, &'static str> {
        &self.train_unit_upgrades
    }

    pub(crate) fn compute(unit_id: &str) -> Self {
        let Some(unit_object) = ObjectLookup::by_id(unit_id) else {
            return Self::empty();
        };
        let WarcraftObjectMeta::Unit(unit_meta) = unit_object.meta() else {
            return Self::empty();
        };

        let regular_abilities = unit_meta.abilities();
        let hero_abilities = unit_meta.hero_abilities();
        let primary_commands =
            CommandCatalog::primary_commands_for(unit_meta, unit_object.race(), unit_id);
        let unit_kind = UnitKindHelpers::effective_kind(unit_meta);

        let primary_train_slots: &[WarcraftObjectId] = if unit_kind == UnitKind::Building {
            unit_meta.trains()
        } else {
            &[]
        };
        let primary_research_slots: &[WarcraftObjectId] = if unit_kind == UnitKind::Building {
            unit_meta.researches()
        } else {
            &[]
        };
        let sell_items: &[WarcraftObjectId] = if unit_kind == UnitKind::Building {
            unit_meta.sell_items()
        } else {
            &[]
        };
        let sell_units: &[WarcraftObjectId] = if unit_kind == UnitKind::Building {
            unit_meta.sell_units()
        } else {
            &[]
        };

        let mut command_card_slots: Vec<GridSlotId> = Vec::with_capacity(
            primary_train_slots.len()
                + primary_research_slots.len()
                + regular_abilities.len()
                + primary_commands.len()
                + sell_items.len()
                + sell_units.len(),
        );
        for command_name in primary_commands {
            if !ObjectLookup::has_icon(command_name) {
                continue;
            }
            command_card_slots.push(GridSlotId::command(command_name));
        }
        let mut seen_train_positions: HashMap<GridCoordinate, &'static str> = HashMap::new();
        let mut train_unit_upgrades: HashMap<&'static str, &'static str> = HashMap::new();
        for trained_id in primary_train_slots {
            let id_str = trained_id.value();
            if !ObjectLookup::has_icon(id_str) {
                continue;
            }
            let default_position =
                ObjectLookup::by_id(id_str).and_then(|object| object.default_button_position());
            if let Some(button_position) = default_position {
                if let Some(existing_id) = seen_train_positions.get(&button_position) {
                    if !train_unit_upgrades.contains_key(existing_id) {
                        train_unit_upgrades.insert(existing_id, id_str);
                    }
                    continue;
                }
                seen_train_positions.insert(button_position, id_str);
            }
            command_card_slots.push(GridSlotId::ability(id_str));
        }
        for research_id in primary_research_slots {
            if !ObjectLookup::has_icon(research_id.value()) {
                continue;
            }
            command_card_slots.push(GridSlotId::ability(research_id.value()));
        }
        for sell_item_id in sell_items {
            if !ObjectLookup::has_icon(sell_item_id.value()) {
                continue;
            }
            command_card_slots.push(GridSlotId::ability(sell_item_id.value()));
        }
        for sell_unit_id in sell_units {
            if !ObjectLookup::has_icon(sell_unit_id.value()) {
                continue;
            }
            command_card_slots.push(GridSlotId::ability(sell_unit_id.value()));
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
            if morphs_into_self(ability_id.value(), unit_id) {
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
                command_card_slots.push(GridSlotId::ability_off(ability_id.value()));
            } else {
                command_card_slots.push(GridSlotId::ability(ability_id.value()));
            }
        }
        if unit_kind == UnitKind::Hero
            && !hero_abilities.is_empty()
            && let Some(select_skill_command) = CommandCatalog::known_command("CmdSelectSkill")
            && ObjectLookup::has_icon(select_skill_command)
        {
            command_card_slots.push(GridSlotId::command(select_skill_command));
        }
        let command_card_slots: Rc<[GridSlotId]> = command_card_slots.into();

        let build_menu_slots: Option<Rc<[GridSlotId]>> =
            if unit_kind == UnitKind::Worker && !unit_meta.builds().is_empty() {
                let unit_builds = unit_meta.builds();
                let build_menu_commands = CommandCatalog::build_menu_commands_for(unit_meta);
                let mut build_menu_slots: Vec<GridSlotId> =
                    Vec::with_capacity(unit_builds.len() + build_menu_commands.len());
                for command_name in build_menu_commands {
                    if !ObjectLookup::has_icon(command_name) {
                        continue;
                    }
                    build_menu_slots.push(GridSlotId::command(command_name));
                }
                for production_id in unit_builds {
                    if !ObjectLookup::has_icon(production_id.value()) {
                        continue;
                    }
                    build_menu_slots.push(GridSlotId::ability(production_id.value()));
                }
                Some(build_menu_slots.into())
            } else {
                None
            };

        let uprooted_menu_slots: Option<Rc<[GridSlotId]>> =
            if unit_kind == UnitKind::Building && BuildingTraits::can_uproot(unit_id) {
                let mut uprooted_slots: Vec<GridSlotId> = Vec::new();
                for command_name in CommandCatalog::mobile_command_ids().iter().copied() {
                    if !ObjectLookup::has_icon(command_name) {
                        continue;
                    }
                    uprooted_slots.push(GridSlotId::command(command_name));
                }
                for ability_id in regular_abilities {
                    if !ObjectLookup::has_icon(ability_id.value()) {
                        continue;
                    }
                    if morphs_into_self(ability_id.value(), unit_id) {
                        continue;
                    }
                    if is_rooted_only_mechanic(ability_id.value()) {
                        continue;
                    }
                    uprooted_slots.push(GridSlotId::ability(ability_id.value()));
                }
                Some(uprooted_slots.into())
            } else {
                None
            };

        let research_menu_slots: Option<Rc<[GridSlotId]>> =
            if unit_kind == UnitKind::Hero && !hero_abilities.is_empty() {
                let mut research_menu_slots: Vec<GridSlotId> =
                    Vec::with_capacity(hero_abilities.len() + 1);
                for ability_id in hero_abilities.iter() {
                    if !ObjectLookup::has_icon(ability_id.value()) {
                        continue;
                    }
                    research_menu_slots.push(GridSlotId::ability(ability_id.value()));
                }
                if let Some(back_command) = CommandCatalog::submenu_back_command()
                    && ObjectLookup::has_icon(back_command)
                {
                    research_menu_slots.push(GridSlotId::command(back_command));
                }
                Some(research_menu_slots.into())
            } else {
                None
            };

        Self {
            command_card_slots,
            build_menu_slots,
            uprooted_menu_slots,
            research_menu_slots,
            train_unit_upgrades,
        }
    }
}

/// Game-mechanic codes that only make sense on a stationary / rooted form.
/// When an Ancient uproots and becomes mobile, the shop UI vanishes in-game
/// — so we suppress these abilities from the editor's uprooted command card.
/// Sourced from `units/abilitydata.slk`'s `code` column via the extractor.
///
/// - `Apit` — Purchase Item (the shop's buy button).
/// - `Aall` — Allied Building (the mechanic that flags a shop as
///   purchasable by allied players in team games; appears as
///   "Select Hero On" / "Pick Shop Buyer" in the unit list).
const ROOTED_ONLY_ABILITY_CODES: &[&str] = &["Apit", "Aall"];

/// Ability aliases that have no `code` entry in abilitydata.slk but are still
/// rooted-only mechanics and must be suppressed from the uprooted panel.
/// `Anei` (Select User / Neutral Interact) is added implicitly by the game
/// engine to shops and disappears when the building uproots.
const ROOTED_ONLY_ABILITY_IDS: &[&str] = &["Anei"];

fn morphs_into_self(ability_id: &str, host_unit_id: &str) -> bool {
    let Some(target_id) = ObjectLookup::morph_target_unit(ability_id) else {
        return false;
    };
    if !target_id.eq_ignore_ascii_case(host_unit_id) {
        return false;
    }
    // Self-morph + alt-state means a two-way toggle living on the same unit
    // (Burrow ⇄ Unburrow on `ucrm`/`ucsB`/`ucsC`/`nbnb`, Defend ⇄ Stop Defend
    // on `Adef`). The on-state's `morph_target` legitimately points at the
    // host unit, so suppressing the ability here would hide the entire toggle
    // from the burrowed/active form. Keep it.
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
        .any(|rooted_only_code| rooted_only_code.eq_ignore_ascii_case(ability_code))
}
