use tw_macro::tw;
classes! {
    base: tw![
        "grid",
        "grid-cols-2",
        "gap-x-7",
        "gap-y-5",
        "items-start",
        "flex-none",
    ],
    mobile: tw![
        "mobile:flex",
        "mobile:flex-col",
        "mobile:items-center",
        "mobile:gap-6",
    ],
    tablet: tw![
        "tablet:grid-cols-[repeat(2,1fr)]",
        "tablet:gap-x-10",
        "tablet:gap-y-7",
    ],
}
