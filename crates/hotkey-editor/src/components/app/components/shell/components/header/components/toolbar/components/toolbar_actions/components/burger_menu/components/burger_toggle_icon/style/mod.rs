use tw_macro::tw;

classes! {
    base: tw![
        "flex",
        "items-center",
        "justify-center",
        "w-4",
        "h-4",
        "leading-none",
        "[&_svg]:block",
        "[&_svg]:w-full",
        "[&_svg]:h-full",
    ],
}
