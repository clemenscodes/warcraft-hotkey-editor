use tw_macro::tw;
classes! {
    base: tw![
        "flex",
        "flex-col",
        "gap-2",
        "self-stretch",
        "flex-[0_0_34rem]",
        "w-136",
    ],
    mobile: tw![
        "mobile:flex-row",
        "mobile:flex-none",
        "mobile:w-full",
        "mobile:gap-2",
    ],
    tablet: tw![
        "tablet:flex-[0_0_18rem]",
        "tablet:w-72",
    ],
    qhd: tw![
        "qhd:flex-[0_0_46rem]",
        "qhd:w-184",
    ],
    uhd: tw![
        "uhd:flex-[0_0_62rem]",
        "uhd:w-248",
    ],
}
