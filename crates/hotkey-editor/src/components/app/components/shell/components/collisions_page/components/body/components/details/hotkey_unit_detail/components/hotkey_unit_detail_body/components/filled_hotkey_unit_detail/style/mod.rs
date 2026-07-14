use tw_macro::tw;
// The populated detail pane's inner layout. The bordered surface now lives in the shared
// `DetailCard`; this leaf owns only how its header and cards stack inside that surface.
classes! {
    base: tw![
        "flex-1",
        "min-h-0",
        "flex",
        "flex-col",
        "gap-6",
    ],
}
