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
    ],
    mobile: tw![
        "mobile:flex-col",
        "mobile:items-center",
        "mobile:gap-1.5",
    ],
}
