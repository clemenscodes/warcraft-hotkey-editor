use tw_macro::tw;

classes! {
    base: tw![
        "@container",
        "flex",
        "shrink-0",
        "items-stretch",
        "gap-1",
        "px-4",
        "pt-2",
        "pb-2",
        "h-28",
        "text-2xs",
        "[--race-tab-align:flex-end]",
        "[--race-tab-label-inset:0.6em]",
        "[--race-tab-banner-size:auto_78%]",
        "[--race-tab-banner-position:center_top_0.4em]",
    ],
    tablet: tw![
        "tablet:h-36",
        "tablet:text-lg",
    ],
    laptop: tw![
        "laptop:h-48",
        "laptop:text-lg",
    ],
}
