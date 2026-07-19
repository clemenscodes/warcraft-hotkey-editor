use tw_macro::tw;
classes! {
    base: tw![
        "text-md",
        "uppercase",
        "tracking-heading",
        "font-normal",
        "text-(--race-color,var(--color-warcraft-gold))",
        "text-shadow-drop",
    ],
    mobile: tw![
        "mobile:text-[3cqi]",
    ],
    tablet: tw![
        "tablet:text-md",
    ],
    desktop: tw![
        "desktop:text-lg",
    ],
    qhd: tw![
        "qhd:text-xl",
    ],
    uhd: tw![
        "uhd:text-2xl",
    ],
}
