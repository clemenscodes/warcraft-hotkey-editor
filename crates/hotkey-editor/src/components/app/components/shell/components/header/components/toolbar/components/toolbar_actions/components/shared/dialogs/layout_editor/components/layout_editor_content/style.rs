use tw_macro::tw;
classes! {
    base: tw![
        "flex",
        "flex-col",
        "items-center",
        "justify-center",
        "gap-[4rem]",
    ],
    mobile: tw![
        "mobile:justify-start",
        "mobile:gap-[20px]",
    ],
    tablet: tw![
        "tablet:justify-start",
        "tablet:gap-[20px]",
    ],
}
