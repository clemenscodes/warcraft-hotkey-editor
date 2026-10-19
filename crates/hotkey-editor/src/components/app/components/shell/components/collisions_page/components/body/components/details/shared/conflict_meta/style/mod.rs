use tw_macro::tw;

classes! {
    base: tw![
        "flex",
        "flex-col",
        "gap-0",
        "min-w-0",
        "flex-auto",
        "py-0",
        "px-2",
    ],
    mobile: tw![
        "mobile:px-[0.5em]",
    ],
    tablet: tw![
        "tablet:px-[0.5em]",
    ],
    laptop: tw![
        "laptop:px-[0.5em]",
    ],
}
