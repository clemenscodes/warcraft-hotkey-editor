use tw_macro::tw;

classes! {
    base: tw![
        "flex",
        "flex-col",
        "gap-2",
    ],
    mobile: tw![
        "mobile:gap-1.5",
    ],
    tablet: tw![
        "tablet:grid",
        "tablet:grid-cols-2",
        "tablet:content-start",
        "tablet:gap-1.5",
    ],
    laptop: tw![
        "laptop:grid",
        "laptop:grid-cols-3",
        "laptop:content-start",
        "laptop:gap-1.5",
    ],
    desktop: tw![
        "desktop:grid",
        "desktop:grid-cols-2",
        "desktop:content-start",
    ],
    qhd: tw![
        "qhd:grid",
        "qhd:grid-cols-3",
        "qhd:content-start",
    ],
    uhd: tw![
        "uhd:grid",
        "uhd:grid-cols-4",
        "uhd:content-start",
    ],
}
