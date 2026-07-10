use tw_macro::tw;
classes! {
    base: tw![
        "grid",
        "grid-cols-[repeat(4,9rem)]",
        "auto-rows-[9rem]",
        "gap-5",
    ],
    mobile: tw![
        "mobile:justify-center",
        "mobile:grid-cols-[repeat(4,4rem)]",
        "mobile:auto-rows-[4rem]",
        "mobile:gap-2",
    ],
    tablet: tw![
        "tablet:justify-center",
        "tablet:grid-cols-[repeat(4,4rem)]",
        "tablet:auto-rows-[4rem]",
        "tablet:gap-2",
    ],
}
