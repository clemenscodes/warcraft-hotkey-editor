use tw_macro::tw;
classes! {
    base: tw![
        "@container",
        "grid",
    ],
    // In the mobile pager one move fills a full viewport height card, so it
    // grows to fill its snap slot rather than sitting centred with large gaps.
    mobile: tw![
        "mobile:flex-1",
        "mobile:min-h-0",
        "mobile:text-[4.47cqi]",
    ],
    tablet: tw![
        "tablet:text-[1.65cqi]",
    ],
    laptop: tw![
        "laptop:text-[1.65cqi]",
        "laptop:flex-1",
    ],
    desktop: tw![
        "desktop:text-[1.65cqi]",
        "desktop:flex-1",
    ],
    qhd: tw![
        "qhd:text-[1.65cqi]",
        "qhd:flex-1",
    ],
    uhd: tw![
        "uhd:text-[1.65cqi]",
        "uhd:flex-1",
    ],
}
