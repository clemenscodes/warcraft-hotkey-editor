use tw_macro::tw;
// The command grid shape: four equal columns of tile squares, filling the preview's
// query container. The gap is in `cqi` so it scales with that container. The same
// shape values are written by the editor and mini grids.

classes! {
    base: tw![
        "grid",
        "grid-cols-4",
        "gap-[1.04cqi]",
        "w-full",
        "overflow-visible",
    ],
}
