use tw_macro::tw;

classes! {
    base: tw![
        "flex",
        "flex-col",
        "items-center",
        "gap-3",
        "pt-2",
        "flex-[1_1_0]",
        "min-w-0",
        "self-stretch",
        "@container",
        "max-w-144.5",
    ],
    mobile: tw![
        "mobile:max-w-120.5",
    ],
    tablet: tw![
        "tablet:max-w-132.5",
    ],
    desktop: tw![
        "desktop:max-w-160.5",
    ],
    qhd: tw![
        "qhd:max-w-176.5",
    ],
    uhd: tw![
        "uhd:max-w-204.5",
    ],
}
