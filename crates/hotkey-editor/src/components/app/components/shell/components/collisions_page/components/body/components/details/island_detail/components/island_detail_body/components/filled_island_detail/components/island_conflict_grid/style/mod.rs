use tw_macro::tw;

classes! {
    base: tw![
        "grid",
        "grid-cols-[repeat(auto-fill,minmax(450px,1fr))]",
        "gap-6",
        "flex-[1_1_0]",
        "min-h-0",
        "overflow-y-auto",
        "content-start",
        "pt-4",
        "pr-3",
        "pb-4",
        "pl-0",
    ],
    mobile: tw![
        "mobile:grid-cols-[minmax(0,1fr)]",
        "mobile:flex-none",
        "mobile:min-h-auto",
        "mobile:overflow-y-visible",
        "mobile:py-4",
        "mobile:px-0",
    ],
    tablet: tw![
        "tablet:grid-cols-[minmax(0,1fr)]",
        "tablet:flex-none",
        "tablet:min-h-auto",
        "tablet:overflow-y-visible",
        "tablet:py-4",
        "tablet:px-0",
    ],
}
