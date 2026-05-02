use std::sync::LazyLock;

use warcraft_api::{UnitKind, UnitMeta, WarcraftObjectKind};
use warcraft_database::WARCRAFT_DATABASE;

use crate::domain::building_traits::BuildingTraits;
use crate::domain::grid_slot::GridSlotId;
use crate::domain::unit_kind::UnitKindHelpers;

const CONTEXT_COMMAND_IDS: &[&str] = &[
    "CmdCancel",
    "CmdCancelBuild",
    "CmdCancelRevive",
    "CmdCancelTrain",
];

pub(crate) struct CommandCatalog;

impl CommandCatalog {
    pub(crate) fn known_command(wanted_name: &str) -> Option<&'static str> {
        WARCRAFT_DATABASE
            .iter()
            .find_map(|(object_id, warcraft_object)| {
                if warcraft_object.kind() == WarcraftObjectKind::Command
                    && object_id.value().eq_ignore_ascii_case(wanted_name)
                {
                    Some(object_id.value())
                } else {
                    None
                }
            })
    }

    pub(crate) fn is_context_command(slot: &GridSlotId) -> bool {
        let GridSlotId::Command(command_name) = slot else {
            return false;
        };
        CONTEXT_COMMAND_IDS
            .iter()
            .any(|context_name| context_name.eq_ignore_ascii_case(command_name))
    }

    pub(crate) fn mobile_command_ids() -> &'static [&'static str] {
        MOBILE_COMMAND_IDS.as_slice()
    }

    pub(crate) fn primary_commands_for(
        unit_meta: &UnitMeta,
        race: Option<warcraft_api::Race>,
        object_id: &str,
    ) -> Vec<&'static str> {
        let unit_kind = UnitKindHelpers::effective_kind(unit_meta);
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
                    && let Some(build_command) =
                        crate::domain::races::RaceLabels::build_command(race)
                {
                    commands.insert(0, build_command);
                }
            }
        }
        commands
    }

    pub(crate) fn build_menu_commands_for(unit_meta: &UnitMeta) -> Vec<&'static str> {
        if UnitKindHelpers::effective_kind(unit_meta) != UnitKind::Worker {
            return Vec::new();
        }
        if unit_meta.builds().is_empty() {
            return Vec::new();
        }
        Self::known_command("CmdCancel").into_iter().collect()
    }
}

static MOBILE_COMMAND_IDS: LazyLock<Vec<&'static str>> = LazyLock::new(|| {
    ["CmdAttack", "CmdMove", "CmdStop", "CmdHoldPos", "CmdPatrol"]
        .into_iter()
        .filter_map(CommandCatalog::known_command)
        .collect()
});

static BUILDING_COMMAND_IDS: LazyLock<Vec<&'static str>> = LazyLock::new(|| {
    ["CmdCancelTrain", "CmdRally", "CmdCancel"]
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
