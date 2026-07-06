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
        "[box-shadow:0_0_14px_color-mix(in_oklab,var(--color-warcraft-gold)_60%,transparent),0_8px_24px_color-mix(in_oklab,var(--color-warcraft-shadow)_60%,transparent)]",
        "data-[race=human]:border-race-human",
        "data-[race=human]:[box-shadow:var(--shadow-glow-human-14-raised)]",
        "data-[race=orc]:border-race-orc",
        "data-[race=orc]:[box-shadow:var(--shadow-glow-orc-14-raised)]",
        "data-[race=nightelf]:border-race-nightelf",
        "data-[race=nightelf]:[box-shadow:var(--shadow-glow-nightelf-14-raised)]",
        "data-[race=undead]:border-race-undead",
        "data-[race=undead]:[box-shadow:var(--shadow-glow-undead-14-raised)]",
        "data-[race=neutral]:border-warcraft-gold",
        "data-[race=neutral]:[box-shadow:0_0_14px_color-mix(in_oklab,var(--color-warcraft-gold)_45%,transparent),0_8px_24px_color-mix(in_oklab,var(--color-warcraft-shadow)_60%,transparent)]",
    ],
}

states! {
    GhostState,
    Default => tw!["bg-warcraft-bg-panel/95"],
    Command => tw!["bg-panel-blue-diag-95"],
}
