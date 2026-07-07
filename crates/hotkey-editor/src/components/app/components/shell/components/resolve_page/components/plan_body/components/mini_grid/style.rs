use tw_macro::tw;
// The move's mini grid frame: a query container the reused `Grid` fills. Its width
// sets the whole grid's scale (the tiles size in `cqi` off it), its height comes
// from the three rows of square tiles, and it is display-only, so pointer events
// pass through.

classes! {
    base: tw![
        "flex-[1_1_auto]",
        "w-full",
        "min-w-0",
        "[container-type:inline-size]",
        "pointer-events-none",
        "p-1",
        "bg-warcraft-bg-panel/70",
        "border",
        "border-warcraft-blue",
        "rounded-control",
        "[&_.empty-tile]:border-[0.35cqi]!",
        "[&_.filled-tile]:border-[0.35cqi]!",
        "[&_.empty-tile]:rounded-[1.04cqi]!",
        "[&_.filled-tile]:rounded-[1.04cqi]!",
    ],
}
