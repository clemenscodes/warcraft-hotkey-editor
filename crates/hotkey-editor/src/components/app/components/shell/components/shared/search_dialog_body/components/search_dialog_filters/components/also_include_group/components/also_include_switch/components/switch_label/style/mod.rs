use tw_macro::tw;

classes! {
    base: tw![
        "inline-flex",
        "items-center",
        "gap-1.5",
        "text-warcraft-text-primary",
        "text-sm",
    ],
    desktop: tw![
        "desktop:gap-1.75",
        "desktop:text-md",
    ],
    qhd: tw![
        "qhd:gap-2.5",
        "qhd:text-lg",
    ],
    uhd: tw![
        "uhd:gap-2.75",
        "uhd:text-lg",
    ],
}
