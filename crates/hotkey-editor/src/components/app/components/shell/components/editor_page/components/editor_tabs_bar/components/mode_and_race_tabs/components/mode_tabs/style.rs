use tw_macro::tw;
classes! {
    base: tw![
        "flex",
        "flex-col",
        "gap-2",
        "self-stretch",
        "flex-[0_0_34rem]",
        "w-[34rem]",
    ],
    mobile: tw![
        "mobile:flex-row",
        "mobile:flex-none",
        "mobile:w-full",
        "mobile:gap-[0.5rem]",
    ],
    tablet: tw![
        "tablet:flex-[0_0_18rem]",
        "tablet:w-72",
    ],
    qhd: tw![
        "qhd:flex-[0_0_46rem]",
        "qhd:w-[46rem]",
    ],
    uhd: tw![
        "uhd:flex-[0_0_62rem]",
        "uhd:w-[62rem]",
    ],
}
