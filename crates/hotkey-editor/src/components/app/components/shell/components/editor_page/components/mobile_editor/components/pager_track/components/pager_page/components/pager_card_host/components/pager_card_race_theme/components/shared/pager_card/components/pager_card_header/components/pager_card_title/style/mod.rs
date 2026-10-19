use tw_macro::tw;
classes! {
    base: tw![
        "@container",
        "flex",
        "flex-col",
        "flex-1",
        "min-w-0",
        "gap-[1cqi]",
    ],
    tablet: tw![
        "tablet:gap-[0.5cqi]",
    ],
    laptop: tw![
        "laptop:gap-[0.5cqi]",
    ],
}
