use tw_macro::tw;
classes! {
    base: tw![
        "@container",
        "flex",
        "flex-col",
        "items-center",
        "gap-[1.92cqi]",
        "flex-none",
        "min-w-28",
    ],
    mobile: tw![
        "mobile:min-w-0",
    ],
    tablet: tw![
        "tablet:min-w-0",
    ],
    laptop: tw![
        "laptop:min-w-0",
    ],
}
