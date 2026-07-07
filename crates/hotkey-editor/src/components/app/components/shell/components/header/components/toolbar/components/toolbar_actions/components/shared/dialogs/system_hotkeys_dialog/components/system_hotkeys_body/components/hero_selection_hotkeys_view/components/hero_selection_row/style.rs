use tw_macro::tw;
classes! {
    base: tw![
        "grid",
        "grid-cols-[repeat(3,26rem)]",
        "gap-6",
    ],
    mobile: tw![
        "mobile:grid-cols-3",
        "mobile:gap-2",
        "mobile:w-full",
        "mobile:max-w-120",
        "mobile:mx-auto",
    ],
    tablet: tw![
        "tablet:grid-cols-3",
        "tablet:gap-2",
        "tablet:w-full",
        "tablet:max-w-120",
        "tablet:mx-auto",
    ],
}
