use tw_macro::tw;

// From a desktop up the main keys and the numpad sit side by side, so every key
// is a smaller share of the board's width than the base's and both fit.
classes! {
    base: tw![
        "flex",
        "flex-row",
        "items-end",
        "justify-center",
        "gap-3",
        "w-full",
        "@container",
        "focus:outline-none",
    ],
    mobile: tw![
        "mobile:flex-col",
        "mobile:items-stretch",
        "mobile:gap-1.5",
    ],
    tablet: tw![
        "tablet:flex-col",
        "tablet:items-stretch",
        "tablet:gap-1.5",
    ],
    laptop: tw![
        "laptop:flex-col",
        "laptop:items-stretch",
        "laptop:gap-1.5",
    ],
}
