use tw_macro::tw;
// The selection marker. It carries no border or glow of its own — those belong to the
// tile's own border, which the `FilledTile` root turns gold and lights (`[&:has(
// .selection-ring)]` in its style), so the selected look replaces the resting border
// rather than stacking a second ring. This inert, pointer-transparent layer exists to
// BE that mounted signal: the tile is selected exactly when this child is mounted, and
// the shell's scroll/focus coordinator finds the selected tile with `:has(.selection-
// ring)` instead of a `data-selected` attribute.
classes! {
    base: tw![
        "absolute",
        "inset-0",
        "-z-10",
        "pointer-events-none",
    ],
}
