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
        "border-[color:var(--race-accent,var(--color-warcraft-gold))]",
        "[--glow-color:var(--race-accent,var(--color-warcraft-gold))]",
        "shadow-glow-raised",
        "bg-warcraft-bg-panel/95",
    ],
}
