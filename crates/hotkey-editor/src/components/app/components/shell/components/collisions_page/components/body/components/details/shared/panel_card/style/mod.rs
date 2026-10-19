use tw_macro::tw;

classes! {
    base: tw![
        "@container",
        "flex",
        "flex-col",
        "py-6",
        "px-4",
        "min-w-0",
        "border",
        "border-warcraft-blue-deep",
        "rounded-panel",
        "bg-warcraft-bg-mid/45",
    ],
    mobile: tw![
        "mobile:px-[0.5em]",
        "mobile:py-[1.5em]",
    ],
    tablet: tw![
        "tablet:px-[0.5em]",
        "tablet:py-[1.5em]",
    ],
    laptop: tw![
        "laptop:px-[0.5em]",
        "laptop:py-[1.5em]",
    ],
}
