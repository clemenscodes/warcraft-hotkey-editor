use tw_macro::tw;

// A layout-neutral grouping wrapper: the race tabs are the flex item the tab bar sizes,
// so the host adds only its identity and no box of its own.
classes! {
    base: tw![
        "contents",
    ],
}
