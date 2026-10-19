use tw_macro::tw;
classes! {
    base: tw![
        "@container",
        "flex",
        "flex-col",
        "items-stretch",
        "px-0",
        "pt-0",
        "flex-none",
        "min-h-0",
        "gap-5",
    ],
    mobile: tw![
        "mobile:grid",
        "mobile:grid-cols-[minmax(0,1fr)]",
        "mobile:gap-6",
        "mobile:items-start",
    ],
    tablet: tw![
        "tablet:grid",
        "tablet:grid-cols-[minmax(0,1fr)]",
        "tablet:gap-6",
        "tablet:items-start",
    ],
    laptop: tw![
        "laptop:grid",
        "laptop:grid-cols-[minmax(0,1fr)]",
        "laptop:gap-6",
        "laptop:items-start",
    ],
    desktop: tw![
        "desktop:grid",
        "desktop:grid-cols-[minmax(0,1fr)]",
        "desktop:grid-rows-[minmax(0,1fr)_auto]",
        "desktop:flex-[1_1_0]",
        "desktop:*:first:size-full",
    ],
    qhd: tw![
        "qhd:grid",
        "qhd:grid-cols-[minmax(0,1fr)]",
        "qhd:grid-rows-[minmax(0,1fr)_auto]",
        "qhd:flex-[1_1_0]",
        "qhd:*:first:size-full",
    ],
    uhd: tw![
        "uhd:grid",
        "uhd:grid-cols-[minmax(0,1fr)]",
        "uhd:grid-rows-[minmax(0,1fr)_auto]",
        "uhd:flex-[1_1_0]",
        "uhd:*:first:size-full",
    ],
}
