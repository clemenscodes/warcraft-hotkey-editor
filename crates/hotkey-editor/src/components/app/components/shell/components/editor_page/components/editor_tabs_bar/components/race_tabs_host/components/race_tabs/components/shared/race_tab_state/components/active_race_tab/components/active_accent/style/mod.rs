use tw_macro::tw;

// The active accent, stacked on top of the base tab as a non-interactive overlay ring:
// the solid accent border and strong glow that mark the current tab. It reads the
// `--race-color` the per-race wrapper publishes, so one overlay serves every race.
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
