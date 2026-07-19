use tw_macro::tw;

classes! {
    base: tw![
        "block",
        "shrink-0",
        "w-4.5",
        "h-4.5",
        "pointer-events-none",
        "[&_svg]:block",
        "[&_svg]:w-full",
        "[&_svg]:h-full",
    ],
}
