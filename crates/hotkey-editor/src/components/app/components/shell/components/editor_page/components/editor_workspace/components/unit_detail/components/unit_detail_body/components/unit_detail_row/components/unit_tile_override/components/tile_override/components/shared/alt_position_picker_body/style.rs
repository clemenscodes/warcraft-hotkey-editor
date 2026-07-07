use tw_macro::tw;
// The body of a position-picker dialog: a centered column holding the explainer and
// the grid. Shared by the off-state and upgraded-form pickers.

classes! {
    base: tw![
        "flex",
        "flex-col",
        "items-center",
        "gap-6",
        "pt-8", "px-10", "pb-10",
    ],
}
