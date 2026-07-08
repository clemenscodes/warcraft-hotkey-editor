use tw_macro::tw;
// The mini-grid highlight layer: the gold wash filling the marked coordinate. The gold
// border and glow belong to the empty tile's own box (the `EmptyTile` root reacts to
// `:has(.highlight-overlay)`, and a box-shadow there is not clipped by the root's
// overflow); this layer carries the wash, which fills the clipped slot cleanly.
classes! {
    base: tw![
        "absolute",
        "inset-0",
        "pointer-events-none",
        "rounded-[var(--tile-corner-radius,5.2cqi)]",
        "bg-warcraft-gold/20",
    ],
}
