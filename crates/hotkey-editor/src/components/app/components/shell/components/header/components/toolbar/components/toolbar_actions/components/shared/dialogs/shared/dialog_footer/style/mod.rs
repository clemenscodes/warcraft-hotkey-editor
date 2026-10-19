use tw_macro::tw;

// The one footer every dialog with an action ends in: its single action
// button, centred under a gold rule.
classes! {
    base: tw![
        "flex",
        "items-center",
        "justify-end",
        "flex-none",
        "gap-4",
        "pt-6",
        "px-18",
        "pb-7",
        "border-t",
        "border-warcraft-gold/40",
    ],
    mobile: tw![
        "mobile:justify-center",
        "mobile:px-6",
    ],
    tablet: tw![
        "tablet:justify-center",
        "tablet:px-6",
    ],
    laptop: tw![
        "laptop:justify-center",
        "laptop:px-6",
    ],
    desktop: tw![
        "desktop:justify-center",
        "desktop:px-6",
    ],
    qhd: tw![
        "qhd:gap-5.5",
        "qhd:pt-8.5",
        "qhd:pb-9.75",
        "qhd:justify-center",
        "qhd:px-8.5",
    ],
    uhd: tw![
        "uhd:gap-6.75",
        "uhd:pt-10.25",
        "uhd:pb-12",
        "uhd:justify-center",
        "uhd:px-10.25",
    ],
}
