use tw_macro::tw;
// The scroll body of a position-picker dialog: the DialogContent's scroll region
// and the centered column holding the explainer and the grid. Shared by the
// off-state and upgraded-form pickers.

classes! {
    base: tw![
        "flex-1",
        "min-h-0",
        "flex",
        "flex-col",
        "items-center",
        "gap-6",
        "overflow-y-auto",
        "pt-8",
        "px-10",
        "pb-10",
    ],
}
