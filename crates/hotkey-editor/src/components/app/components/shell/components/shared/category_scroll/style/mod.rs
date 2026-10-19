use tw_macro::tw;

// A plain vertical list. Below 1920px the editor page mounts the pager instead
// of the aside, so the only list rendering there is the search dialog's, and a
// dialog wants the vertical list.
classes! {
    base: tw![
        "flex-1",
        "flex",
        "flex-col",
        "min-h-0",
        "overflow-y-auto",
        "overflow-x-hidden",
        "pr-1",
    ],
}
