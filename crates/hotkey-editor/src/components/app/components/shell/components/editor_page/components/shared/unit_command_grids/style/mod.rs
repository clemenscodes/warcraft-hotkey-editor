use tw_macro::tw;
classes! {
    base: tw![
        "@container",
        "grid",
        "grid-cols-2",
        "gap-4",
        "items-center",
        "flex-none",
        "isolate",
    ],
    mobile: tw![
        "mobile:flex",
        "mobile:flex-row",
        "mobile:flex-nowrap",
        "mobile:overflow-x-auto",
        "mobile:overflow-y-hidden",
        "mobile:overscroll-x-contain",
        "mobile:snap-x",
        "mobile:snap-mandatory",
        "mobile:scrollbar-none",
        "mobile:[&::-webkit-scrollbar]:hidden",
        "mobile:*:w-full",
        "mobile:*:shrink-0",
        "mobile:*:snap-start",
        "mobile:*:snap-always",
    ],
    tablet: tw![
        "tablet:grid-cols-[repeat(2,1fr)]",
    ],
}
