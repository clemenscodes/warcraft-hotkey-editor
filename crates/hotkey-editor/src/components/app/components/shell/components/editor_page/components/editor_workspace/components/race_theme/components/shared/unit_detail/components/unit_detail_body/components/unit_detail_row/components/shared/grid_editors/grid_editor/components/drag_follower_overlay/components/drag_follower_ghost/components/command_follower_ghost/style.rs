use tw_macro::tw;

// The full floating-tile chrome (fixed position, border, race-tinted glow) written out,
// plus the command menu's own blue panel background. The ability ghost shares these
// *values* and writes its own list; neither can restyle the other.

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
        "border-(--race-accent,var(--color-warcraft-gold))",
        "[--glow-color:var(--race-accent,var(--color-warcraft-gold))]",
        "shadow-glow-raised",
        "bg-panel-blue",
    ],
}
