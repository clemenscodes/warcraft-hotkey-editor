use tw_macro::tw;

// The room left under the card's other rows, so the level row it holds sits on
// the card's bottom edge whichever of those rows are shown.
classes! {
    base: tw![
        "flex",
        "flex-1",
        "flex-col",
        "justify-end",
        "min-h-0",
    ],
}
