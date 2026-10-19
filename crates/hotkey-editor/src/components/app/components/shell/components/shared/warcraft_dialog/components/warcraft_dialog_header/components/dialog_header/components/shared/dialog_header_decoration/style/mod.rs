use tw_macro::tw;
classes! {
    base: tw![
        "block",
        "h-[2.4rem]",
        "w-auto",
        "flex-none",
        "filter-[drop-shadow(0_1px_0_color-mix(in_oklab,var(--color-warcraft-shadow)_70%,transparent))]",
    ],
    // The ornament is a wide flourish. Base drives it by height and lets the
    // width follow. The narrow bands drive it by width instead, so they have to
    // hand the height back to the aspect ratio or the artwork is squashed.
    mobile: tw![
        "mobile:w-8",
        "mobile:h-auto",
    ],
    tablet: tw![
        "tablet:w-8",
        "tablet:h-auto",
    ],
    laptop: tw![
        "laptop:w-8",
        "laptop:h-auto",
    ],
    desktop: tw![
        "desktop:w-8",
        "desktop:h-auto",
    ],
    qhd: tw![
        "qhd:w-11.25",
        "qhd:h-auto",
    ],
    uhd: tw![
        "uhd:w-13.5",
        "uhd:h-auto",
    ],
}
