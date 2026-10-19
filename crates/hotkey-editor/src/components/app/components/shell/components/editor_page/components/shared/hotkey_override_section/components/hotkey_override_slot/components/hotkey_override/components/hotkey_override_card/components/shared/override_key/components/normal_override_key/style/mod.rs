use tw_macro::tw;
classes! {
    base: tw![
        "group/editable-keycap",
        "[--keycap-radius:var(--radius-tile)]",
        "flex",
        "items-center",
        "justify-center",
        "h-full",
        "aspect-square",
        "p-0",
        "text-2xl",
        "cursor-pointer",
        "kb-focus:outline-none",
        "kb-focus:shadow-glow-soft",
    ],
    mobile: tw![
        "mobile:text-[1.5em]",
    ],
    tablet: tw![
        "tablet:text-[1.5em]",
    ],
    laptop: tw![
        "laptop:text-[1.5em]",
    ],
    desktop: tw![
        "desktop:text-[1.5em]",
    ],
    qhd: tw![
        "qhd:text-[1.5em]",
    ],
    uhd: tw![
        "uhd:text-[1.5em]",
    ],
}
