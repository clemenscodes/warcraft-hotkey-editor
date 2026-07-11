use tw_macro::tw;
// The blocked-drop-target layer: a faint danger wash filling the slot. The danger
// border, dashed style, and not-allowed cursor belong to the empty tile's own border
// (the `EmptyTile` root reacts to `:has(.blocked-drop-target-overlay)`); this layer
// carries only the wash, which fills the clipped slot cleanly.
classes! {
    base: tw![
        "absolute",
        "inset-0",
        "pointer-events-none",
        "rounded-(--tile-corner-radius,5.2cqi)",
        "[background:color-mix(in_oklab,var(--color-warcraft-danger)_4%,transparent)]",
    ],
}
