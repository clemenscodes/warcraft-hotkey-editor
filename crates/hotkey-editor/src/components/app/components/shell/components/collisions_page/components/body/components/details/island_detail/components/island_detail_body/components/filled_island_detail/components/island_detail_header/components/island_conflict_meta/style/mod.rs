use tw_macro::tw;

classes! {
    base: tw![
        "flex",
        "flex-col",
        "gap-2",
        "min-w-0",
        "flex-auto",
        "py-1.5",
        "px-2",
    ],
    mobile: tw![
        "mobile:gap-[0.5em]",
        "mobile:py-[0.375em]",
        "mobile:px-[0.5em]",
    ],
    tablet: tw![
        "tablet:gap-[0.5em]",
        "tablet:py-[0.375em]",
        "tablet:px-[0.5em]",
    ],
    laptop: tw![
        "laptop:gap-[0.5em]",
        "laptop:py-[0.375em]",
        "laptop:px-[0.5em]",
    ],
}
