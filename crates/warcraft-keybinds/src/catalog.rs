use std::sync::LazyLock;

use warcraft_api::{Race, UnitKind, UnitMeta, WarcraftObjectKind, WarcraftObjectMeta};
use warcraft_database::WARCRAFT_DATABASE;

use crate::slot::GridSlotId;

pub struct BuildingTraits;

impl BuildingTraits {
    pub fn can_attack(object_id: &str) -> bool {
        let lowercase_id = object_id.to_ascii_lowercase();
        matches!(
            lowercase_id.as_str(),
            "hgtw"
                | "hatw"
                | "hctw"
                | "owtw"
                | "otrb"
                | "unp1"
                | "unp2"
                | "uzg1"
                | "uzg2"
                | "nadt"
                | "ndgt"
                | "ntt1"
        )
    }

    pub fn can_uproot(object_id: &str) -> bool {
        let lowercase_id = object_id.to_ascii_lowercase();
        matches!(
            lowercase_id.as_str(),
            "etol" | "etoa" | "etoe" | "eaow" | "eaoe" | "eaom" | "etrp" | "eden"
        )
    }

    pub fn unit_starts_in_toggle_alt_state(unit_id: &str) -> bool {
        if Self::can_uproot(unit_id) {
            return true;
        }
        if Self::is_burrowed_form(unit_id) {
            return true;
        }
        unit_id.eq_ignore_ascii_case("hmil")
    }

    pub fn ability_is_on_alt_state_unit(ability_id: &str) -> bool {
        for (unit_id_obj, warcraft_object) in WARCRAFT_DATABASE.iter() {
            let unit_id = unit_id_obj.value();
            if !Self::unit_starts_in_toggle_alt_state(unit_id) {
                continue;
            }
            let WarcraftObjectMeta::Unit(unit_meta) = warcraft_object.meta() else {
                continue;
            };
            let has_ability = unit_meta.abilities().iter().any(|ability_id_obj| {
                let ability_object_id = ability_id_obj.value();
                ability_object_id.eq_ignore_ascii_case(ability_id)
            });
            if has_ability {
                return true;
            }
        }
        false
    }

    pub fn is_burrowed_form(unit_id: &str) -> bool {
        let Some(warcraft_object) = WARCRAFT_DATABASE.by_id(unit_id) else {
            return false;
        };
        let names = warcraft_object.names();
        let Some(first_name) = names.first().copied() else {
            return false;
        };
        let lowercase_name = first_name.to_ascii_lowercase();
        lowercase_name.starts_with("burrowed ")
    }

    pub fn ability_has_alt_state(ability_id: &str) -> bool {
        let Some(warcraft_object) = WARCRAFT_DATABASE.by_id(ability_id) else {
            return false;
        };
        warcraft_object.un_tip().is_some() || warcraft_object.un_ubertip().is_some()
    }
}

const CONTEXT_COMMAND_IDS: &[&str] = &[
    "CmdCancel",
    "CmdCancelBuild",
    "CmdCancelRevive",
    "CmdCancelTrain",
];

pub struct CommandCatalog;

impl CommandCatalog {
    pub(crate) fn effective_kind(unit_meta: &UnitMeta) -> UnitKind {
        if unit_meta.is_special() && unit_meta.unit_kind() == UnitKind::Worker {
            return UnitKind::Soldier;
        }
        unit_meta.unit_kind()
    }

    fn build_command_for_race(race: Option<Race>) -> Option<&'static str> {
        let race_value = race?;
        let preferred_name = match race_value {
            Race::Human => "CmdBuildHuman",
            Race::Orc => "CmdBuildOrc",
            Race::Nightelf => "CmdBuildNightElf",
            Race::Undead => "CmdBuildUndead",
            Race::Neutral => "CmdBuild",
        };
        Self::known_command(preferred_name).or_else(|| Self::known_command("CmdBuild"))
    }

    pub fn known_command(wanted_name: &str) -> Option<&'static str> {
        WARCRAFT_DATABASE
            .iter()
            .find_map(|(object_id, warcraft_object)| {
                let id_value = object_id.value();
                if warcraft_object.kind() == WarcraftObjectKind::Command
                    && id_value.eq_ignore_ascii_case(wanted_name)
                {
                    Some(id_value)
                } else {
                    None
                }
            })
    }

    pub fn is_context_command(slot: &GridSlotId) -> bool {
        let GridSlotId::Command(command_id) = slot else {
            return false;
        };
        Self::is_context_command_id(command_id.value())
    }

    pub fn is_context_command_id(command_name: &str) -> bool {
        CONTEXT_COMMAND_IDS
            .iter()
            .any(|context_name| context_name.eq_ignore_ascii_case(command_name))
    }

    pub fn submenu_back_command() -> Option<&'static str> {
        Self::known_command("CmdCancel")
    }

    pub fn mobile_command_ids() -> &'static [&'static str] {
        MOBILE_COMMAND_IDS.as_slice()
    }

    pub fn primary_commands_for(
        unit_meta: &UnitMeta,
        race: Option<Race>,
        object_id: &str,
    ) -> Vec<&'static str> {
        let unit_kind = Self::effective_kind(unit_meta);
        let has_builds = !unit_meta.builds().is_empty();
        let has_trains = !unit_meta.trains().is_empty();
        let has_production = has_builds || has_trains;
        let mut commands: Vec<&'static str> = Vec::new();
        match unit_kind {
            UnitKind::Building => {
                if BuildingTraits::can_attack(object_id) {
                    for command_name in TOWER_COMMAND_IDS.iter().copied() {
                        commands.push(command_name);
                    }
                }
                for command_name in BUILDING_COMMAND_IDS.iter().copied() {
                    if command_name.eq_ignore_ascii_case("CmdRally") && !has_production {
                        continue;
                    }
                    if command_name.eq_ignore_ascii_case("CmdCancelTrain") && !has_production {
                        continue;
                    }
                    commands.push(command_name);
                }
            }
            UnitKind::Worker | UnitKind::Soldier | UnitKind::Hero => {
                for command_name in MOBILE_COMMAND_IDS.iter().copied() {
                    commands.push(command_name);
                }
                if has_builds
                    && unit_kind == UnitKind::Worker
                    && let Some(build_command) = Self::build_command_for_race(race)
                {
                    commands.insert(0, build_command);
                }
            }
        }
        commands.retain(|command_name| !Self::is_context_command_id(command_name));
        commands
    }

    pub fn build_menu_commands_for(unit_meta: &UnitMeta) -> Vec<&'static str> {
        if Self::effective_kind(unit_meta) != UnitKind::Worker {
            return Vec::new();
        }
        if unit_meta.builds().is_empty() {
            return Vec::new();
        }
        Self::submenu_back_command().into_iter().collect()
    }
}

static MOBILE_COMMAND_IDS: LazyLock<Vec<&'static str>> = LazyLock::new(|| {
    ["CmdAttack", "CmdMove", "CmdStop", "CmdHoldPos", "CmdPatrol"]
        .into_iter()
        .filter_map(CommandCatalog::known_command)
        .collect()
});

static BUILDING_COMMAND_IDS: LazyLock<Vec<&'static str>> = LazyLock::new(|| {
    ["CmdCancelTrain", "CmdRally"]
        .into_iter()
        .filter_map(CommandCatalog::known_command)
        .collect()
});

static TOWER_COMMAND_IDS: LazyLock<Vec<&'static str>> = LazyLock::new(|| {
    ["CmdAttack", "CmdStop"]
        .into_iter()
        .filter_map(CommandCatalog::known_command)
        .collect()
});
