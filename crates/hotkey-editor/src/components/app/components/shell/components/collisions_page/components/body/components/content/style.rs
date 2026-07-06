use tw_macro::tw;
// The two-pane layout (sidebar column + fluid detail), self-contained here rather

classes! {
    base: tw![
        "grid",
        "grid-cols-[34rem_minmax(0,1fr)]",
        "gap-10",
        "items-stretch",
        "mt-6",
        "flex-[1_1_0]",
        "min-h-0",
    ],
    mobile: tw![
        "mobile:grid-cols-[1fr]",
        "mobile:flex-none",
        "mobile:min-h-[auto]",
    ],
    tablet: tw!["tablet:grid-cols-[18rem_minmax(0,1fr)]"],
    qhd: tw!["qhd:grid-cols-[46rem_minmax(0,1fr)]"],
    uhd: tw!["uhd:grid-cols-[62rem_minmax(0,1fr)]"],
}
