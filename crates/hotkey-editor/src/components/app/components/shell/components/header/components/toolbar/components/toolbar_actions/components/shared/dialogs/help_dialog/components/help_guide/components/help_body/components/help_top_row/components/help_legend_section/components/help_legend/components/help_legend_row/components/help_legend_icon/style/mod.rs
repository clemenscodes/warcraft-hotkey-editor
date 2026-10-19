use tw_macro::tw;
classes! {
    base: tw![
        "inline-flex",
        "items-center",
        "justify-center",
        "shrink-0",
        "size-[3em]",
        "border",
        "border-warcraft-gold/40",
        "rounded-card",
        "bg-panel-gold-resting",
        "text-warcraft-gold",
        "[&_svg]:size-[2em]",
    ],
    mobile: tw![
        "mobile:size-[1.8em]",
        "mobile:[&_svg]:size-[1.15em]",
    ],
    tablet: tw![
        "tablet:size-[1.8em]",
        "tablet:[&_svg]:size-[1.15em]",
    ],
    laptop: tw![
        "laptop:size-[1.8em]",
        "laptop:[&_svg]:size-[1.15em]",
    ],
    desktop: tw![
        "desktop:size-[1.8em]",
        "desktop:[&_svg]:size-[1.15em]",
    ],
    qhd: tw![
        "qhd:size-[1.8em]",
        "qhd:[&_svg]:size-[1.15em]",
    ],
    uhd: tw![
        "uhd:size-[1.8em]",
        "uhd:[&_svg]:size-[1.15em]",
    ],
}
