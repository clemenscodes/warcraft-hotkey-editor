use tw_macro::tw;
classes! {
    base: tw![
        "flex",
        "flex-col",
        "items-stretch",
        "px-0",
        "mt-0",
        "pt-0",
        "flex-none",
        "min-h-0",
        "gap-[clamp(0.95rem,1.6vh,1.5rem)]",
    ],
    mobile: tw![
        "mobile:grid",
        "mobile:grid-cols-[minmax(0,1fr)]",
        "mobile:gap-[1.5rem]",
        "mobile:items-start",
        "mobile:mt-[14px]",
    ],
    tablet: tw![
        "tablet:grid",
        "tablet:grid-cols-[minmax(0,1fr)]",
        "tablet:gap-[1.5rem]",
        "tablet:items-start",
        "tablet:mt-[14px]",
    ],
}
