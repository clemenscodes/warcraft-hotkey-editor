use tw_macro::tw;
classes! {
    base: tw![
        "grid",
        "grid-cols-[repeat(3,26rem)]",
        "gap-[1.5rem]",
    ],
    mobile: tw![
        "mobile:grid-cols-[repeat(3,minmax(0,1fr))]",
        "mobile:gap-[0.5rem]",
        "mobile:w-full",
        "mobile:max-w-[30rem]",
        "mobile:mx-auto",
    ],
    tablet: tw![
        "tablet:grid-cols-[repeat(3,minmax(0,1fr))]",
        "tablet:gap-[0.5rem]",
        "tablet:w-full",
        "tablet:max-w-[30rem]",
        "tablet:mx-auto",
    ],
}
