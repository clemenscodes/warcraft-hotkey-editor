use tw_macro::tw;

classes! {
    base: tw![
        "flex",
        "flex-row",
        "flex-[2_1_0]",
        "gap-2",
        "[&>button]:min-h-14!",
    ],
    mobile: tw![
        "mobile:flex-none",
        "mobile:w-[10rem]",
    ],
    tablet: tw![
        "tablet:flex-none",
        "tablet:w-[12rem]",
    ],
}
