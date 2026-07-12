use tw_macro::tw;
// The empty detail pane's inner layout: the centered, muted empty-prompt treatment. The
// bordered surface now lives in the shared `DetailCard`; this leaf owns only its centring.
classes! {
    base: tw![
        "flex-1",
        "flex",
        "items-center",
        "justify-center",
        "min-h-64",
        "text-warcraft-text-faint",
        "italic",
    ],
}
