use tw_macro::tw;
// The mini grid frame: a small query container the reused `Grid` fills. Its width
// sets the whole grid's scale (the tiles size in `cqi` off it), its height comes
// from the three rows of square tiles, and it is display-only, so pointer events
// pass through.

classes! {
    base: tw![
        "shrink-0",
        "w-[calc(80px/3*4)]",
        "[container-type:inline-size]",
        "pointer-events-none",
        "p-[3px]",
        "bg-warcraft-bg-panel/70",
        "border",
        "border-warcraft-blue",
        "rounded-hairline",
        "[&_.empty-tile]:border-[0.35cqi]!",
        "[&_.filled-tile]:border-[0.35cqi]!",
        "[&_.empty-tile]:rounded-[1.04cqi]!",
        "[&_.filled-tile]:rounded-[1.04cqi]!",
    ],
    mobile: tw!["mobile:w-[calc(66px/3*4)]"],
    tablet: tw!["tablet:w-[calc(92px/3*4)]"],
}
