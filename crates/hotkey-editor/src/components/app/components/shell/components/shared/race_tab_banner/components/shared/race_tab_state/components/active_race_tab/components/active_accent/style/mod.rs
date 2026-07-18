use tw_macro::tw;

classes! {
    base: tw![
        "absolute",
        "inset-0",
        "z-3",
        "rounded-card",
        "border",
        "border-(--race-color)",
        "[--glow-color:var(--race-color)]",
        "shadow-glow-strong",
        "pointer-events-none",
    ],
}
