use tw_macro::tw;

classes! {
    base: tw![
        "block",
        "shrink-0",
        "w-3.5",
        "h-3.5",
        "pointer-events-none",
        "[&_svg]:block",
        "[&_svg]:w-full",
        "[&_svg]:h-full",
    ],
    desktop: tw![
        "desktop:w-4.25",
        "desktop:h-4.25",
    ],
    qhd: tw![
        "qhd:w-6",
        "qhd:h-6",
    ],
    uhd: tw![
        "uhd:w-6.5",
        "uhd:h-6.5",
    ],
}
