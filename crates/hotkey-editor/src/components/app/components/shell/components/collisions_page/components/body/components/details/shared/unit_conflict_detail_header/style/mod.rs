use tw_macro::tw;
classes! {
    base: tw![
        "flex",
        "items-center",
        "gap-4",
        "flex-none",
        "h-20",
        "pb-3.5",
        "border-b",
        "border-b-warcraft-gold/25",
    ],
    mobile: tw![
        "mobile:gap-[1em]",
        "mobile:h-[5em]",
        "mobile:pb-[0.875em]",
    ],
    tablet: tw![
        "tablet:gap-[1em]",
        "tablet:h-[5em]",
        "tablet:pb-[0.875em]",
    ],
    laptop: tw![
        "laptop:gap-[1em]",
        "laptop:h-[5em]",
        "laptop:pb-[0.875em]",
    ],
}
