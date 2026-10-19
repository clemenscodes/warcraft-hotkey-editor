use tw_macro::tw;
// A template card is a whole dialog wide on a laptop, so its grid headings
// take the larger step that matches its tiles.
classes! {
    base: tw![
        "text-md",
        "uppercase",
        "tracking-heading",
        "font-normal",
        "text-(--race-color,var(--color-warcraft-gold))",
        "text-shadow-drop",
    ],
    mobile: tw![
        "mobile:text-sm",
    ],
    tablet: tw![
        "tablet:text-sm",
    ],
    laptop: tw![
        "laptop:text-lg",
    ],
    desktop: tw![
        "desktop:text-lg",
    ],
    qhd: tw![
        "qhd:text-2xl",
    ],
    uhd: tw![
        "uhd:text-3xl",
    ],
}
