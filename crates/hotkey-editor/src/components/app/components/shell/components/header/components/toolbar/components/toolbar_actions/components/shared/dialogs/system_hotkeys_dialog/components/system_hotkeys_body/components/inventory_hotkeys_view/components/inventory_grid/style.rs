use tw_macro::tw;
classes! {
    base: tw![
        "grid",
        "grid-cols-[repeat(2,28rem)]",
        "auto-rows-[20rem]",
        "gap-6",
    ],
    mobile: tw![
        "mobile:grid-cols-[repeat(2,minmax(0,1fr))]",
        "mobile:auto-rows-[minmax(86px,auto)]",
        "mobile:gap-2.5",
        "mobile:w-full",
        "mobile:max-w-[26rem]",
        "mobile:mx-auto",
    ],
    tablet: tw![
        "tablet:grid-cols-[repeat(2,minmax(0,1fr))]",
        "tablet:auto-rows-[minmax(86px,auto)]",
        "tablet:gap-2.5",
        "tablet:w-full",
        "tablet:max-w-[26rem]",
        "tablet:mx-auto",
    ],
    laptop: tw![
        "laptop:grid-cols-[repeat(2,230px)]",
        "laptop:auto-rows-[165px]",
        "laptop:gap-3",
    ],
    desktop: tw![
        "desktop:grid-cols-[repeat(2,230px)]",
        "desktop:auto-rows-[165px]",
        "desktop:gap-3",
    ],
}
