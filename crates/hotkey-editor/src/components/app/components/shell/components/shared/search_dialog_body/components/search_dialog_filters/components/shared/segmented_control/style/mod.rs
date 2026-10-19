use tw_macro::tw;

classes! {
    base: tw![
        "flex",
        "gap-1",
        "p-0.75",
        "border",
        "border-warcraft-gold/35",
        "rounded-panel",
        "bg-warcraft-bg-base",
        "*:flex-1",
    ],
    desktop: tw![
        "desktop:gap-1.25",
        "desktop:p-1",
    ],
    qhd: tw![
        "qhd:gap-1.75",
        "qhd:p-1.25",
    ],
    uhd: tw![
        "uhd:gap-1.75",
        "uhd:p-1.5",
    ],
}
