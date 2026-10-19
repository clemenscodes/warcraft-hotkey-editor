use tw_macro::tw;
classes! {
    base: tw![
        "@container",
        "grid",
        "grid-cols-[repeat(4,9rem)]",
        "auto-rows-36",
        "gap-5",
    ],
    mobile: tw![
        "mobile:w-full",
        "mobile:grid-cols-4",
        "mobile:auto-rows-auto",
        "mobile:gap-2",
    ],
    tablet: tw![
        "tablet:w-full",
        "tablet:grid-cols-4",
        "tablet:auto-rows-auto",
        "tablet:gap-2",
    ],
    laptop: tw![
        "laptop:w-full",
        "laptop:grid-cols-4",
        "laptop:auto-rows-auto",
        "laptop:gap-2",
    ],
    desktop: tw![
        "desktop:w-full",
        "desktop:grid-cols-4",
        "desktop:auto-rows-auto",
        "desktop:gap-2",
    ],
    qhd: tw![
        "qhd:w-full",
        "qhd:grid-cols-4",
        "qhd:auto-rows-auto",
        "qhd:gap-2.75",
    ],
    uhd: tw![
        "uhd:w-full",
        "uhd:grid-cols-4",
        "uhd:auto-rows-auto",
        "uhd:gap-3.5",
    ],
}
