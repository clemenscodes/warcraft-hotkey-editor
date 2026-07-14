use tw_macro::tw;

classes! {
    base: tw![
        "@container",
        "hidden",
        "items-center",
        "justify-center",
        "h-full",
        "aspect-39/10",
    ],
    laptop: tw![
        "laptop:flex",
    ],
    desktop: tw![
        "desktop:flex",
    ],
    qhd: tw![
        "qhd:flex",
    ],
    uhd: tw![
        "uhd:flex",
    ],
}
