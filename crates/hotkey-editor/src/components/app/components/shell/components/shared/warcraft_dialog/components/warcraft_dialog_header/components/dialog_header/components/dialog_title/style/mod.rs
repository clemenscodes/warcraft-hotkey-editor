use tw_macro::tw;
classes! {
    base: tw![
        "min-w-0",
        "text-3xl",
        "leading-none",
        "whitespace-nowrap",
        "uppercase",
        "tracking-heading",
        "text-warcraft-gold",
        "text-glow",
    ],
    // The token is the largest the title is drawn. A title wider than the room
    // between the flourishes shrinks below it until it fits on one line (see
    // `TitleFit`), so a short name is drawn large and a long one still fits,
    // with no clip, no ellipsis and no wrap.
    mobile: tw![
        "mobile:text-2xl",
    ],
    tablet: tw![
        "tablet:text-2xl",
    ],
    laptop: tw![
        "laptop:text-2xl",
    ],
    desktop: tw![
        "desktop:text-2xl",
    ],
    qhd: tw![
        "qhd:text-3xl",
    ],
    uhd: tw![
        "uhd:text-4xl",
    ],
}
