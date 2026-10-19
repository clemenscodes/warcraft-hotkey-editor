use tw_macro::tw;
classes! {
    base: tw![
        "inline-flex",
        "items-center",
        "justify-center",
        "size-[1.7em]",
        "align-[-0.3em]",
        "text-warcraft-gold",
        "[&_svg]:w-full",
        "[&_svg]:h-full",
    ],
    mobile: tw![
        "mobile:size-[1.15em]",
        "mobile:align-[-0.2em]",
    ],
    tablet: tw![
        "tablet:size-[1.15em]",
        "tablet:align-[-0.2em]",
    ],
    laptop: tw![
        "laptop:size-[1.15em]",
        "laptop:align-[-0.2em]",
    ],
    desktop: tw![
        "desktop:size-[1.15em]",
        "desktop:align-[-0.2em]",
    ],
    qhd: tw![
        "qhd:size-[1.15em]",
        "qhd:align-[-0.2em]",
    ],
    uhd: tw![
        "uhd:size-[1.15em]",
        "uhd:align-[-0.2em]",
    ],
}
