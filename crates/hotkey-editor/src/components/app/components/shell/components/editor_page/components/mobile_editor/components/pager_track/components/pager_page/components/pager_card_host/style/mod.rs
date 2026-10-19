use tw_macro::tw;
// Each card fills the width it is given; its height follows from that width,
// and a card taller than the pager is reached by scrolling.
classes! {
    base: tw![
        "@container",
        "flex",
        "flex-col",
        "min-w-0",
        "py-1.5",
    ],
    mobile: tw![
        "mobile:w-full",
    ],
    laptop: tw![
        "laptop:w-full",
    ],
}
