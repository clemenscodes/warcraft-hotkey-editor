use tw_macro::tw;

classes! {
    base: tw![
        "flex",
        "flex-row",
        "items-end",
        "justify-center",
        "gap-3",
        "w-full",
        "@container",
        "focus:outline-none",
    ],
    mobile: tw![
        "mobile:flex-col",
        "mobile:items-stretch",
        "mobile:gap-1.5",
    ],
}
