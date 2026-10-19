use tw_macro::tw;
classes! {
    base: tw![
        "flex",
        "flex-col",
        "items-center",
        "justify-center",
        "gap-2",
        "px-2.5",
        "py-3.5",
        "cursor-default",
        "border-2",
        "border-dashed",
        "border-warcraft-gold/18",
        "text-3xl",
        "text-warcraft-gold/25",
        "bg-warcraft-bg-base/50",
    ],
    mobile: tw![
        "mobile:aspect-[1/0.85]",
        "mobile:text-sm",
    ],
    tablet: tw![
        "tablet:aspect-[1/0.85]",
        "tablet:text-base",
    ],
    laptop: tw![
        "laptop:aspect-[1/0.85]",
        "laptop:text-md",
    ],
    desktop: tw![
        "desktop:aspect-[1/0.85]",
        "desktop:text-md",
    ],
    qhd: tw![
        "qhd:gap-2.75",
        "qhd:px-3.5",
        "qhd:py-5",
        "qhd:aspect-[1/0.85]",
        "qhd:text-xl",
    ],
    uhd: tw![
        "uhd:gap-3.5",
        "uhd:px-4.25",
        "uhd:py-6",
        "uhd:aspect-[1/0.85]",
        "uhd:text-2xl",
    ],
}
