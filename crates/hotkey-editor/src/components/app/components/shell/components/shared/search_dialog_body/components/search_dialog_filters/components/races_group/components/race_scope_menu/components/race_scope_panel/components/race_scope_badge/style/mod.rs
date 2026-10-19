use tw_macro::tw;

// The label's type is a share of the badge's width. A phone's badges are too
// narrow for that and keep the smallest type step.
classes! {
    base: tw![
        "@container",
        "flex",
        "items-stretch",
        "flex-1",
        "min-w-0",
        "h-24",
        "*:text-[9cqi]",
    ],
    mobile: tw![
        "mobile:*:text-2xs",
    ],
    desktop: tw![
        "desktop:h-29",
    ],
    qhd: tw![
        "qhd:h-41.5",
    ],
    uhd: tw![
        "uhd:h-45",
    ],
}
