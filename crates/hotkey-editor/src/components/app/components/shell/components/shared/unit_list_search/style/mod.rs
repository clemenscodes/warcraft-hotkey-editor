use tw_macro::tw;

classes! {
    base: tw![
        "relative",
        "flex",
        "items-center",
        "gap-2",
        "min-w-0",
        "flex-none",
        "h-11",
        "rounded-panel",
        "border",
        "border-warcraft-gold/45",
        "bg-warcraft-bg-base",
        "shadow-bevel",
        "has-[:focus-visible]:border-warcraft-gold",
        "has-[:focus-visible]:shadow-ring",
    ],
    desktop: tw![
        "desktop:gap-2.5",
        "desktop:h-13.25",
    ],
    qhd: tw![
        "qhd:gap-3.5",
        "qhd:h-19",
    ],
    uhd: tw![
        "uhd:gap-3.75",
        "uhd:h-20.5",
    ],
}
