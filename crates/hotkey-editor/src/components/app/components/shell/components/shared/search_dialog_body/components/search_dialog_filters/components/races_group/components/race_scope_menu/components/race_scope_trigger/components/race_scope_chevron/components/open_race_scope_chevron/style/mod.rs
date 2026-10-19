use tw_macro::tw;

classes! {
    base: tw![
        "inline-flex",
        "[&_svg]:size-[1em]",
        "shrink-0",
        "text-warcraft-text-muted",
        "text-sm",
    ],
    desktop: tw![
        "desktop:text-md",
    ],
    qhd: tw![
        "qhd:text-lg",
    ],
    uhd: tw![
        "uhd:text-lg",
    ],
}
