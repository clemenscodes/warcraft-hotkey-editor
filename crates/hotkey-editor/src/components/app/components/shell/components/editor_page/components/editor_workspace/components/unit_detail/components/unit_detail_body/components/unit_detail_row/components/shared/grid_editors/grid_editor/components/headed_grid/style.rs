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
        "[container-type:inline-size]",
        "max-w-[578px]",
    ],
    mobile: tw!["mobile:max-w-[482px]"],
    tablet: tw!["tablet:max-w-[530px]"],
    desktop: tw!["desktop:max-w-[642px]"],
    qhd: tw!["qhd:max-w-[706px]"],
    uhd: tw!["uhd:max-w-[818px]"],
}
