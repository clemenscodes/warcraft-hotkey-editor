use tw_macro::tw;

classes! {
    base: tw![
        "max-w-full",
        "overflow-hidden",
        "whitespace-nowrap",
        "text-ellipsis",
        "font-normal",
        "text-2xl",
        "leading-title",
        "text-(--race-color,var(--color-warcraft-gold))",
        "text-shadow-drop",
    ],
    mobile: tw![
        "mobile:text-[1.25em]",
        "mobile:break-normal",
    ],
    tablet: tw![
        "tablet:text-[1.25em]",
        "tablet:break-normal",
    ],
    laptop: tw![
        "laptop:text-[1.25em]",
        "laptop:break-normal",
    ],
    desktop: tw![
        "desktop:text-[1.25em]",
        "desktop:break-normal",
    ],
    qhd: tw![
        "qhd:text-[1.25em]",
        "qhd:break-normal",
    ],
    uhd: tw![
        "uhd:text-[1.25em]",
        "uhd:break-normal",
    ],
}
