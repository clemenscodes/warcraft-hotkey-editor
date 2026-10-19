use tw_macro::tw;
classes! {
    base: tw![
        "inline-flex",
        "[&_svg]:size-[1em]",
    ],
    mobile: tw![
        "mobile:flex-none",
        "mobile:text-[0.9em]",
        "mobile:leading-none",
        "mobile:transition-[transform]",
        "mobile:duration-slow",
    ],
    tablet: tw![
        "tablet:flex-none",
        "tablet:text-[0.9em]",
        "tablet:leading-none",
        "tablet:transition-[transform]",
        "tablet:duration-slow",
    ],
    laptop: tw![
        "laptop:flex-none",
        "laptop:text-[0.9em]",
        "laptop:leading-none",
        "laptop:transition-[transform]",
        "laptop:duration-slow",
    ],
    desktop: tw![
        "desktop:flex-none",
        "desktop:text-[0.9em]",
        "desktop:leading-none",
        "desktop:transition-[transform]",
        "desktop:duration-slow",
    ],
    qhd: tw![
        "qhd:flex-none",
        "qhd:text-[0.9em]",
        "qhd:leading-none",
        "qhd:transition-[transform]",
        "qhd:duration-slow",
    ],
    uhd: tw![
        "uhd:flex-none",
        "uhd:text-[0.9em]",
        "uhd:leading-none",
        "uhd:transition-[transform]",
        "uhd:duration-slow",
    ],
}
