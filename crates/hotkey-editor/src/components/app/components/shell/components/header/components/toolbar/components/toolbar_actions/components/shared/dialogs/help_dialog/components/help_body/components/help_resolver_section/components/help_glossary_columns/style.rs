use tw_macro::tw;
classes! {
    base: tw![
        "flex",
        "flex-row",
        "items-start",
        "gap-[3.2rem]",
    ],
    mobile: tw![
        "mobile:flex-col",
        "mobile:gap-[2.6rem]",
    ],
    tablet: tw![
        "tablet:flex-col",
        "tablet:gap-[2.6rem]",
    ],
}
