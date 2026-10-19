use tw_macro::tw;

classes! {
    base: tw![
        "text-center",
        "max-w-360",
        "uppercase",
        "tracking-heading",
        "leading-heading",
        "text-(--race-color,var(--color-warcraft-gold))/85",
        "text-shadow-drop",
    ],
    mobile: tw![
        "mobile:normal-case",
        "mobile:text-sm",
        "mobile:tracking-normal",
    ],
    tablet: tw![
        "tablet:text-md",
    ],
    laptop: tw![
        "laptop:text-md",
    ],
    desktop: tw![
        "desktop:text-md",
    ],
    qhd: tw![
        "qhd:text-xl",
    ],
    uhd: tw![
        "uhd:text-2xl",
    ],
}
