use tw_macro::tw;
// The draggable marker: mounted on every tile the domain allows the player to drag.
// It carries no look of its own — the grab cursor belongs to the `GridEditorTile` root
// (via `:has(.draggable-marker)`), and the off-state position picker keys its dim /
// gold treatment off the same presence. A pointer-transparent full-cover presence
// signal, exactly like the tile's other mounted markers.
classes! {
    base: tw![
        "absolute",
        "inset-0",
        "pointer-events-none",
    ],
}
