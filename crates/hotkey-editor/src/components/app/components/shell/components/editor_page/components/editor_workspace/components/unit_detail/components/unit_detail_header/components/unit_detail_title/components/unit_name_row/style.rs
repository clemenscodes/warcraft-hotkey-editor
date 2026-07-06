use tw_macro::tw;
classes! {
    base: tw![
        "flex",
        "items-center",
        "justify-between",
        "gap-[clamp(0.75rem,0.8vw,1rem)]",
        "min-h-0",
    ],
    mobile: tw![
        "mobile:flex-row",
        "mobile:items-center",
        "mobile:gap-[8px]",
        "mobile:w-full",
        "mobile:min-w-0",
    ],
    tablet: tw![
        "tablet:flex-row",
        "tablet:items-center",
        "tablet:gap-[8px]",
        "tablet:w-full",
        "tablet:min-w-0",
    ],
}
