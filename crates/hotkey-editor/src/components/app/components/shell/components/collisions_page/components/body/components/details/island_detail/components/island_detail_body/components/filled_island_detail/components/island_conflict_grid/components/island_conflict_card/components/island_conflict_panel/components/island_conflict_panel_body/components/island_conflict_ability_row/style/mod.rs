use tw_macro::tw;

// Both ability columns get the same fixed share of the row, so the icon each
// draws, 64.3% of its column, has a height the separator can match exactly.
// The row is as wide as a pair row's two 31cqi ability columns plus this
// separator and its gaps, so a cross collision's icons are the same size as
// those of an intra or hotkey collision.
classes! {
    base: tw![
        "grid",
        "grid-cols-[1fr_auto_1fr]",
        "items-start",
        "justify-items-center",
        "gap-[2.88cqi]",
        "w-full",
    ],
    mobile: tw![
        "mobile:grid-cols-[minmax(0,1fr)_8cqi_minmax(0,1fr)]",
        "mobile:justify-items-stretch",
        "mobile:w-[75.76cqi]",
    ],
    tablet: tw![
        "tablet:grid-cols-[minmax(0,1fr)_8cqi_minmax(0,1fr)]",
        "tablet:justify-items-stretch",
        "tablet:w-[75.76cqi]",
    ],
    laptop: tw![
        "laptop:grid-cols-[minmax(0,1fr)_8cqi_minmax(0,1fr)]",
        "laptop:justify-items-stretch",
        "laptop:w-[75.76cqi]",
    ],
}
