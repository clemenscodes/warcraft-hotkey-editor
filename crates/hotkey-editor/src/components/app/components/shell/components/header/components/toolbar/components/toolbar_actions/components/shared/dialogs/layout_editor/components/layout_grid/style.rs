use tw_macro::tw;
classes! {
    base: tw![
        "grid",
        "[grid-template-columns:repeat(4,9rem)]",
        "[grid-auto-rows:9rem]",
        "gap-5",
        "mx-auto",
    ],
    mobile: tw![
        "mobile:justify-center",
        "mobile:[grid-template-columns:repeat(4,4rem)]",
        "mobile:[grid-auto-rows:4rem]",
        "mobile:gap-2",
    ],
    tablet: tw![
        "tablet:justify-center",
        "tablet:[grid-template-columns:repeat(4,4rem)]",
        "tablet:[grid-auto-rows:4rem]",
        "tablet:gap-2",
    ],
}
