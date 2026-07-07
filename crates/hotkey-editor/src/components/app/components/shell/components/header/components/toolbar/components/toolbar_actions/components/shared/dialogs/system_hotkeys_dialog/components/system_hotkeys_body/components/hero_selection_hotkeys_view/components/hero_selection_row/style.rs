use tw_macro::tw;
classes! {
    base: tw![
        "grid",
        "grid-cols-[repeat(3,26rem)]",
        "gap-6",
    ],
    mobile: tw![
        "mobile:grid-cols-[repeat(3,minmax(0,1fr))]",
        "mobile:gap-2",
        "mobile:w-full",
        "mobile:max-w-[30rem]",
        "mobile:mx-auto",
    ],
    tablet: tw![
        "tablet:grid-cols-[repeat(3,minmax(0,1fr))]",
        "tablet:gap-2",
        "tablet:w-full",
        "tablet:max-w-[30rem]",
        "tablet:mx-auto",
    ],
}
