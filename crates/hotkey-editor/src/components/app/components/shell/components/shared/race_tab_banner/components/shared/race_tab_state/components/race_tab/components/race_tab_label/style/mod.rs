use tw_macro::tw;

classes! {
    base: tw![
        "relative",
        "z-1",
        "w-full",
        "min-w-0",
        "whitespace-nowrap",
        "text-center",
        "text-(--label-color,var(--color-white))",
        "transition-colors",
        "duration-base",
    ],
    mobile: tw![
        "mobile:whitespace-normal",
        "mobile:leading-flush",
        "mobile:tracking-snug",
    ],
    tablet: tw![
        "tablet:whitespace-normal",
        "tablet:leading-flush",
        "tablet:tracking-snug",
    ],
    laptop: tw![
        "laptop:whitespace-normal",
        "laptop:leading-flush",
        "laptop:tracking-snug",
    ],
}
