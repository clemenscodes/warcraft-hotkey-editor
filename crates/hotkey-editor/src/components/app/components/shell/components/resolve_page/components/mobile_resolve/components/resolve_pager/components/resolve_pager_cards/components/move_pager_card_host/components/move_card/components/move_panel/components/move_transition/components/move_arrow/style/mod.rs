use tw_macro::tw;
classes! {
    base: tw![
        "inline-flex",
        "[&_svg]:size-[1em]",
        "absolute",
        "left-[50%]",
        "top-[50%]",
        "transform-[translate(-50%,-50%)]",
        "flex-none",
        "text-warcraft-gold",
        "text-3xl",
        "leading-none",
        "text-shadow-drop",
    ],
    // With the grids stacked on a phone the arrow leaves the absolute centre,
    // sits in flow between them and rotates a quarter turn to point down.
    mobile: tw![
        "mobile:static",
        "mobile:transform-[rotate(90deg)]",
        "mobile:text-[2em]",
    ],
    tablet: tw![
        "tablet:text-[2.5em]",
    ],
    laptop: tw![
        "laptop:text-[2.5em]",
    ],
    desktop: tw![
        "desktop:text-[2.5em]",
    ],
    qhd: tw![
        "qhd:text-[2.5em]",
    ],
    uhd: tw![
        "uhd:text-[2.5em]",
    ],
}
