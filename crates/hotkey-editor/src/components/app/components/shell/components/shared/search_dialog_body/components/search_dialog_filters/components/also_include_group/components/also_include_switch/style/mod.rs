use tw_macro::tw;

classes! {
    base: tw![
        "relative",
        "flex",
        "items-center",
        "justify-between",
        "gap-3",
        "py-2",
        "px-1",
        "text-warcraft-text-primary",
        "text-sm",
    ],
    desktop: tw![
        "desktop:gap-3.5",
        "desktop:py-2.5",
        "desktop:px-1.25",
        "desktop:text-md",
    ],
    qhd: tw![
        "qhd:gap-5.25",
        "qhd:py-3.5",
        "qhd:px-1.75",
        "qhd:text-lg",
    ],
    uhd: tw![
        "uhd:gap-5.5",
        "uhd:py-3.75",
        "uhd:px-1.75",
        "uhd:text-lg",
    ],
}
