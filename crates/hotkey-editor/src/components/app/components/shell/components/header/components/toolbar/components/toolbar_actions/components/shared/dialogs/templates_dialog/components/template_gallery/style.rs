use tw_macro::tw;
classes! {
    base: tw![
        "grid",
        "grid-cols-2",
        "gap-9",
        "w-full",
    ],
    mobile: tw![
        "mobile:grid-cols-1",
        "mobile:gap-[10px]",
    ],
    tablet: tw![
        "tablet:grid-cols-1",
        "tablet:gap-[10px]",
    ],
}
