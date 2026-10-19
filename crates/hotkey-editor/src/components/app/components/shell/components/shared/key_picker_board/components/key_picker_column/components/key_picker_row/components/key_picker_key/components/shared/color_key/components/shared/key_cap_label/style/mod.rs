use tw_macro::tw;
// The icon takes the key's font size, so an arrow is drawn as large as a
// letter on the same key.
classes! {
    base: tw![
        "inline-flex",
        "items-center",
        "justify-center",
        "[&_svg]:size-[1em]",
    ],
}
