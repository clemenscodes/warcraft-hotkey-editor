use tw_macro::tw;

classes! {
    base: tw![
        "flex",
        "items-stretch",
        "flex-none",
        "gap-10",
        "min-h-36",
    ],
    mobile: tw![
        "mobile:flex-col",
        "mobile:min-h-0",
        "mobile:gap-2.5",
    ],
}
