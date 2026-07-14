use tw_macro::tw;
classes! {
    base: tw![
        "flex",
        "items-baseline",
        "justify-center",
        "flex-wrap",
        "gap-3",
        "flex-auto",
    ],
    mobile: tw![
        "mobile:hidden",
    ],
    tablet: tw![
        "tablet:hidden",
    ],
}
