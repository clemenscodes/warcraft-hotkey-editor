use tw_macro::tw;
// The top row of the alt-state block: label on the left, the position button and key
// cell on the right.

classes! {
    base: tw![
        "grid",
        "grid-cols-[minmax(0,1fr)_auto_auto]",
        "items-center",
        "gap-x-3.5",
    ],
}
