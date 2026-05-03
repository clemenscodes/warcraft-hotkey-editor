/// Set of runtime contexts in which a system hotkey is active. Two bindings
/// can only collide if their context sets intersect — e.g. an inventory
/// hotkey (player-only) cannot in practice conflict with a replay-only
/// hotkey, even if both bind the same key+modifier, because the contexts
/// never overlap at runtime.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) struct ContextSet {
    in_player: bool,
    in_observer: bool,
    in_replay: bool,
}

impl ContextSet {
    pub(crate) const ALWAYS: Self = Self {
        in_player: true,
        in_observer: true,
        in_replay: true,
    };

    pub(crate) const PLAYER_ONLY: Self = Self {
        in_player: true,
        in_observer: false,
        in_replay: false,
    };

    pub(crate) const OBSERVER_ONLY: Self = Self {
        in_player: false,
        in_observer: true,
        in_replay: false,
    };

    pub(crate) const REPLAY_ONLY: Self = Self {
        in_player: false,
        in_observer: false,
        in_replay: true,
    };

    pub(crate) fn overlaps(self, other: Self) -> bool {
        (self.in_player && other.in_player)
            || (self.in_observer && other.in_observer)
            || (self.in_replay && other.in_replay)
    }
}

/// Look up the runtime context(s) in which the given system hotkey section is
/// active. Defaults to `ALWAYS` for any section not explicitly enumerated, so
/// unmodelled keys err on the side of overflagging conflicts rather than
/// missing them.
///
/// TODO: audit each assignment against in-game behavior. The current mapping
/// is the user's best guess based on category and label semantics.
pub(crate) fn context_set_for(section_id: &str) -> ContextSet {
    match section_id {
        // Inventory slots — only meaningful while playing as a participant.
        "itm1" | "itm2" | "itm3" | "itm4" | "itm5" | "itm6" => ContextSet::PLAYER_ONLY,

        // Hero selection — player-only.
        "her1" | "her2" | "her3" => ContextSet::PLAYER_ONLY,

        // Control groups — player-only (you can't select your own units when
        // observing or replaying).
        "Ctr1" | "Ctr2" | "Ctr3" | "Ctr4" | "Ctr5" | "Ctr6" | "Ctr7" | "Ctr8" | "Ctr9"
        | "Ctr0" => ContextSet::PLAYER_ONLY,

        // General commands that act on player-controlled units / resources.
        "sbgp"            // Switch subgroups
        | "sidw"          // Select idle workers
        | "mpng"          // Minimap ping
        | "TFmv"          // Toggle formation movement
        | "Ally"          // Toggle Allies panel
        | "QSav"          // Quick save
        | "MSav"          // Save menu
        | "MLod"          // Load menu
            => ContextSet::PLAYER_ONLY,

        // Observer-mode panels and switches.
        "THer" | "TSta" | "TPUQ" | "TSel" | "TMap" | "TToD" | "TAll" | "SPQM" | "SULM" => {
            ContextSet::OBSERVER_ONLY
        }

        // Replay-mode controls.
        "TRpl" | "TPPl" | "InGS" | "DeGS" | "TFoW" | "TAuC" => ContextSet::REPLAY_ONLY,

        // Everything else — menu toggles, camera controls, audio toggles,
        // generic settings — is active in every runtime context.
        _ => ContextSet::ALWAYS,
    }
}
