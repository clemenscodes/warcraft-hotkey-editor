use tw_macro::tw;
// The editor's grid slot: it establishes the query container the grid and its
// tiles size against, and carries the responsive width (four tiles plus gaps).
// Everything inside sizes in `cqi` off this width, so the whole grid is one
// scalable shape — a mini grid is the same `Grid` in a smaller container.

classes! {
    base: tw![
        "flex",
        "flex-col",
        "items-center",
        "flex-[1_1_0]",
        "min-w-0",
        "self-stretch",
        "@container",
        "max-w-144.5",
    ],
    mobile: tw!["mobile:max-w-120.5"],
    tablet: tw!["tablet:max-w-132.5"],
    desktop: tw!["desktop:max-w-160.5"],
    qhd: tw!["qhd:max-w-176.5"],
    uhd: tw!["uhd:max-w-204.5"],
}
