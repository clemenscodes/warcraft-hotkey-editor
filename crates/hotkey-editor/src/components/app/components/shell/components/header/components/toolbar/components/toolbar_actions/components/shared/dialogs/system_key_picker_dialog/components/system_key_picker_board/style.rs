use tw_macro::tw;
classes! {
    base: tw![
        "flex",
        "flex-row",
        "items-end",
        "gap-[0.8rem]",
    ],
    mobile: tw![
        "mobile:flex-col",
        "mobile:items-center",
        "mobile:gap-[0.4rem]",
    ],
}
