use tw_macro::tw;
// The shared mini grid frame: a query container the reused `Grid` fills. It fills
// the width its page wrapper gives it (`w-full`) so the tiles size in `cqi` off that
// width, its height comes from the three rows of square tiles, and it is display-
// only, so pointer events pass through. It owns the tiles' SIZE — the mini border
// width and corner radius — by setting `--tile-border-width` / `--tile-corner-radius`
// on itself; those inherit down and each tile writes its OWN border and radius from
// them (a component owns its look, its parent owns its size). The wrapper owns the
// outer width and corner radius; this frame owns the chrome those clip.
classes! {
    base: tw![
        "w-full",
        "@container",
        "pointer-events-none",
        "p-1",
        "bg-warcraft-bg-panel/70",
        "border",
        "border-warcraft-blue",
        "[--tile-border-width:0.35cqi]",
        "[--tile-corner-radius:1.04cqi]",
    ],
}
