use tw_macro::tw;
classes! {
    base: tw![
        "@container",
        "flex",
        "flex-col",
        "gap-6",
        "flex-1",
        "min-w-0",
    ],
    mobile: tw![
        "mobile:grid",
        "mobile:grid-cols-1",
        "mobile:gap-x-[2em]",
        "mobile:gap-y-[1em]",
        "mobile:content-start",
    ],
    tablet: tw![
        "tablet:grid",
        "tablet:grid-cols-2",
        "tablet:gap-x-[2em]",
        "tablet:gap-y-[1em]",
        "tablet:content-start",
    ],
    laptop: tw![
        "laptop:grid",
        "laptop:grid-cols-2",
        "laptop:gap-x-[2em]",
        "laptop:gap-y-[1em]",
        "laptop:content-start",
    ],
    desktop: tw![
        "desktop:grid",
        "desktop:grid-cols-2",
        "desktop:gap-x-[2em]",
        "desktop:gap-y-[1em]",
        "desktop:content-start",
    ],
    qhd: tw![
        "qhd:grid",
        "qhd:grid-cols-2",
        "qhd:gap-x-[2em]",
        "qhd:gap-y-[1em]",
        "qhd:content-start",
    ],
    uhd: tw![
        "uhd:grid",
        "uhd:grid-cols-2",
        "uhd:gap-x-[2em]",
        "uhd:gap-y-[1em]",
        "uhd:content-start",
    ],
}
