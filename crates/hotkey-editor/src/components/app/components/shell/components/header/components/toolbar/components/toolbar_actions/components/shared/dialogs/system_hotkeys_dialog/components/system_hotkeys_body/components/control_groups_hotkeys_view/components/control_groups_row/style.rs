use tw_macro::tw;
classes! {
    base: tw![
        "grid",
        "grid-cols-[repeat(10,11rem)]",
        "gap-[0.8rem]",
    ],
    mobile: tw![
        "mobile:grid-cols-[repeat(5,minmax(0,1fr))]",
        "mobile:auto-rows-[minmax(72px,auto)]",
        "mobile:gap-[0.4rem]",
        "mobile:w-full",
    ],
    tablet: tw![
        "tablet:grid-cols-[repeat(5,minmax(0,1fr))]",
        "tablet:auto-rows-[minmax(72px,auto)]",
        "tablet:gap-[0.4rem]",
        "tablet:w-full",
    ],
}
