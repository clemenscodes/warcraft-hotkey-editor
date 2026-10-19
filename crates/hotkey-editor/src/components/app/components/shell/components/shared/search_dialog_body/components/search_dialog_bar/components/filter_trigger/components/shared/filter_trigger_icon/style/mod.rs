use tw_macro::tw;

classes! {
    base: tw![
        "block",
        "shrink-0",
        "w-4.5",
        "h-4.5",
        "pointer-events-none",
        "[&_svg]:block",
        "[&_svg]:w-full",
        "[&_svg]:h-full",
    ],
    desktop: tw![
        "desktop:w-5.5",
        "desktop:h-5.5",
    ],
    qhd: tw![
        "qhd:w-7.75",
        "qhd:h-7.75",
    ],
    uhd: tw![
        "uhd:w-8.5",
        "uhd:h-8.5",
    ],
}
