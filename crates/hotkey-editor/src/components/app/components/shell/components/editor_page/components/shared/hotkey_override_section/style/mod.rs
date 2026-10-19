use tw_macro::tw;

classes! {
    base: tw![
        "flex",
        "flex-col",
        "min-w-0",
        "self-start",
    ],
    mobile: tw![
        "mobile:w-full",
        "mobile:self-stretch",
        "mobile:items-center",
        "mobile:*:first:pb-[2cqi]",
    ],
    tablet: tw![
        "tablet:w-full",
        "tablet:self-stretch",
        "tablet:items-center",
        "tablet:*:first:pb-[1cqi]",
    ],
    laptop: tw![
        "laptop:w-full",
        "laptop:self-stretch",
        "laptop:items-center",
        "laptop:*:first:pb-[1cqi]",
    ],
    desktop: tw![
        "desktop:w-full",
        "desktop:self-stretch",
        "desktop:items-center",
        "desktop:*:first:pb-[1cqi]",
    ],
    qhd: tw![
        "qhd:w-full",
        "qhd:self-stretch",
        "qhd:items-center",
        "qhd:*:first:pb-[1cqi]",
    ],
    uhd: tw![
        "uhd:w-full",
        "uhd:self-stretch",
        "uhd:items-center",
        "uhd:*:first:pb-[1cqi]",
    ],
}
