use std::sync::LazyLock;

use warcraft_api::{UnitKind, UnitMeta, WarcraftObjectKind};
use warcraft_database::WARCRAFT_DATABASE;

use crate::domain::building_traits::BuildingTraits;
use crate::domain::grid_slot::GridSlotId;
use crate::domain::unit_kind::UnitKindHelpers;

/// Commands the Warcraft III engine overlays *dynamically* during transient
/// game states — they are never persistent command-card slots. The in-game
/// hotkey editor reflects this: `CmdCancel` etc. don't appear as
/// configurable buttons on a shop, a tower, or any normal unit. The engine
/// paints them on the bottom-right while the player is mid-interaction
/// (single-target ability picking, building placement, training queue
/// cancel, hero revive cancel, etc.).
///
/// The one place the engine *does* expose a `CmdCancel` slot persistently
/// is inside submenus modelled as their own command card: the hero
/// learn-skill menu's "back" button and the worker build menu's "back"
/// button. Anywhere else, the editor must keep them out of slot lists —
/// otherwise their database-default position silently overlaps a real
/// ability (the original Ancient-of-Wonders bug, where `CmdCancel` was
/// stacked invisibly underneath the Root ability).
///
/// `CommandCatalog::is_context_command` and its string-id sibling
/// `is_context_command_id` consult this list. The slot-list builders
/// inside this module defensively filter through it so a future addition
/// to `BUILDING_COMMAND_IDS` etc. cannot reintroduce the regression.
/// `submenu_back_command` is the one sanctioned way to add `CmdCancel`
/// back into a slot list — used by hero-research and worker-build menus.
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
        Self::is_context_command_id(command_name)
    }

    pub(crate) fn is_context_command_id(command_name: &str) -> bool {
        CONTEXT_COMMAND_IDS
            .iter()
            .any(|context_name| context_name.eq_ignore_ascii_case(command_name))
    }

    /// Sanctioned entry point for putting `CmdCancel` back into a slot
    /// list — used as the "back" button inside submenus the editor models
    /// as their own command card (hero learn-skill, worker build menu).
    /// Any other call site adding `CmdCancel` directly is almost certainly
    /// a bug that will overlap real abilities at the database-default
    /// position.
    pub(crate) fn submenu_back_command() -> Option<&'static str> {
        Self::known_command("CmdCancel")
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
        // Defense in depth: even if a future maintainer adds a context
        // command (CmdCancel etc.) to one of the static command lists
        // above, strip it here so it can never become a persistent slot
        // on a unit's main command card. The only sanctioned path for
        // surfacing those commands is `submenu_back_command()`.
        commands.retain(|command_name| !Self::is_context_command_id(command_name));
        commands
    }

    pub(crate) fn build_menu_commands_for(unit_meta: &UnitMeta) -> Vec<&'static str> {
        if UnitKindHelpers::effective_kind(unit_meta) != UnitKind::Worker {
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

// CmdCancel is intentionally absent. See `CONTEXT_COMMAND_IDS` for the
// rationale. `primary_commands_for` also filters context commands
// defensively, so adding one here would still be stripped — but keeping
// the list itself clean documents intent and matches what the in-game
// hotkey editor exposes for the same units.
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
