use tw_macro::tw;

classes! {
    base: tw![
        "relative",
        "group/tooltip",
        "[anchor-name:--tooltip-anchor]",
        "[anchor-scope:--tooltip-anchor]",
        "cursor-pointer",
        "touch-none",
        "focus:outline-none",
        "kb-focus:outline-none",
    ],
    mobile: tw![
        "mobile:aspect-[1/0.85]",
        "mobile:min-h-0",
    ],
    tablet: tw![
        "tablet:aspect-[1/0.85]",
        "tablet:min-h-0",
    ],
}
