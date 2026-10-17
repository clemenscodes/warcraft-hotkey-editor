use tw_macro::tw;
classes! {
    base: tw![
        "@container",
        "grid",
        "grid-cols-[repeat(4,9rem)]",
        "auto-rows-[9rem]",
        "gap-5",
    ],
    mobile: tw![
        "mobile:w-full",
        "mobile:grid-cols-4",
        "mobile:auto-rows-auto",
        "mobile:gap-2",
    ],
    tablet: tw![
        "tablet:justify-center",
        "tablet:grid-cols-[repeat(4,4rem)]",
        "tablet:auto-rows-[4rem]",
        "tablet:gap-2",
    ],
}
