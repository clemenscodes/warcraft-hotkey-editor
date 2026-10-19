use tw_macro::tw;

classes! {
    base: tw![
        "absolute",
        "top-full",
        "translate-y-1",
        "left-0",
        "z-20",
        "min-w-full",
        "flex",
        "flex-col",
        "gap-0.5",
        "p-1",
        "bg-warcraft-bg-base",
        "border",
        "border-warcraft-gold/45",
        "rounded-card",
        "shadow-overlay",
    ],
    desktop: tw![
        "desktop:top-full",
        "desktop:translate-y-1.25",
        "desktop:gap-0.5",
        "desktop:p-1.25",
    ],
    qhd: tw![
        "qhd:top-full",
        "qhd:translate-y-1.75",
        "qhd:gap-0.75",
        "qhd:p-1.75",
    ],
    uhd: tw![
        "uhd:top-full",
        "uhd:translate-y-1.75",
        "uhd:gap-1",
        "uhd:p-1.75",
    ],
}
