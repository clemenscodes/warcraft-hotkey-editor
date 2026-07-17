use tw_macro::tw;

// One row, each toggle as wide as its own label. They only ever collided with
// each other because the shared button inflated every label to uppercase and
// then hid the overflow behind an ellipsis. With the label at its true width
// all four fit, and if a translation ever makes one too long the row wraps
// rather than cutting a word in half.
classes! {
    base: tw![
        "flex",
        "flex-wrap",
        "gap-1.5",
    ],
}
