use tw_macro::tw;
// Phone/tablet: the five banners share one full-width row (no clumping, no
// horizontal scroll), with a tighter gap and a little breathing room below.

classes! {
    base: tw![
        "flex",
        "gap-4",
        "flex-nowrap",
        "w-full",
        "min-w-0",
        "grow",
        "self-stretch",
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
}
