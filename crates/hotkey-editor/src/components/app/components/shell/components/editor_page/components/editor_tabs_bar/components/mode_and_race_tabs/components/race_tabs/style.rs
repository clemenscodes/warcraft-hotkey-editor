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
        "mobile:gap-[0.4rem]",
        "mobile:overflow-visible",
        "mobile:p-[0.15rem_0_0.4rem]",
    ],
    tablet: tw![
        "tablet:gap-[0.4rem]",
        "tablet:overflow-visible",
        "tablet:p-[0.15rem_0_0.4rem]",
    ],
}
