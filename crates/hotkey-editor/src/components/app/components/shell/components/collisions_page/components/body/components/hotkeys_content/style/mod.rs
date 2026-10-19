use tw_macro::tw;
classes! {
    base: tw![
        "grid",
        "grid-cols-[34rem_minmax(0,1fr)]",
        "gap-10",
        "items-stretch",
        "flex-[1_1_0]",
        "min-h-0",
    ],
    mobile: tw![
        "mobile:grid-cols-[1fr]",
        "mobile:flex-none",
        "mobile:min-h-auto",
    ],
    tablet: tw![
        "tablet:grid-cols-[1fr]",
        "tablet:flex-none",
        "tablet:min-h-auto",
    ],
    laptop: tw![
        "laptop:grid-cols-[1fr]",
        "laptop:flex-none",
        "laptop:min-h-auto",
    ],
    desktop: tw![
        "desktop:contents",
        "desktop:*:first:col-start-1",
        "desktop:*:first:row-start-1",
        "desktop:*:first:row-span-2",
        "desktop:*:last:col-start-2",
        "desktop:*:last:row-start-2",
    ],
    qhd: tw![
        "qhd:contents",
        "qhd:*:first:col-start-1",
        "qhd:*:first:row-start-1",
        "qhd:*:first:row-span-2",
        "qhd:*:last:col-start-2",
        "qhd:*:last:row-start-2",
    ],
    uhd: tw![
        "uhd:contents",
        "uhd:*:first:col-start-1",
        "uhd:*:first:row-start-1",
        "uhd:*:first:row-span-2",
        "uhd:*:last:col-start-2",
        "uhd:*:last:row-start-2",
    ],
}
