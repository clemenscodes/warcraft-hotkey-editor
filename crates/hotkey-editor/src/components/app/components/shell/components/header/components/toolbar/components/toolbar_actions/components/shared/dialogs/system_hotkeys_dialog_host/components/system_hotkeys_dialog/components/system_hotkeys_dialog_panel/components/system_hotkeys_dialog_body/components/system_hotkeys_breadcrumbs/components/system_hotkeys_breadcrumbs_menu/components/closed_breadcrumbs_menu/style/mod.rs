use tw_macro::tw;
classes! {
    base: tw![
        "flex",
        "items-baseline",
        "justify-center",
        "flex-wrap",
        "gap-3",
        "flex-[1_1_auto]",
    ],
    mobile: tw![
        "mobile:hidden",
    ],
    tablet: tw![
        "tablet:hidden",
    ],
}
