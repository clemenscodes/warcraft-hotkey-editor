use tw_macro::tw;

classes! {
    // The bottom divider is drawn by a pseudo element inset by the same px-4 as
    // the padding, so it stops short of the outer edges exactly like the app
    // header divider rather than running to the viewport edge.
    base: tw![
        "relative",
        "flex",
        "items-center",
        "justify-between",
        "gap-3",
        "shrink-0",
        "px-4",
        "py-2",
        "after:content-['']",
        "after:absolute",
        "after:bottom-0",
        "after:left-4",
        "after:right-4",
        "after:h-px",
        "after:bg-warcraft-gold/30",
    ],
}
