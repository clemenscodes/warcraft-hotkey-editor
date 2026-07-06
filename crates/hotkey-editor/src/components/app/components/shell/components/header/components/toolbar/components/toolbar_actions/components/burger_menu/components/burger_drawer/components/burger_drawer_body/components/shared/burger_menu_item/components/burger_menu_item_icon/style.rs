use tw_macro::tw;
classes! {
    base: tw![
        "inline-flex",
        "items-center",
        "justify-center",
        "w-5",
        "h-5",
        "shrink-0",
        "text-inherit",
        "[&_svg]:w-full",
        "[&_svg]:h-full",
    ],
}
