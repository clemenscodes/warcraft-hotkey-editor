use tw_macro::tw;

classes! {
    base: tw![
        "@container",
        "relative",
        "grid",
        "grid-cols-[minmax(0,1fr)]",
        "flex-[1_1_0]",
        "items-stretch",
        "min-h-0",
        "gap-4",
    ],
    mobile: tw![
        "mobile:flex-none",
    ],
    tablet: tw![
        "tablet:flex-none",
    ],
    laptop: tw![
        "laptop:grid-cols-[34rem_minmax(0,1fr)]",
        "laptop:grid-rows-[1fr]",
        "laptop:gap-10",
        "laptop:overflow-hidden",
    ],
    desktop: tw![
        "desktop:grid-cols-[34rem_minmax(0,1fr)]",
        "desktop:grid-rows-[1fr]",
        "desktop:gap-10",
        "desktop:overflow-hidden",
    ],
    qhd: tw![
        "qhd:grid-cols-[46rem_minmax(0,1fr)]",
        "qhd:grid-rows-[1fr]",
        "qhd:gap-10",
        "qhd:overflow-hidden",
    ],
    uhd: tw![
        "uhd:grid-cols-[62rem_minmax(0,1fr)]",
        "uhd:grid-rows-[1fr]",
        "uhd:gap-10",
        "uhd:overflow-hidden",
    ],
}
