use tw_macro::tw;
classes! {
    base: tw![
        "relative",
        "flex",
        "flex-col",
        "shrink-0",
        "h-full",
        "min-w-0",
    ],
    laptop: tw![
        "laptop:grid",
        "laptop:h-auto",
        "laptop:grid-cols-2",
        "laptop:content-start",
        "laptop:gap-x-4",
    ],
    desktop: tw![
        "desktop:grid",
        "desktop:h-auto",
        "desktop:grid-cols-2",
        "desktop:content-start",
        "desktop:gap-x-4",
    ],
    qhd: tw![
        "qhd:grid",
        "qhd:h-auto",
        "qhd:grid-cols-3",
        "qhd:content-start",
        "qhd:gap-x-4",
    ],
    uhd: tw![
        "uhd:grid",
        "uhd:h-auto",
        "uhd:grid-cols-3",
        "uhd:content-start",
        "uhd:gap-x-4",
    ],
}
