use tw_macro::tw;
classes! {
    base: tw![
        "max-w-360",
        "text-center",
        "uppercase",
        "tracking-heading",
        "text-2xl",
        "leading-body",
        "text-warcraft-gold/75",
        "text-shadow-drop",
    ],
    mobile: tw![
        "mobile:text-sm",
        "mobile:normal-case",
        "mobile:tracking-normal",
    ],
    tablet: tw![
        "tablet:text-md",
    ],
    laptop: tw![
        "laptop:text-lg",
    ],
    desktop: tw![
        "desktop:text-lg",
    ],
    qhd: tw![
        "qhd:max-w-504",
        "qhd:text-2xl",
    ],
    uhd: tw![
        "uhd:max-w-612",
        "uhd:text-3xl",
    ],
}
