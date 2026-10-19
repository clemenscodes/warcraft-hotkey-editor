use tw_macro::tw;

classes! {
    base: tw![
        "fixed",
        "pointer-events-none",
        "@container",
        "z-10",
        "overflow-hidden",
        "select-none",
        "border-2",
        "rounded-tile",
        "border-(--race-color,var(--color-warcraft-gold))",
        "[--glow-color:var(--race-color,var(--color-warcraft-gold))]",
        "shadow-glow-raised",
        "bg-panel-blue",
    ],
}
