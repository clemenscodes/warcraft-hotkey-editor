use tw_macro::tw;

// From a desktop up the label sits at the foot of each tab under its banner,
// as it does in the race nav.
classes! {
    base: tw![
        "@container",
        "flex",
        "gap-4",
        "flex-nowrap",
        "w-full",
        "min-w-0",
        "grow",
        "self-stretch",
        "text-xl",
    ],
    mobile: tw![
        "mobile:h-40",
        "mobile:gap-1.5",
        "mobile:overflow-visible",
        "mobile:pt-0.5",
        "mobile:px-0",
        "mobile:pb-1.5",
    ],
    tablet: tw![
        "tablet:h-40",
        "tablet:gap-1.5",
        "tablet:overflow-visible",
        "tablet:pt-0.5",
        "tablet:px-0",
        "tablet:pb-1.5",
    ],
    laptop: tw![
        "laptop:h-40",
        "laptop:gap-1.5",
        "laptop:overflow-visible",
        "laptop:pt-0.5",
        "laptop:px-0",
        "laptop:pb-1.5",
    ],
    desktop: tw![
        "desktop:gap-1",
        "desktop:[--race-tab-align:flex-end]",
        "desktop:[--race-tab-label-inset:0.6em]",
        "desktop:[--race-tab-banner-size:auto_78%]",
        "desktop:[--race-tab-banner-position:center_top_0.4em]",
    ],
    qhd: tw![
        "qhd:gap-1",
        "qhd:[--race-tab-align:flex-end]",
        "qhd:[--race-tab-label-inset:0.6em]",
        "qhd:[--race-tab-banner-size:auto_78%]",
        "qhd:[--race-tab-banner-position:center_top_0.4em]",
    ],
    uhd: tw![
        "uhd:gap-1",
        "uhd:[--race-tab-align:flex-end]",
        "uhd:[--race-tab-label-inset:0.6em]",
        "uhd:[--race-tab-banner-size:auto_78%]",
        "uhd:[--race-tab-banner-position:center_top_0.4em]",
    ],
}
