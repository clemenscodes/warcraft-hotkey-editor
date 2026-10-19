use tw_macro::tw;

classes! {
    base: tw![
        "@container",
        "flex",
        "flex-col",
        "gap-2",
        "flex-[1_1_0]",
        "min-h-0",
        "min-w-0",
    ],
    mobile: tw![
        "mobile:hidden",
    ],
    tablet: tw![
        "tablet:flex-none",
    ],
    laptop: tw![
        "laptop:flex-none",
    ],
    desktop: tw![
        "desktop:grid",
        "desktop:grid-cols-[43rem_minmax(0,1fr)]",
        "desktop:grid-rows-[auto_minmax(0,1fr)]",
        "desktop:gap-x-5",
        "desktop:gap-y-4",
        "desktop:p-4",
    ],
    qhd: tw![
        "qhd:grid",
        "qhd:grid-cols-[64.5rem_minmax(0,1fr)]",
        "qhd:grid-rows-[auto_minmax(0,1fr)]",
        "qhd:gap-x-5",
        "qhd:gap-y-4",
        "qhd:p-4",
    ],
    uhd: tw![
        "uhd:grid",
        "uhd:grid-cols-[86rem_minmax(0,1fr)]",
        "uhd:grid-rows-[auto_minmax(0,1fr)]",
        "uhd:gap-x-5",
        "uhd:gap-y-4",
        "uhd:p-4",
    ],
}
