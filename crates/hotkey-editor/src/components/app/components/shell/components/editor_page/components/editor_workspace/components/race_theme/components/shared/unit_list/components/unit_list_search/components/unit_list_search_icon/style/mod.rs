use tw_macro::tw;

classes! {
    base: tw![
        "hidden",
        "text-warcraft-gold",
        "pointer-events-none",
        "[&_svg]:block",
        "[&_svg]:w-full",
        "[&_svg]:h-full",
    ],
    mobile: tw![
        "mobile:block",
        "mobile:absolute",
        "mobile:left-3",
        "mobile:top-1/2",
        "mobile:-translate-y-1/2",
        "mobile:w-4.5",
        "mobile:h-4.5",
    ],
    tablet: tw![
        "tablet:block",
        "tablet:absolute",
        "tablet:left-3",
        "tablet:top-1/2",
        "tablet:-translate-y-1/2",
        "tablet:w-4.5",
        "tablet:h-4.5",
    ],
}
