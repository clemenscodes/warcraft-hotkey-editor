use tw_macro::tw;
classes! {
    base: tw![
        "flex",
        "flex-wrap",
        "justify-end",
        "gap-4",
        "flex-none",
        "pt-4",
    ],
    mobile: tw![
        "mobile:flex-nowrap",
        "mobile:*:flex-1",
    ],
}
