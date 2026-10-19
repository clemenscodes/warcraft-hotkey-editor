use tw_macro::tw;
classes! {
    base: tw![
        "@container",
        "grid",
        "grid-cols-[repeat(3,26rem)]",
        "gap-6",
    ],
    mobile: tw![
        "mobile:grid-cols-3",
        "mobile:gap-2",
        "mobile:w-full",
        "mobile:[--slot-key-size:26cqi]",
        "mobile:max-w-120",
    ],
    tablet: tw![
        "tablet:grid-cols-3",
        "tablet:gap-2",
        "tablet:w-full",
        "tablet:[--slot-key-size:26cqi]",
        "tablet:max-w-120",
    ],
    laptop: tw![
        "laptop:grid-cols-3",
        "laptop:gap-2",
        "laptop:w-full",
        "laptop:[--slot-key-size:26cqi]",
        "laptop:max-w-120",
    ],
    desktop: tw![
        "desktop:grid-cols-3",
        "desktop:gap-2",
        "desktop:w-full",
        "desktop:[--slot-key-size:26cqi]",
        "desktop:max-w-120",
    ],
    qhd: tw![
        "qhd:grid-cols-3",
        "qhd:gap-2.75",
        "qhd:w-full",
        "qhd:[--slot-key-size:26cqi]",
        "qhd:max-w-2xl",
    ],
    uhd: tw![
        "uhd:grid-cols-3",
        "uhd:gap-3.5",
        "uhd:w-full",
        "uhd:[--slot-key-size:26cqi]",
        "uhd:max-w-204",
    ],
}
