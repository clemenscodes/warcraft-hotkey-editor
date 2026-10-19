use tw_macro::tw;
classes! {
    base: tw![
        "@container",
        "grid",
        "grid-cols-2",
        "gap-9",
        "w-full",
    ],
    mobile: tw![
        "mobile:grid-cols-1",
        "mobile:gap-2.5",
    ],
    tablet: tw![
        "tablet:grid-cols-1",
        "tablet:gap-2.5",
    ],
    laptop: tw![
        "laptop:grid-cols-1",
        "laptop:gap-2.5",
    ],
    desktop: tw![
        "desktop:grid-cols-2",
        "desktop:gap-2.5",
    ],
    qhd: tw![
        "qhd:grid-cols-2",
        "qhd:gap-3.5",
    ],
    uhd: tw![
        "uhd:grid-cols-2",
        "uhd:gap-4.25",
    ],
}
