use tw_macro::tw;
// The drop-target layer. The dashed accent border, the gold-on-hover, and the cursor
// all belong to the empty tile's own border (the `EmptyTile` root reacts to
// `:has(.drop-target-overlay)`), so this layer stays inert: its mere presence IS the
// drop-target signal. It is a pointer-transparent full-cover marker with no look of its
// own; a parent that recolors the drop target (the off-state picker) styles the tile
// through this same `:has` selector.
classes! {
    base: tw![
        "absolute",
        "inset-0",
        "pointer-events-none",
    ],
}
