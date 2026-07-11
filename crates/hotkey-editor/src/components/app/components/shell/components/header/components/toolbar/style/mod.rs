use tw_macro::tw;
// One fixed inter-control gap on every band: the icon buttons scale, but a few px of gap
// between them reads the same at any size, so this needs neither a per-band value nor a
// clamp — a single `gap-1` in BASE covers phone through 4K.

classes! {
    base: tw![
        "flex",
        "flex-row",
        "items-center",
        "justify-end",
        "gap-1",
        "min-w-0",
    ],
}
