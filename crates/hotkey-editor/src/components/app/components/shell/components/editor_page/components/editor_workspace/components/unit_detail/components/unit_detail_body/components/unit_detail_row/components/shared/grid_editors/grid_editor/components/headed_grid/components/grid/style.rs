use tw_macro::tw;
// The generic grid shape: four equal columns of tile squares, filling whatever
// query container the extension gives it. The gap is in `cqi` so it scales with
// that container — the editor's is full width, a mini grid's is small, and the
// same shape renders at both sizes.

classes! {
    base: tw![
        "grid",
        "grid-cols-4",
        "gap-[1.04cqi]",
        "w-full",
        "overflow-visible",
    ],
}
