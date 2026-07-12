use tw_macro::tw;
// The conflict panel's inner layout: how the caption and the clash layouts stack, centred,
// inside the shared `PanelCard` surface. The surface chrome (tint, border, radius, padding)
// lives in `PanelCard`; this leaf owns only the stacking of its parts.
classes! {
    base: tw![
        "flex",
        "flex-col",
        "gap-6",
        "items-center",
        "min-w-0",
    ],
}
