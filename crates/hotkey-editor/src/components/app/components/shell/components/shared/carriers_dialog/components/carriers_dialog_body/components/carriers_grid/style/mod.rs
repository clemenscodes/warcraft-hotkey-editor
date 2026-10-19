use tw_macro::tw;
classes! {
    base: tw![
        "grid",
        "grid-cols-[repeat(auto-fill,minmax(320px,1fr))]",
        "gap-6",
    ],
    mobile: tw![
        "mobile:gap-3",
    ],
    tablet: tw![
        "tablet:gap-3",
    ],
    laptop: tw![
        "laptop:gap-3",
    ],
}
