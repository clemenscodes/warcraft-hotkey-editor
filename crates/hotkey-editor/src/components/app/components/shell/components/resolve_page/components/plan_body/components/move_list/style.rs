use tw_macro::tw;

// The auto-fill grid of move cards. The identity class (`move-list`, derived from this
// directory) and its `data-category` attribute are coupled to the e2e suite.
classes! {
    base: tw![
        "grid",
        "grid-cols-[repeat(auto-fill,minmax(min(760px,100%),1fr))]",
        "gap-4",
        "content-start",
    ],
}
