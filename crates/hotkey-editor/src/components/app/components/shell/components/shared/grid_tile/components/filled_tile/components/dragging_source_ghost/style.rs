use tw_macro::tw;
// The dragging-source ghost: an inert presence marker for the tile currently lifted as a
// drag's source. It carries no look of its own — the dashed deep-blue ghost border, the
// dark placeholder fill, and the hiding of the lifted icon all belong to the `FilledTile`
// root, which reacts to `:has(.dragging-source-ghost)`, exactly as the drop-target and
// selection looks are driven. Keeping the look on the root (not an opaque cover here)
// lets a parent — the off-state position picker — recolor the source through the same
// selector. Pointer-transparent so the lifted source stays hit-testable and can become
// its own drop target.
classes! {
    base: tw![
        "absolute",
        "inset-0",
        "pointer-events-none",
    ],
}
