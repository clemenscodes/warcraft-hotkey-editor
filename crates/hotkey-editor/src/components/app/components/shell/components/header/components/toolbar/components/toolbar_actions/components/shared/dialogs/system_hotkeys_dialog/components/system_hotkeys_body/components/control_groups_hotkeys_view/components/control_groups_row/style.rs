use tw_macro::tw;
classes! {
    base: tw![
        "grid",
        "grid-cols-[repeat(10,11rem)]",
        "gap-3",
    ],
    mobile: tw![
        "mobile:grid-cols-5",
        "mobile:auto-rows-[minmax(72px,auto)]",
        "mobile:gap-1.5",
        "mobile:w-full",
    ],
    tablet: tw![
        "tablet:grid-cols-5",
        "tablet:auto-rows-[minmax(72px,auto)]",
        "tablet:gap-1.5",
        "tablet:w-full",
    ],
}
