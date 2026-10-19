use tw_macro::tw;

classes! {
    base: tw![
        "flex",
        "flex-col",
        "flex-1",
        "w-full",
        "min-w-0",
        "p-4",
        "border",
        "border-warcraft-blue-deep",
        "rounded-card",
        "bg-panel-dark",
        "shadow-bevel",
    ],
    mobile: tw![
        "mobile:text-[4.47cqi]",
        "mobile:p-[1em]",
    ],
    tablet: tw![
        "tablet:text-[1.65cqi]",
        "tablet:p-[1em]",
    ],
    laptop: tw![
        "laptop:text-[1.14cqi]",
        "laptop:p-[1em]",
    ],
}
