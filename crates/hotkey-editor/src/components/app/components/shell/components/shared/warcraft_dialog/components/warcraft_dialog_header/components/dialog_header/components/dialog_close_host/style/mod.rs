use tw_macro::tw;
classes! {
    base: tw![
        "@container",
        "absolute",
        "right-4",
        "top-1/2",
        "-translate-y-1/2",
        "w-10",
        "h-10",
    ],
    mobile: tw![
        "mobile:right-2",
    ],
    tablet: tw![
        "tablet:right-2",
    ],
    laptop: tw![
        "laptop:right-2",
    ],
    desktop: tw![
        "desktop:right-2",
    ],
    qhd: tw![
        "qhd:w-14",
        "qhd:h-14",
        "qhd:right-2.75",
    ],
    uhd: tw![
        "uhd:w-17",
        "uhd:h-17",
        "uhd:right-3.5",
    ],
}
