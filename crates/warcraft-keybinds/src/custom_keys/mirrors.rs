//! Hard-coded keybind mirror tables: a handful of build/morph commands whose
//! live-game keybind lives in a section other than the one the editor's grid
//! button edits. Normalization copies the edited section onto its mirror so the
//! game honors the position. These are deliberately hand-listed, not derived
//! from the database — see each struct's doc for why a database field cannot
//! distinguish the safe cases from the ones a mirror would clobber.

use warcraft_api::WarcraftObjectId;

/// Pairs a `CmdBuild*` command with the build ability whose keybind the live
/// game actually reads. The ability (e.g. `AHbu`) is what plays in game; the
/// `CmdBuild*` command only drives the in-game hotkey editor. Moving the build
/// command in the editor must write both, so the live game honors the position.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub(crate) struct BuildCommandMirror {
    command_id: WarcraftObjectId,
    ability_id: WarcraftObjectId,
}

impl BuildCommandMirror {
    pub(crate) fn command_id(&self) -> WarcraftObjectId {
        self.command_id
    }

    pub(crate) fn ability_id(&self) -> WarcraftObjectId {
        self.ability_id
    }
}

pub(crate) const BUILD_COMMAND_MIRRORS: &[BuildCommandMirror] = &[
    BuildCommandMirror {
        command_id: WarcraftObjectId::new("CmdBuildHuman"),
        ability_id: WarcraftObjectId::new("AHbu"),
    },
    BuildCommandMirror {
        command_id: WarcraftObjectId::new("CmdBuildOrc"),
        ability_id: WarcraftObjectId::new("AObu"),
    },
    BuildCommandMirror {
        command_id: WarcraftObjectId::new("CmdBuildUndead"),
        ability_id: WarcraftObjectId::new("AUbu"),
    },
    BuildCommandMirror {
        command_id: WarcraftObjectId::new("CmdBuildNightElf"),
        ability_id: WarcraftObjectId::new("AEbu"),
    },
];

/// Pairs a permanent one-way morph ability with the produced-unit section the
/// live game reads its keybind from. The Obsidian Statue's Transform (`Aave`)
/// is irreversible — a Destroyer (`ubsp`) can never become a Statue again — so
/// the morph is a one-time command whose keybind lives in a section keyed by
/// the produced unit id, separate from the `Aave` ability the editor's grid
/// button edits. Editing the button only touches `Aave`, so without this mirror
/// the produced-unit section keeps its stale default hotkey and the morph binds
/// the wrong key in game.
///
/// This is why the list is a single entry and is not derived from the database:
/// every *other* morph is a reversible toggle whose second state is the base
/// unit's off-state (`Unhotkey`/`Unbuttonpos`, handled by
/// `sync_mirrored_off_states` and the independent-off-slot logic), so it has no
/// orphaned produced-unit command section to sync. The `morph_target_unit`
/// database field cannot distinguish these — it is also set for reversible
/// toggles, summon spells, and mount actions, several of whose targets are
/// ordinary train/sell units that this mirror would clobber, the same invariant
/// that makes [`BuildCommandMirror`] safe.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub(crate) struct MorphAbilityMirror {
    ability_id: WarcraftObjectId,
    produced_unit_id: WarcraftObjectId,
}

impl MorphAbilityMirror {
    pub(crate) fn ability_id(&self) -> WarcraftObjectId {
        self.ability_id
    }

    pub(crate) fn produced_unit_id(&self) -> WarcraftObjectId {
        self.produced_unit_id
    }
}

pub(crate) const MORPH_ABILITY_MIRRORS: &[MorphAbilityMirror] = &[MorphAbilityMirror {
    ability_id: WarcraftObjectId::new("Aave"),
    produced_unit_id: WarcraftObjectId::new("ubsp"),
}];
