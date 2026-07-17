use tw_macro::tw;

// The five races sit on one row and each chip is as wide as its own name, because
// a fixed column count would size them to the grid instead of to the label and
// cut "Nightelf" to fit. Nothing here may be shrunk to make it fit: it wraps.
classes! {
    base: tw![
        "flex",
        "flex-wrap",
        "gap-1.5",
    ],
}
