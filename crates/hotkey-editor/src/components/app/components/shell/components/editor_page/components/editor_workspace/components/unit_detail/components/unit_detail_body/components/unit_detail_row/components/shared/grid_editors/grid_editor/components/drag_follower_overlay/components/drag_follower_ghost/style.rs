use super::state::GhostState;
use tw_macro::tw;

classes! {
    base: tw![
        "fixed",
        "pointer-events-none",
        "[container-type:inline-size]",
        "z-[1000]",
        "overflow-hidden",
        "select-none",
        "border-2",
        "rounded-tile",
        "border-warcraft-gold",
        "[box-shadow:var(--shadow-glow-gold-raised)]",
        "data-[race=human]:border-race-human",
        "data-[race=human]:[box-shadow:var(--shadow-glow-human-raised)]",
        "data-[race=orc]:border-race-orc",
        "data-[race=orc]:[box-shadow:var(--shadow-glow-orc-raised)]",
        "data-[race=nightelf]:border-race-nightelf",
        "data-[race=nightelf]:[box-shadow:var(--shadow-glow-nightelf-raised)]",
        "data-[race=undead]:border-race-undead",
        "data-[race=undead]:[box-shadow:var(--shadow-glow-undead-raised)]",
        "data-[race=neutral]:border-warcraft-gold",
        "data-[race=neutral]:[box-shadow:var(--shadow-glow-gold-select-raised)]",
    ],
}

states! {
    GhostState,
    Default => tw!["bg-warcraft-bg-panel/95"],
    Command => tw!["bg-panel-blue-diag-95"],
}
