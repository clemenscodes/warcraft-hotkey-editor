use tw_macro::tw;
classes! {
    base: tw![
        "flex",
        "flex-row",
        "flex-nowrap",
        "items-start",
        "gap-8",
    ],
    mobile: tw![
        "mobile:items-stretch",
        "mobile:gap-3",
        "mobile:overflow-x-auto",
        "mobile:overflow-y-hidden",
        "mobile:overscroll-x-contain",
        "mobile:touch-pan-x",
        "mobile:snap-x",
        "mobile:snap-mandatory",
        "mobile:scrollbar-none",
        "mobile:[&::-webkit-scrollbar]:hidden",
        "mobile:*:w-full",
        "mobile:*:shrink-0",
        "mobile:*:basis-full",
        "mobile:*:snap-start",
        "mobile:*:snap-always",
    ],
    tablet: tw![
        "tablet:gap-2",
    ],
}
