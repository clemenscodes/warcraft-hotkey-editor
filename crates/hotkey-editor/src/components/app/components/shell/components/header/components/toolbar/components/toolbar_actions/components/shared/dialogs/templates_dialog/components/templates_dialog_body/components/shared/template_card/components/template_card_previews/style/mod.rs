use tw_macro::tw;
classes! {
    base: tw![
        "flex",
        "flex-row",
        "flex-nowrap",
        "items-start",
        "gap-8",
    ],
    mobile: tw![
        "mobile:items-stretch",
        "mobile:gap-3",
        "mobile:overflow-x-auto",
        "mobile:overflow-y-hidden",
        "mobile:overscroll-x-contain",
        "mobile:touch-pan-x",
        "mobile:touch-pan-y",
        "mobile:snap-x",
        "mobile:snap-mandatory",
        "mobile:scrollbar-none",
        "mobile:[&::-webkit-scrollbar]:hidden",
        "mobile:*:w-full",
        "mobile:*:shrink-0",
        "mobile:*:basis-full",
        "mobile:*:snap-start",
        "mobile:*:snap-always",
    ],
    tablet: tw![
        "tablet:justify-center",
        "tablet:gap-2",
    ],
    laptop: tw![
        "laptop:justify-center",
        "laptop:gap-6",
    ],
    desktop: tw![
        "desktop:justify-center",
        "desktop:gap-6",
    ],
    qhd: tw![
        "qhd:justify-center",
        "qhd:gap-8.5",
    ],
    uhd: tw![
        "uhd:justify-center",
        "uhd:gap-10.25",
    ],
}
