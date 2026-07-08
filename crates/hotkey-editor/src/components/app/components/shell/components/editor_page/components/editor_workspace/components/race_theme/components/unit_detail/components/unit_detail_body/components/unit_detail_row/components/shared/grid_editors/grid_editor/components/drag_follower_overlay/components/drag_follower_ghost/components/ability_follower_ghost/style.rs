use tw_macro::tw;

// The full floating-tile chrome (fixed position, border, race-tinted glow) written out,
// plus the ability menu's own panel background. The command ghost shares these *values*
// and writes its own list; neither can restyle the other.

classes! {
    base: tw![
        "fixed",
        "pointer-events-none",
        "@container",
        "z-1000",
        "overflow-hidden",
        "select-none",
        "border-2",
        "rounded-tile",
        "border-warcraft-gold",
        "shadow-glow-raised",
        "data-[race=human]:border-race-human",
        "data-[race=human]:[--glow-color:var(--color-race-human)]",
        "data-[race=human]:shadow-glow-raised",
        "data-[race=orc]:border-race-orc",
        "data-[race=orc]:[--glow-color:var(--color-race-orc)]",
        "data-[race=orc]:shadow-glow-raised",
        "data-[race=nightelf]:border-race-nightelf",
        "data-[race=nightelf]:[--glow-color:var(--color-race-nightelf)]",
        "data-[race=nightelf]:shadow-glow-raised",
        "data-[race=undead]:border-race-undead",
        "data-[race=undead]:[--glow-color:var(--color-race-undead)]",
        "data-[race=undead]:shadow-glow-raised",
        "data-[race=neutral]:border-warcraft-gold",
        "data-[race=neutral]:shadow-glow-raised",
        "bg-warcraft-bg-panel/95",
    ],
}
