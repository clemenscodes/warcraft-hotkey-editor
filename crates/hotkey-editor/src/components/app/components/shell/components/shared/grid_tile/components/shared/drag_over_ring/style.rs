use tw_macro::tw;
// The drag-over marker: mounted on the tile the cursor currently hovers during a drag.
// It carries no look of its own — the gold border belongs to the tile root (a filled
// tile via `:has(.drag-over-ring)`, an empty drop target via
// `:has(.drop-target-overlay):has(.drag-over-ring)`). A pointer-transparent full-cover
// presence signal, exactly like the drop-target and selection markers it partners.
classes! {
    base: tw![
        "absolute",
        "inset-0",
        "pointer-events-none",
    ],
}
