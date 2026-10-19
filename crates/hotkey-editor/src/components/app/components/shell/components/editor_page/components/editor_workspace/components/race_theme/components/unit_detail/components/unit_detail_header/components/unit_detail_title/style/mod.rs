use tw_macro::tw;
classes! {
    base: tw![
        "@container",
        "flex",
        "flex-col",
        "min-w-0",
        "gap-2",
        "overflow-x-clip",
    ],
    mobile: tw![
        "mobile:flex-1",
        "mobile:items-start",
        "mobile:gap-1",
        "mobile:text-left",
        "mobile:overflow-visible",
    ],
    tablet: tw![
        "tablet:flex-1",
        "tablet:items-start",
        "tablet:gap-1",
        "tablet:text-left",
        "tablet:overflow-visible",
    ],
    laptop: tw![
        "laptop:flex-1",
        "laptop:items-start",
        "laptop:gap-1",
        "laptop:text-left",
        "laptop:overflow-visible",
    ],
}
