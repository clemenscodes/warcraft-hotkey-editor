use tw_macro::tw;
// The shared mini grid frame: a query container the reused `Grid` fills. It fills
// the width its page wrapper gives it (`w-full`) so the tiles size in `cqi` off that
// width, its height comes from the three rows of square tiles, and it is display-
// only, so pointer events pass through. The tile-scope overrides shrink the base
// tile's border and radius to the mini scale. The wrapper owns the outer width and
// corner radius; this frame owns the chrome those clip.
classes! {
    base: tw![
        "w-full",
        "@container",
        "pointer-events-none",
        "p-1",
        "bg-warcraft-bg-panel/70",
        "border",
        "border-warcraft-blue",
        "[&_.empty-tile]:border-[0.35cqi]!",
        "[&_.filled-tile]:border-[0.35cqi]!",
        "[&_.empty-tile]:rounded-[1.04cqi]!",
        "[&_.filled-tile]:rounded-[1.04cqi]!",
    ],
}
