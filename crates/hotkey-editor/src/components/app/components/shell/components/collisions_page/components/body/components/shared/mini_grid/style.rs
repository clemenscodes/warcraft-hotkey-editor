use tw_macro::tw;
// The mini grid frame: a small query container the reused `Grid` fills. Its width
// sets the whole grid's scale (the tiles size in `cqi` off it), its height comes
// from the three rows of square tiles, and it is display-only, so pointer events
// pass through.

classes! {
    base: tw![
        "shrink-0",
        "w-[106.67px]",
        "@container",
        "pointer-events-none",
        "p-1",
        "bg-warcraft-bg-panel/70",
        "border",
        "border-warcraft-blue",
        "rounded-hairline",
        "[&_.empty-tile]:border-[0.35cqi]!",
        "[&_.filled-tile]:border-[0.35cqi]!",
        "[&_.empty-tile]:rounded-[1.04cqi]!",
        "[&_.filled-tile]:rounded-[1.04cqi]!",
    ],
    mobile: tw!["mobile:w-[88px]"],
    tablet: tw!["tablet:w-[122.67px]"],
}
