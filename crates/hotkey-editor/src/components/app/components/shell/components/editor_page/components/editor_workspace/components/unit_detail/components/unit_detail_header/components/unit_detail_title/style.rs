use tw_macro::tw;
classes! {
    base: tw![
        "flex",
        "flex-col",
        "min-w-0",
        "gap-[0.45rem]",
        "overflow-x-clip",
    ],
    mobile: tw![
        "mobile:flex-1",
        "mobile:items-start",
        "mobile:gap-[3px]",
        "mobile:text-left",
        "mobile:overflow-visible",
    ],
    tablet: tw![
        "tablet:flex-1",
        "tablet:items-start",
        "tablet:gap-[3px]",
        "tablet:text-left",
        "tablet:overflow-visible",
    ],
}
